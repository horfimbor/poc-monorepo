mod consumer;
mod web;

#[macro_use]
extern crate rocket;

use crate::consumer::civilization_admin::handle_service_planet_added;
use crate::consumer::planet::handle_planet_start_building;
use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand, ValueEnum};
use consumer::civilization::handle_account_public_event_for_planet;
use consumer::planet;
use horfimbor_callback_recall::database::sqlite::open;
use horfimbor_callback_recall::{SchedulerBuilder, SchedulerListener};
use horfimbor_eventsource::cache_db::redis::StateDb;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::{Repository, StateRepository};
use kurrentdb::Client;
use planet_state::PlanetState;
use planet_state::admin::PlanetAdminState;
use public_mono::planet::{PLANET_ADMIN_STREAM, UUID_ADMIN_V8_KIND};
use rocket::futures::future::try_join_all;
use rocket::futures::{FutureExt, StreamExt};
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::env;
use std::time::Duration;
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
    PlanetStartConstruction,
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

    let planet_repository = PlanetRepository::new(
        event_store_db.clone(),
        PlanetStateCache::new(redis_client.clone()),
    );

    let planet_admin_repository = PlanetAdminRepository::new(
        event_store_db.clone(),
        PlanetAdminStateCache::new(redis_client.clone()),
    );

    let db = open("test").await.context("cannot create sqlite db")?;

    let mut builder = SchedulerBuilder::new(db, Duration::from_secs(2))
        .await
        .context("cannot create builder")?;

    let event_start_building_name =
        planet::listen_planet_start_building(&planet_repository, &mut builder);

    let (emitter, listener) = builder.start();

    match args.command {
        Command::Service { list } => {
            let mut services = Vec::new();

            if list.is_empty() || list.contains(&Service::Web) {
                services.push(
                    web::start_server(
                        event_store_db.clone(),
                        planet_repository.clone(),
                        planet_admin_repository.clone(),
                        redis_client.clone(),
                        app_host.clone(),
                    )
                    .boxed(),
                );
            }

            if list.is_empty() || list.contains(&Service::AccountCreatedForPlanet) {
                services.push(
                    handle_account_public_event_for_planet(
                        event_store_db.clone(),
                        planet_repository.clone(),
                        planet_admin_repository.clone(),
                        app_host.clone(),
                    )
                    .boxed(),
                );
            }

            if list.is_empty() || list.contains(&Service::AdminServicePlanetAdded) {
                services.push(
                    handle_service_planet_added(
                        event_store_db.clone(),
                        planet_admin_repository.clone(),
                        app_host,
                    )
                    .boxed(),
                );
            }
            if list.is_empty() || list.contains(&Service::PlanetStartConstruction) {
                services.push(
                    handle_planet_start_building(
                        event_store_db,
                        emitter,
                        event_start_building_name,
                    )
                    .boxed(),
                );
            }

            services.push(join_error(listener).boxed());

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

async fn join_error(l: SchedulerListener) -> Result<(), anyhow::Error> {
    l.join().await;

    Ok(())
}

async fn handle_signals(mut signals: Signals) -> Result<()> {
    if signals.next().await.is_some() {
        bail!("Exit required")
    }

    Ok(())
}

fn get_admin_id(audience: &ModelKey, service_host: &Url) -> ModelKey {
    ModelKey::new_uuid_v8(
        PLANET_ADMIN_STREAM,
        UUID_ADMIN_V8_KIND,
        format!("{audience},{service_host}").as_str(),
    )
}
