use actix_web::{post, web, HttpResponse, ResponseError, Result};
use ory_hydra_client::{
    apis::admin_api::{accept_consent_request, get_consent_request},
    models::{AcceptConsentRequest, ConsentRequestSession},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;
use zagreus_domain::{db::PgPool, models::user::User};

use crate::hydra_configuration::CONFIGURATION;
use crate::validations::validate;

#[derive(Error, Debug)]
pub enum ConsentError {
    #[error("consent couldn't be found")]
    ConsentNotFound,
    #[error("consent subject couldn't be found")]
    SubjectNotFound,
    #[error("consent subject is not a valid uuid")]
    WrongSubject,
    #[error("user not found")]
    UserNotFound,
    #[error("user request error")]
    UserError,
    #[error("couldn't accept consent")]
    CouldntAcceptConsent,
}

impl ResponseError for ConsentError {}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConsentPayload {
    #[validate(length(min = 1))]
    code: String,
}

#[derive(Debug, Serialize)]
struct IdToken {
    email: String,
}

#[derive(Debug, Serialize)]
struct ConsentResponse {
    redirect_to: String,
}

// TODO: INACTIVE - Not used by the application, can probably be removed.
/// Takes a code that can be resolved to a consent challenge and consent to the terms.
/// _You can use the get version for a faster consent policy._
#[allow(dead_code)]
#[post("/api/consent")]
pub async fn consent(
    payload: web::Json<ConsentPayload>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    validate!(payload);

    let get = get_consent_request(&CONFIGURATION, payload.code.as_str())
        .await
        .map_err(|_| ConsentError::ConsentNotFound)?;

    let subject = get.subject.ok_or(ConsentError::SubjectNotFound)?;

    let uuid = Uuid::parse_str(subject.as_str()).map_err(|_| ConsentError::WrongSubject)?;

    let user = User::get_by_id(&pool, &uuid)
        .await
        .map_err(|_| ConsentError::UserError)?;

    let user = user.ok_or(ConsentError::UserNotFound)?;

    let id_token = IdToken { email: user.email };

    let id_token = serde_json::to_value(&id_token)?;

    let accept = accept_consent_request(
        &CONFIGURATION,
        payload.code.as_str(),
        Some(AcceptConsentRequest {
            grant_scope: Some(zagreus_config::hydra_scopes()),
            remember: Some(true),
            remember_for: Some(0),
            session: Some(Box::new(ConsentRequestSession {
                id_token: Some(id_token),
                ..ConsentRequestSession::new()
            })),
            ..AcceptConsentRequest::new()
        }),
    )
    .await
    .map_err(|_| ConsentError::CouldntAcceptConsent)?;

    Ok(HttpResponse::Ok().json(ConsentResponse {
        redirect_to: accept.redirect_to,
    }))
}
