use crate::{ PlanetRepository};
use anyhow::{Context, Error};
use chrono::{ Utc};
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Event, Stream};
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};
use planet_shared::event::SharedPlanetEvent;
use planet_state::PlanetCommand;
use planet_state::PrivatePlanetCommand::FinnishConstruction;
pub use std::time::{Duration};
use tokio::time::sleep;

pub async fn handle_planet_start_building(
    event_store_db: Client,
    planet_repository: PlanetRepository,
) -> anyhow::Result<()> {
    let e = SharedPlanetEvent::UpdateConstruction {
        key: Default::default(),
        building: Default::default(),
        end: Default::default(),
    };

    let stream = Stream::Event(e.event_name());
    let group_name = "mono_planet_start_building";

    create_subscription(&event_store_db, &stream, group_name)
        .await
        .context("cannot create subscription")?;

    let options = SubscribeToPersistentSubscriptionOptions::default()
        .buffer_size(1);

    let mut sub = event_store_db
        .subscribe_to_persistent_subscription(stream.to_string(), group_name, &options)
        .await
        .context("cannot subscribe")?;

    let planet_repository = planet_repository.clone();
    loop {
        let rcv_event = sub.next().await.context("cannot get next event")?;

        let full_event = match rcv_event.event.as_ref() {
            None => {
                continue;
            }
            Some(event) => event,
        };

        let event = rcv_event.event.as_ref().context("cannot extract event")?;

        let json = event
            .as_json::<SharedPlanetEvent>()
            .context("cannot extract json")?;

        let model_key =
            ModelKey::try_from(event.stream_id()).context("cannot convert streamId to ModelKey")?;

        if let SharedPlanetEvent::UpdateConstruction { key, end, .. } = json {
            let planet_repository = planet_repository.clone();

            tokio::spawn(async move {
                let now = Utc::now();

                let to_wait = (end - now).num_seconds();
                dbg!(to_wait);
                if to_wait > 0 {
                    sleep(Duration::from_secs(to_wait as u64)).await;
                }

                let _s = planet_repository
                    .add_command(
                        &model_key,
                        PlanetCommand::Private(FinnishConstruction { key }),
                        None,
                    )
                    .await
                    .context("cannot add command")?;

                Ok::<(), Error>(())
            });
        }

        sub.ack(&rcv_event).await.context("cannot ack")?;
    }
}
