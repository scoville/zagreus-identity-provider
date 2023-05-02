/// This crate exposes the global env(ironment) and config(uration) modules
/// that can be used by the binaries.
/// The sub module `env` is generated from the `.env` file located at the root of
/// the IDP project _not Zagreus_.
use anyhow::Result;
use itconfig::config;

pub static CORS_ALLOWED_METHODS: &[&str] = &["GET", "POST", "PUT"];

// 1. In macros, the :: prefix before the module name tells Rust
// that we're referring to the top level std module, not some potentially inner std module
// 2. https://docs.rs/once_cell/latest/once_cell/sync/struct.Lazy.html is much better, we can initialize our vec once
// and make it available for the whole app. I would question the need for the vec string
// We can probably just use a [&' static; n] and iter().map(to_owned).collect::<Vec<_>>() when needed
macro_rules! define_ownable_strings_static {
    (static $global:ident, fn $name:ident, $values:expr) => {
        pub static $global: &[&str] = $values;

        pub fn $name() -> Vec<String> {
            $global
                .iter()
                .map(::std::string::ToString::to_string)
                .collect()
        }
    };
}

define_ownable_strings_static!(
    static HYDRA_SCOPES,
    fn hydra_scopes,
    &["openid", "offline_access", "email", "profile"]
);

define_ownable_strings_static!(
    static HYDRA_GRANT_TYPES,
    fn hydra_grant_types,
    &["authorization_code", "refresh_token"]
);

define_ownable_strings_static!(
    static HYDRA_RESPONSE_TYPES,
    fn hydra_response_types,
    &["token", "code", "id_token"]
);

config! {
    #![config(name="env")]

    #[allow(non_snake_case)]
    PORT: u32,
    #[allow(non_snake_case)]
    URL: &'static str,
    #[allow(non_snake_case)]
    DATABASE {
        URL: &'static str
    },
    #[allow(non_snake_case)]
    ACCESS_TOKEN_AUDIENCE: String,
    #[allow(non_snake_case)]
    CLIENT {
        ID: String,
        SECRET: String,
    },
    #[allow(non_snake_case)]
    REDIRECT_URL: String,
    #[allow(non_snake_case)]
    HYDRA {
        ADMIN_API_URL: String,
        PUBLIC_API_URL: String,
        PUBLIC_AUTH_URL < ( HYDRA_PUBLIC_API_URL, "/oauth2/auth" ),
        PUBLIC_TOKEN_URL < ( HYDRA_PUBLIC_API_URL, "/oauth2/token" ),
    },
    #[allow(non_snake_case)]
    STATIC_PATH: &'static str,
    #[allow(non_snake_case)]
    TEMPLATES_PATH: &'static str,
}

pub fn init() -> Result<()> {
    dotenv::dotenv()?;

    env::init();

    Ok(())
}
