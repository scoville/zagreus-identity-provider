use actix_web::{get, http::header, web, HttpResponse, ResponseError, Result};
use ory_hydra_client::apis::admin_api::{accept_logout_request, get_logout_request};
use serde::Deserialize;
use thiserror::Error;
use validator::Validate;

use crate::hydra_configuration::CONFIGURATION;
use crate::validations::validate;

#[derive(Debug, Error)]
pub enum LogoutError {
    #[error("logout request rejected")]
    LogoutRequestRejected,
}

impl ResponseError for LogoutError {}

#[derive(Debug, Deserialize, Validate)]
pub struct LogoutPayload {
    #[validate(length(min = 1))]
    logout_challenge: String,
}

/// Log a user out.
#[get("/api/logout")]
pub async fn logout(payload: web::Query<LogoutPayload>) -> Result<HttpResponse> {
    validate!(payload);

    get_logout_request(&CONFIGURATION, payload.logout_challenge.as_str())
        .await
        .map_err(|_| LogoutError::LogoutRequestRejected)?;

    let completed_request =
        accept_logout_request(&CONFIGURATION, payload.logout_challenge.as_str())
            .await
            .map_err(|_| LogoutError::LogoutRequestRejected)?;

    Ok(HttpResponse::PermanentRedirect()
        .append_header((header::LOCATION, completed_request.redirect_to))
        .finish())
}
