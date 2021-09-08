use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::db::PgPool;

#[derive(Debug)]
pub struct Invitation {
    pub id: Uuid,
    pub email: String,
    pub code: String,
    pub redirect_uri: String,
    pub idp_client_id: String,
    pub used_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Invitation {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Invitation>> {
        let invitations = query_as!(
            Invitation,
            "
                SELECT id, email, code, redirect_uri, idp_client_id, used_at, created_at, updated_at
                FROM invitations
            ",
        )
        .fetch_all(pool)
        .await?;

        Ok(invitations)
    }

    pub async fn get_by_code(pool: &PgPool, code: &str) -> Result<Option<Invitation>> {
        let invitation = query_as!(
            Invitation,
            "
                SELECT id, email, code, redirect_uri, idp_client_id, used_at, created_at, updated_at
                FROM invitations
                WHERE code = $1
            ",
            code
        )
        .fetch_optional(pool)
        .await?;

        Ok(invitation)
    }

    pub async fn create(
        pool: &PgPool,
        email: &str,
        code: &str,
        idp_client_id: &str,
        redirect_uri: &str,
    ) -> Result<Uuid> {
        let invitation = query!(
            "
                INSERT INTO invitations(email, code, idp_client_id, redirect_uri)
                VALUES ($1, $2, $3, $4)
                RETURNING id
            ",
            email,
            code,
            idp_client_id,
            redirect_uri
        )
        .fetch_one(pool)
        .await?;

        Ok(invitation.id)
    }

    pub async fn update_used_at(
        pool: &PgPool,
        code: &str,
        used_at: &NaiveDateTime,
    ) -> Result<Option<Uuid>> {
        let invitation = query!(
            "
                UPDATE invitations SET used_at = $1
                WHERE code = $2
                RETURNING id
            ",
            used_at,
            code,
        )
        .fetch_optional(pool)
        .await?;

        Ok(invitation.map(|invitation| invitation.id))
    }
}
