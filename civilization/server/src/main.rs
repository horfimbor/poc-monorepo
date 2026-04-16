mod consumer;
mod web;

#[macro_use]
extern crate rocket;

use crate::web::AuthConfig;
use anyhow::{Context, Result, anyhow, bail};
use civilization_state::CivilizationState;
use civilization_state::admin::CivilizationAdminState;
use clap::{Parser, Subcommand, ValueEnum};
use consumer::auth::handle_account_public_event;
use consumer::planet::handle_planet_public_event;
use horfimbor_eventsource::cache_db::redis::StateDb;
use horfimbor_eventsource::repository::{Repository, StateRepository};
use kurrentdb::Client;
use rocket::futures::future::try_join_all;
use rocket::futures::{FutureExt, StreamExt};
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::env;
use url::Url;

type CivilizationStateCache = StateDb<CivilizationState>;
type CivilizationRepository = StateRepository<CivilizationState, CivilizationStateCache>;

type CivilizationAdminStateCache = StateDb<CivilizationAdminState>;
type CivilizationAdminRepository =
    StateRepository<CivilizationAdminState, CivilizationAdminStateCache>;

#[derive(Debug, PartialEq, Clone, ValueEnum)]
enum Service {
    Web,
    Delay,
    State,
    Dto,
    AccountCreated,
    PlanetOwnerChange,
    AccountCreatedForPlanet,
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
    }

    let settings = env::var("EVENTSTORE_URI")
        .context("fail to get EVENTSTORE_URI env var")?
        .parse()
        .context("fail to parse the settings")?;

    let redis_client =
        redis::Client::open(env::var("REDIS_URI").context("fail to get REDIS_URI env var")?)?;

    let event_store_db =
        Client::new(settings).map_err(|e| anyhow!(" cannot connect to eventstore : {e}"))?;

    let repo_civilization_state = CivilizationRepository::new(
        event_store_db.clone(),
        CivilizationStateCache::new(redis_client.clone()),
    );

    let repo_civilization_admin_state = CivilizationAdminRepository::new(
        event_store_db.clone(),
        CivilizationAdminStateCache::new(redis_client.clone()),
    );

    let app_host = Url::parse(&env::var("APP_HOST").context("fail to get APP_HOST env var")?)
        .context("cannot parse APP_HOST as url")?;
    let app_key = env::var("APP_KEY").context("APP_KEY is not defined")?;

    let auth_host = Url::parse(&env::var("AUTH_HOST").context("AUTH_HOST is not defined")?)
        .context("cannot parse AUTH_HOST as url")?;

    let auth_callback_host =
        Url::parse(&env::var("AUTH_CALLBACK_HOST").context("AUTH_CALLBACK_HOST is not defined")?)
            .context("cannot parse AUTH_CALLBACK_HOST as url")?;

    let auth_config = AuthConfig {
        app_host: app_host.clone(),
        app_key,
        auth_host,
        auth_callback_host,
    };

    match args.command {
        Command::Service { list } => {
            let mut services = Vec::new();

            if list.is_empty() || list.contains(&Service::Web) {
                services.push(
                    web::start_server(
                        event_store_db.clone(),
                        repo_civilization_state.clone(),
                        repo_civilization_admin_state.clone(),
                        redis_client.clone(),
                        auth_config.clone(),
                    )
                    .boxed(),
                );
            }

            if list.is_empty() || list.contains(&Service::AccountCreated) {
                services.push(
                    handle_account_public_event(
                        event_store_db.clone(),
                        repo_civilization_state.clone(),
                        repo_civilization_admin_state.clone(),
                        auth_config,
                    )
                    .boxed(),
                );
            }
            if list.is_empty() || list.contains(&Service::PlanetOwnerChange) {
                services.push(
                    handle_planet_public_event(event_store_db.clone(), repo_civilization_state)
                        .boxed(),
                );
            }

            let signals = Signals::new([SIGTERM, SIGINT, SIGQUIT])?;

            let signals_task = handle_signals(signals).boxed();
            services.push(signals_task);

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
