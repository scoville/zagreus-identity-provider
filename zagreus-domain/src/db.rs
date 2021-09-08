use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type PgPool = Pool<Postgres>;

pub async fn connect() -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(zagreus_config::env::DATABASE::URL())
        .await?;

    Ok(pool)
}
