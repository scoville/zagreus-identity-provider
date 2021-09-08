#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use clap::{crate_version, Clap};
use log::info;

mod api;
mod commands;
mod hydra_configuration;
mod validations;
mod views;

#[derive(Debug, Clap)]
#[clap(version = crate_version!())]
struct Options {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Clap)]
enum Command {
    Init {
        /// The client (IDP) name
        #[clap(short, long)]
        client_name: String,
    },
    Run,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let options = Options::parse();

    zagreus_config::init()?;

    env_logger::init();

    match options.command {
        Command::Init { client_name } => {
            commands::init(client_name.as_str()).await?;

            info!("Zagreus has been successfully initiailized");
        }
        Command::Run => commands::run().await?,
    };

    Ok(())
}
