use anyhow::{anyhow, Result};
use ory_hydra_client::{apis::admin_api::create_o_auth2_client, models::OAuth2Client};
use tokio::try_join;
use zagreus_domain::models::client::Client;

use crate::hydra_configuration::CONFIGURATION;

pub async fn init(client_name: &str) -> Result<()> {
    let pool = zagreus_domain::db::connect().await?;

    try_join!(
        async move {
            create_o_auth2_client(
                &CONFIGURATION,
                OAuth2Client {
                    audience: Some(vec![zagreus_config::env::ACCESS_TOKEN_AUDIENCE()]),
                    grant_types: Some(zagreus_config::hydra_grant_types()),
                    client_id: Some(client_name.to_string()),
                    client_name: Some(client_name.to_string()),
                    client_secret: Some(zagreus_config::env::CLIENT::SECRET()),
                    redirect_uris: Some(vec![zagreus_config::env::REDIRECT_URL()]),
                    response_types: Some(zagreus_config::hydra_response_types()),
                    scope: Some(zagreus_config::hydra_scopes().join(" ")),
                    ..OAuth2Client::new()
                },
            )
            .await
            .map_err(|_| anyhow!("Couldn't create client in Hydra database"))
        },
        Client::create(&pool, client_name)
    )?;

    Ok(())
}
