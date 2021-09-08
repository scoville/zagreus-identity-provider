use actix_web::{
    get, http::header, web, HttpRequest, HttpResponse, Responder, ResponseError, Result,
};
use oauth2::CsrfToken;
use ory_hydra_client::{
    apis::admin_api::{accept_login_request, get_login_request},
    models::AcceptLoginRequest,
};
use rand::{distributions, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;
use validator::Validate;

use super::HtmlTemplate;
use crate::hydra_configuration::{CLIENT, CONFIGURATION};
use crate::validations::validate;

#[derive(Debug, Serialize)]
struct LoginTemplate {
    login_challenge: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
enum LoginError {
    #[error("wrong login_challenge argument")]
    WrongChallenge,
    #[error("login request returned a wrong request_url")]
    WrongRequestUrl,
    #[error("login request returned a wrong redirect_uri")]
    WrongRedirectUri,
}

impl ResponseError for LoginError {}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(length(min = 1))]
    login_challenge: Option<String>,
}

#[get("/login")]
pub async fn login(req: HttpRequest, payload: web::Query<LoginPayload>) -> Result<HttpResponse> {
    validate!(payload);

    let login_challenge = match payload.into_inner().login_challenge {
        Some(login_challenge) => login_challenge,
        None => {
            let state: String = thread_rng()
                .sample_iter(distributions::Alphanumeric)
                .take(12)
                .map(char::from)
                .collect();

            let (redirect_to, _) = CLIENT.authorize_url(|| CsrfToken::new(state)).url();

            return Ok(HttpResponse::PermanentRedirect()
                .append_header((header::LOCATION, redirect_to.to_string()))
                .finish());
        }
    };

    let login_request = get_login_request(&CONFIGURATION, login_challenge.as_ref())
        .await
        .map_err(|_| LoginError::WrongChallenge)?;

    let request_url =
        Url::parse(login_request.request_url.as_str()).map_err(|_| LoginError::WrongRequestUrl)?;

    let redirect_uri = request_url
        .query_pairs()
        .find(|(key, _)| key == "redirect_uri");

    let (_, redirect_uri) = redirect_uri.ok_or(LoginError::WrongRedirectUri)?;

    if login_request.skip {
        let completed_request = accept_login_request(
            &CONFIGURATION,
            login_challenge.as_ref(),
            Some(AcceptLoginRequest {
                remember: Some(true),
                ..AcceptLoginRequest::new(login_request.subject)
            }),
        )
        .await;

        let redirect_to = match completed_request {
            Ok(completed_request) => completed_request.redirect_to,
            Err(_) => redirect_uri.into_owned(),
        };

        return Ok(HttpResponse::PermanentRedirect()
            .append_header((header::LOCATION, redirect_to))
            .finish());
    }

    Ok(HtmlTemplate::new("login.html", LoginTemplate { login_challenge }).respond_to(&req))
}
