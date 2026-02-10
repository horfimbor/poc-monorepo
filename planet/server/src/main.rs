mod consumer;
mod web;

#[macro_use]
extern crate rocket;

use crate::consumer::civilisation_admin::handle_service_planet_added;
use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand, ValueEnum};
use consumer::civilisation::handle_account_public_event_for_planet;
use horfimbor_eventsource::cache_db::redis::StateDb;
use horfimbor_eventsource::repository::{Repository, StateRepository};
use kurrentdb::Client;
use planet_admin::PlanetAdminState;
use planet_state::PlanetState;
use rocket::futures::future::try_join_all;
use rocket::futures::{FutureExt, StreamExt};
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::env;
use url::Url;

type PlanetStateCache = StateDb<PlanetState>;
type PlanetRepository = StateRepository<PlanetState, PlanetStateCache>;

type PlanetAdminStateCache = StateDb<PlanetAdminState>;
type PlanetAdminRepository = StateRepository<PlanetAdminState, PlanetAdminStateCache>;

#[derive(Debug, PartialEq, Clone, ValueEnum)]
enum Service {
    Web,
    Delay,
    State,
    Dto,
    AccountCreated,
    PlanetOwnerChange,
    AccountCreatedForPlanet,
    AdminServicePlanetAdded,
}

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    real_env: bool,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Cli {
        #[arg(long)]
        hello: String,
    },
    Service {
        #[arg(long)]
        list: Vec<Service>,
    },
}

#[rocket::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if !args.real_env {
        dotenvy::dotenv().context("cannot get env")?;
        dotenvy::from_filename_override(".env.planet").context("cannot get env")?;
    }

    let app_host = Url::parse(&env::var("APP_HOST").context("fail to get APP_HOST env var")?)
        .context("cannot parse APP_HOST as url")?;

    let settings = env::var("EVENTSTORE_URI")
        .context("fail to get EVENTSTORE_URI env var")?
        .parse()
        .context("fail to parse the settings")?;

    let redis_client =
        redis::Client::open(env::var("REDIS_URI").context("fail to get REDIS_URI env var")?)?;

    let event_store_db =
        Client::new(settings).map_err(|e| anyhow!(" cannot connect to eventstore : {e}"))?;

    let repo_planet_state = PlanetRepository::new(
        event_store_db.clone(),
        PlanetStateCache::new(redis_client.clone()),
    );

    let repo_planet_admin = PlanetAdminRepository::new(
        event_store_db.clone(),
        PlanetAdminStateCache::new(redis_client.clone()),
    );

    match args.command {
        Command::Service { list } => {
            let mut services = Vec::new();

            if list.is_empty() || list.contains(&Service::Web) {
                services.push(
                    web::start_server(
                        event_store_db.clone(),
                        repo_planet_state.clone(),
                        repo_planet_admin.clone(),
                        redis_client.clone(),
                        app_host.port(),
                    )
                    .boxed(),
                );
            }

            if list.is_empty() || list.contains(&Service::AccountCreatedForPlanet) {
                services.push(
                    handle_account_public_event_for_planet(
                        event_store_db.clone(),
                        repo_planet_state,
                    )
                    .boxed(),
                );
            }

            if list.is_empty() || list.contains(&Service::AdminServicePlanetAdded) {
                services.push(
                    handle_service_planet_added(event_store_db, repo_planet_admin, app_host)
                        .boxed(),
                );
            }

            let signals = Signals::new([SIGTERM, SIGINT, SIGQUIT])?;

            let signals_task = handle_signals(signals).boxed();
            services.push(signals_task);

            dbg!(services.len());

            try_join_all(services)
                .await
                .map(|_| ())
                .context("some service failed")
        }
        Command::Cli { hello } => {
            println!("hello {hello} !");
            Ok(())
        }
    }
}

async fn handle_signals(mut signals: Signals) -> Result<()> {
    if signals.next().await.is_some() {
        bail!("Exit required")
    }

    Ok(())
}
