use actix_web::{get, http::header, web, HttpResponse, ResponseError, Result};
use ory_hydra_client::{
    apis::admin_api::{accept_consent_request, get_consent_request},
    models::{AcceptConsentRequest, ConsentRequestSession},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;
use uuid::Uuid;
use validator::Validate;
use zagreus_domain::{db::PgPool, models::user::User};

use crate::hydra_configuration::CONFIGURATION;
use crate::validations::validate;

#[derive(Error, Debug)]
pub enum ConsentError {
    #[error("consent subject is not a valid uuid")]
    WrongSubject,
    #[error("user not found")]
    UserNotFound,
    #[error("consent request failed")]
    ConsentRequestFailed,
    #[error("consent request returned a wrong request_url")]
    WrongRequestUrl,
    #[error("consent request returned no subject")]
    NoSubject,
    #[error("consent request returned a wrong redirect_uri")]
    WrongRedirectUri,
}

impl ResponseError for ConsentError {}

#[derive(Debug, Serialize)]
struct IdToken {
    email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct FastConsentPayload {
    #[validate(length(min = 1))]
    consent_challenge: String,
}

/// Takes a consent challenge and assumes the user already consented to the terms and service.
/// _Must be used only if the complete invitation form contains a consent/accept terms checkbox._
#[get("/api/public/consent")]
pub async fn public_consent(
    payload: web::Query<FastConsentPayload>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    validate!(payload);

    let consent_request = get_consent_request(&CONFIGURATION, payload.consent_challenge.as_str())
        .await
        .map_err(|_| ConsentError::ConsentRequestFailed)?;

    let subject = consent_request.subject.ok_or(ConsentError::NoSubject)?;

    let user_id = Uuid::parse_str(subject.as_str()).map_err(|_| ConsentError::WrongSubject)?;

    let request_url = consent_request
        .request_url
        .ok_or(ConsentError::WrongRequestUrl)?;

    let request_url =
        Url::parse(request_url.as_str()).map_err(|_| ConsentError::WrongRequestUrl)?;

    let redirect_uri_param = request_url
        .query_pairs()
        .find(|(key, _)| key == "redirect_uri");

    let (_, redirect_uri) = redirect_uri_param.ok_or(ConsentError::WrongRedirectUri)?;

    let user = User::get_by_id(&pool, &user_id)
        .await
        .map_err(|_| ConsentError::UserNotFound)?;

    let user = user.ok_or(ConsentError::UserNotFound)?;

    let id_token = IdToken { email: user.email };

    let id_token = serde_json::to_value(&id_token)?;

    let consent_request = accept_consent_request(
        &CONFIGURATION,
        payload.consent_challenge.as_str(),
        Some(AcceptConsentRequest {
            grant_access_token_audience: Some(vec![zagreus_config::env::ACCESS_TOKEN_AUDIENCE()]),
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
    .await;

    let redirect_to = match consent_request {
        Ok(consent_request) => consent_request.redirect_to,
        Err(_) => redirect_uri.into_owned(),
    };

    Ok(HttpResponse::PermanentRedirect()
        .append_header((header::LOCATION, redirect_to))
        .finish())
}
