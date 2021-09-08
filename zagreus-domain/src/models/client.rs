use anyhow::Result;
use sqlx::query;

use crate::db::PgPool;

#[derive(Debug)]
pub struct Client;

impl Client {
    pub async fn create(pool: &PgPool, client_name: &str) -> Result<()> {
        query(
            "
                    INSERT INTO idp_clients (id, name, redirect_uris)
                    VALUES ($1, $1, '{/}');
                ",
        )
        .bind(client_name)
        .execute(pool)
        .await?;

        Ok(())
    }
}
