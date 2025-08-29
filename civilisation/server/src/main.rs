mod consumer;
mod web;

#[macro_use]
extern crate rocket;

use anyhow::{Context, Result, anyhow, bail};
use civilisation_admin::CivilisationAdminState;
use civilisation_state::CivilisationState;
use clap::{Parser, Subcommand, ValueEnum};
use consumer::civilisation::auth::handle_account_public_event;
use consumer::civilisation::planet::handle_planet_public_event;
use horfimbor_eventsource::cache_db::redis::StateDb;
use horfimbor_eventsource::repository::{Repository, StateRepository};
use kurrentdb::Client;
use rocket::futures::future::try_join_all;
use rocket::futures::{FutureExt, StreamExt};
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::env;

type CivilisationStateCache = StateDb<CivilisationState>;
type CivilisationRepository = StateRepository<CivilisationState, CivilisationStateCache>;

type CivilisationAdminStateCache = StateDb<CivilisationAdminState>;
type CivilisationAdminRepository =
    StateRepository<CivilisationAdminState, CivilisationAdminStateCache>;

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

    #[arg(short, long, default_value_t = 3000)]
    port: u16,
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

    let repo_civilisation_state = CivilisationRepository::new(
        event_store_db.clone(),
        CivilisationStateCache::new(redis_client.clone()),
    );

    let repo_civilisation_admin_state = CivilisationAdminRepository::new(
        event_store_db.clone(),
        CivilisationAdminStateCache::new(redis_client.clone()),
    );

    match args.command {
        Command::Service { list } => {
            let mut services = Vec::new();

            if list.is_empty() || list.contains(&Service::Web) {
                services.push(
                    web::start_server(
                        event_store_db.clone(),
                        repo_civilisation_state.clone(),
                        repo_civilisation_admin_state.clone(),
                        redis_client.clone(),
                    )
                    .boxed(),
                );
            }

            if list.is_empty() || list.contains(&Service::AccountCreated) {
                services.push(
                    handle_account_public_event(
                        event_store_db.clone(),
                        repo_civilisation_state.clone(),
                    )
                    .boxed(),
                );
            }
            if list.is_empty() || list.contains(&Service::PlanetOwnerChange) {
                services.push(
                    handle_planet_public_event(event_store_db.clone(), repo_civilisation_state)
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
