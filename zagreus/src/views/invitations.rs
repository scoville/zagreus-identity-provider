use actix_web::{get, web, Responder, ResponseError, Result};
use serde::Serialize;
use thiserror::Error;
use zagreus_domain::{db::PgPool, models::invitation::Invitation};

use super::HtmlTemplate;

#[derive(Debug, Serialize)]
struct RenderedInvitation {
    email: String,
    path: String,
}

impl From<Invitation> for RenderedInvitation {
    fn from(invitation: Invitation) -> Self {
        let path = format!("/invitations/{challenge}", challenge = invitation.code);

        Self {
            email: invitation.email,
            path,
        }
    }
}

#[derive(Debug, Serialize)]
struct InvitationsTemplate {
    invitations: Vec<RenderedInvitation>,
}

#[derive(Debug, Error)]
enum InvitationsError {
    #[error("invitations couldn't be found")]
    NotFound,
}

impl ResponseError for InvitationsError {}

#[get("/invitations")]
pub async fn invitations(pool: web::Data<PgPool>) -> Result<impl Responder> {
    let invitations = Invitation::get_all(&pool)
        .await
        .map_err(|_| InvitationsError::NotFound)?
        .into_iter()
        .map(RenderedInvitation::from)
        .collect();

    Ok(HtmlTemplate::new(
        "invitations.html",
        InvitationsTemplate { invitations },
    ))
}
