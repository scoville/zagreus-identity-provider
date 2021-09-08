use oauth2::{basic::BasicClient, AuthUrl, ClientId, RedirectUrl, TokenUrl};
use ory_hydra_client::apis::configuration::Configuration;

lazy_static! {
    pub static ref CONFIGURATION: Configuration = Configuration {
        base_path: zagreus_config::env::HYDRA::ADMIN_API_URL(),
        ..Configuration::default()
    };
    pub static ref CLIENT: BasicClient = BasicClient::new(
        ClientId::new(zagreus_config::env::CLIENT::ID()),
        None,
        AuthUrl::new(zagreus_config::env::HYDRA::PUBLIC_AUTH_URL()).expect("wrong auth url format"),
        Some(
            TokenUrl::new(zagreus_config::env::HYDRA::PUBLIC_TOKEN_URL())
                .expect("wrong token url format")
        )
    )
    .set_redirect_uri(
        RedirectUrl::new(zagreus_config::env::REDIRECT_URL()).expect("wrong redirect url")
    );
}
