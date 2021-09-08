use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::db::PgPool;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub encrypted_password: String,
    pub terms_accepted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub async fn get_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<User>> {
        let user = query_as!(
            User,
            "
                SELECT id, email, encrypted_password, terms_accepted_at, created_at, updated_at
                FROM users
                WHERE id = $1
            ",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
        let user = query_as!(
            User,
            "
                SELECT id, email, encrypted_password, terms_accepted_at, created_at, updated_at
                FROM users
                WHERE email = $1
            ",
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn create(
        pool: &PgPool,
        email: &str,
        encrypted_password: &str,
        terms_accepted_at: &NaiveDateTime,
    ) -> Result<Uuid> {
        let invitation = query!(
            "
                INSERT INTO users(email, encrypted_password, terms_accepted_at)
                VALUES ($1, $2, $3)
                RETURNING id
            ",
            email,
            encrypted_password,
            terms_accepted_at
        )
        .fetch_one(pool)
        .await?;

        Ok(invitation.id)
    }
}
