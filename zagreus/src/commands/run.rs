use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Result;

use crate::api;
use crate::views;

pub async fn run() -> Result<()> {
    let pool = Data::new(zagreus_domain::db::connect().await?);

    HttpServer::new(move || {
        let logger = Logger::default();

        let cors = Cors::default()
            .allowed_origin(zagreus_config::env::URL())
            .allowed_methods(zagreus_config::CORS_ALLOWED_METHODS.to_vec())
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(logger)
            .app_data(pool.clone())
            // Public endpoints used by Hydra mostly
            .service(api::public::consent::public_consent)
            // Private endpoints used internally by the webapp
            // .service(api::consent::consent)
            // .service(api::invitation::get_complete_invitation)
            .service(api::invitation::create_invitation)
            .service(api::invitation::complete_invitation)
            .service(api::login::login)
            .service(api::logout::logout)
            // Views for the webapp
            .service(views::home::home)
            .service(views::invitation::invitation)
            .service(views::invitations::invitations)
            .service(views::login::login)
            // Static files
            .service(Files::new("/", zagreus_config::env::STATIC_PATH()))
    })
    .bind(format!("0.0.0.0:{}", zagreus_config::env::PORT()))?
    .run()
    .await?;

    Ok(())
}
