use actix_web::{post, web, HttpResponse, ResponseError, Result};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use ory_hydra_client::apis::admin_api::accept_login_request;
use ory_hydra_client::models::AcceptLoginRequest;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;
use zagreus_domain::{db::PgPool, models::user::User};

use crate::hydra_configuration::CONFIGURATION;
use crate::validations::{validate, validate_password};

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("user not found")]
    UserNotFound,
    #[error("persisted encrypted password couldn't be hashed")]
    PersistedPasswordInvalidFormat,
    #[error("invalid password")]
    InvalidPassword,
    #[error("login request rejected")]
    LoginRequestRejected,
}

impl ResponseError for LoginError {}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginPayload {
    #[validate(length(min = 1))]
    login_challenge: String,
    #[validate(email)]
    email: String,
    #[validate(custom = "validate_password")]
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginResponse {
    redirect_to: String,
}

/// Take credentials and try to authenticate the user.
#[post("/api/login")]
pub async fn login(
    payload: web::Json<LoginPayload>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    validate!(payload);

    let user = User::get_by_email(&pool, payload.email.as_str())
        .await
        .map_err(|_| LoginError::UserNotFound)?;

    let user = user.ok_or(LoginError::UserNotFound)?;

    let password_hash = PasswordHash::new(user.encrypted_password.as_str())
        .map_err(|_| LoginError::PersistedPasswordInvalidFormat)?;

    let argon2 = Argon2::default();

    argon2
        .verify_password(payload.password.as_bytes(), &password_hash)
        .map_err(|_| LoginError::InvalidPassword)?;

    let completed_request = accept_login_request(
        &CONFIGURATION,
        payload.login_challenge.as_str(),
        Some(AcceptLoginRequest {
            remember: Some(true),
            ..AcceptLoginRequest::new(user.id.to_string())
        }),
    )
    .await
    .map_err(|_| LoginError::LoginRequestRejected)?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        redirect_to: completed_request.redirect_to,
    }))
}
