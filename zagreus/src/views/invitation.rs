use actix_web::{get, web, HttpRequest, Responder, ResponseError, Result};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;
use zagreus_domain::{db::PgPool, models::invitation::Invitation};

use super::HtmlTemplate;
use crate::validations::validate;

#[derive(Debug, Serialize)]
struct InvitationTemplate {
    email: String,
    invitation_challenge: String,
}

#[derive(Debug, Error)]
enum InvitationError {
    #[error("invitation couldn't be found")]
    NotFound,
    #[error("invitation already used")]
    AlreadyUsed,
}

impl ResponseError for InvitationError {}

#[derive(Debug, Deserialize, Validate)]
pub struct InvitationPayload {
    #[validate(length(min = 1))]
    challenge: String,
}

#[get("/invitations/{challenge}")]
pub async fn invitation(
    req: HttpRequest,
    payload: web::Path<InvitationPayload>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    validate!(payload);

    let invitation = Invitation::get_by_code(&pool, payload.challenge.as_str())
        .await
        .map_err(|_| InvitationError::NotFound)?;

    let invitation = invitation.ok_or(InvitationError::NotFound)?;

    if invitation.used_at.is_some() {
        return Err(InvitationError::AlreadyUsed.into());
    }

    Ok(HtmlTemplate::new(
        "invitation.html",
        InvitationTemplate {
            email: invitation.email,
            invitation_challenge: payload.into_inner().challenge,
        },
    )
    .respond_to(&req))
}
