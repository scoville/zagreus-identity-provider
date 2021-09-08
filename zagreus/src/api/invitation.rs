use actix_web::{get, post, put, web, HttpResponse, ResponseError, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::Utc;
use rand::{distributions, thread_rng, Rng};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use url::Url;
use validator::Validate;
use zagreus_domain::{
    db::PgPool,
    models::{invitation::Invitation, user::User},
};

use crate::validations::{validate, validate_password, validate_terms_accepted};

#[derive(Error, Debug)]
pub enum InvitationError {
    #[error("invitation couldn't be found")]
    InvitationNotFound,
    #[error("user request error")]
    UserError,
    #[error("user couldn't be found")]
    UserNotFound,
    #[error("email already exists")]
    EmailAlreadyExists,
    #[error("invitation couldn't be created")]
    InvitationNotCreated,
    #[error("invitation couldn't be updated")]
    InvitationNotUpdated,
    #[error("invitation has already been used")]
    InvitationAlreadyUsed,
    #[error("password encryption failed")]
    PasswordEncryptionFailed,
    #[error("user couldn't be created")]
    UserNotCreated,
    #[error("invalid redirect to url")]
    InvalidRedirectToUrl,
}

impl ResponseError for InvitationError {}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCompleteInvitationPayload {
    #[validate(length(min = 1))]
    invitation_challenge: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetCompleteInvitationResponse {
    email: String,
    user_id: String,
}

// TODO: INACTIVE - Not used by the application, can probably be removed.
/// Retrieves a completed invitation.
#[allow(dead_code)]
#[get("/api/invitation")]
pub async fn get_complete_invitation(
    payload: web::Query<GetCompleteInvitationPayload>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    validate!(payload);

    let invitation = Invitation::get_by_code(&pool, payload.invitation_challenge.as_str())
        .await
        .map_err(|_| InvitationError::InvitationNotFound)?;

    let invitation = invitation.ok_or(InvitationError::InvitationNotFound)?;

    let user = User::get_by_email(&pool, &invitation.email)
        .await
        .map_err(|_| InvitationError::UserNotFound)?;

    let user = user.ok_or(InvitationError::UserNotFound)?;

    Ok(HttpResponse::Ok().json(GetCompleteInvitationResponse {
        email: user.email,
        user_id: user.id.to_string(),
    }))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvitationPayload {
    #[validate(length(min = 1))]
    client_id: String,
    #[validate(email)]
    email: String,
    #[validate(url)]
    redirect_uri: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateInvitationResponse {
    code: String,
    redirect_to: String,
}

/// Creates an invitation from an email.
#[post("/api/invitation")]
pub async fn create_invitation(
    payload: web::Json<CreateInvitationPayload>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    validate!(payload);

    let code: String = thread_rng()
        .sample_iter(distributions::Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    let user = User::get_by_email(&pool, payload.email.as_str())
        .await
        .map_err(|_| InvitationError::UserError)?;

    if user.is_some() {
        return Err(InvitationError::EmailAlreadyExists.into());
    }

    Invitation::create(
        &pool,
        payload.email.as_str(),
        code.as_str(),
        &payload.client_id,
        payload.redirect_uri.as_str(),
    )
    .await
    .map_err(|_| InvitationError::InvitationNotCreated)?;

    Ok(HttpResponse::Ok().json(CreateInvitationResponse {
        code,
        redirect_to: String::from("/"),
    }))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CompleteInvitationPayload {
    #[validate(length(min = 1))]
    invitation_challenge: String,
    #[validate(custom = "validate_password")]
    password: String,
    extra_payload: Option<HashMap<String, String>>,
    #[validate(custom = "validate_terms_accepted")]
    terms_accepted: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CompleteInvitationResponse {
    redirect_to: String,
}

/// Completes an invitation, that is, register a user.
/// Additionally to the required password and terms values a generic "payload" attribute
/// that will be serialized and injected into the redirect url _use with care and don't send
/// sensitive data in the `payload` attribute_.
#[put("/api/invitation")]
pub async fn complete_invitation(
    payload: web::Json<CompleteInvitationPayload>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    validate!(payload);

    let invitation = Invitation::get_by_code(&pool, payload.invitation_challenge.as_str())
        .await
        .map_err(|_| InvitationError::InvitationNotFound)?;

    let invitation = invitation.ok_or(InvitationError::InvitationNotFound)?;

    if invitation.used_at.is_some() {
        return Err(InvitationError::InvitationAlreadyUsed.into());
    }

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let encrypted_password = argon2
        .hash_password_simple(payload.password.as_bytes(), salt.as_ref())
        .map_err(|_| InvitationError::PasswordEncryptionFailed)?
        .to_string();

    let terms_accepted_at = Utc::now().naive_utc();

    let new_user_id = User::create(
        &pool,
        invitation.email.as_str(),
        encrypted_password.as_str(),
        &terms_accepted_at,
    )
    .await
    .map_err(|_| InvitationError::UserNotCreated)?;

    let invitation_id = Invitation::update_used_at(
        &pool,
        payload.invitation_challenge.as_str(),
        &terms_accepted_at,
    )
    .await
    .map_err(|_| InvitationError::InvitationNotUpdated)?;

    invitation_id.ok_or(InvitationError::InvitationNotFound)?;

    let mut redirect_to = Url::parse(invitation.redirect_uri.as_str())
        .map_err(|_| InvitationError::InvalidRedirectToUrl)?;

    redirect_to
        .query_pairs_mut()
        .append_pair("user_id", new_user_id.to_string().as_str())
        .append_pair("email", invitation.email.as_str());

    if let Some(ref extra_payload) = payload.extra_payload {
        for (key, value) in extra_payload {
            redirect_to
                .query_pairs_mut()
                .append_pair(key.as_str(), value.as_str());
        }
    }

    Ok(HttpResponse::Ok().json(CompleteInvitationResponse {
        redirect_to: redirect_to.to_string(),
    }))
}
