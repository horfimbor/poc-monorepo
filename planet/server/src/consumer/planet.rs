use crate::{PlanetAdminRepository, PlanetRepository};
use anyhow::{Context, Error};
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::Repository;
use horfimbor_eventsource::{Event, Stream};
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};
use planet_shared::event::SharedPlanetEvent;
use planet_state::PlanetCommand;
use planet_state::PrivatePlanetCommand::FinnishConstruction;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
use url::Url;

pub async fn handle_planet_start_building(
    event_store_db: Client,
    planet_repository: PlanetRepository,
    state_admin_repository: PlanetAdminRepository,
) -> anyhow::Result<()> {
    let e = SharedPlanetEvent::UpdateConstruction {
        key: Default::default(),
        building: Default::default(),
    };

    let stream = Stream::Event(e.event_name());
    let group_name = "mono_planet_start_building";

    create_subscription(&event_store_db, &stream, group_name)
        .await
        .context("cannot create subscription")?;

    let options = SubscribeToPersistentSubscriptionOptions::default().buffer_size(1);

    let mut sub = event_store_db
        .subscribe_to_persistent_subscription(stream.to_string(), group_name, &options)
        .await
        .context("cannot subscribe")?;

    let planet_repository = planet_repository.clone();
    let planet_admin_repository = state_admin_repository.clone();
    loop {
        let rcv_event = sub.next().await.context("cannot get next event")?;

        let full_event = match rcv_event.event.as_ref() {
            None => {
                continue;
            }
            Some(event) => event,
        };

        // FIXME change this metadata check
        let metadata: Metadata = serde_json::from_slice(full_event.custom_metadata.as_ref())
            .context("cannot deserialize")?;

        let event = rcv_event.event.as_ref().context("cannot extract event")?;

        let json = event
            .as_json::<SharedPlanetEvent>()
            .context("cannot extract json")?;

        let model_key =
            ModelKey::try_from(event.stream_id()).context("cannot convert streamId to ModelKey")?;

        if let SharedPlanetEvent::UpdateConstruction { key, .. } = json {
            let planet_repository = planet_repository.clone();
            let planet_admin_repository = planet_admin_repository.clone();

            tokio::spawn(async move {
                let model = planet_repository
                    .clone()
                    .get_model(&model_key)
                    .await
                    .context("cannot get model")?;

                let admin_model = planet_admin_repository
                    .clone()
                    .get_model(model.state().planet_admin())
                    .await
                    .context("cannot get admin model")?;

                let now = SystemTime::now();
                let epoch = now
                    .duration_since(UNIX_EPOCH)
                    .context("cannot get timestamp")?
                    .as_secs();

                let to_wait = 10;
                // TODO compute duration
                dbg!(to_wait);
                if to_wait > 0 {
                    sleep(Duration::from_secs(1) * to_wait as u32).await;
                }

                let s = planet_repository
                    .add_command(
                        &model_key,
                        PlanetCommand::Private(FinnishConstruction {
                            key,
                            time_config: admin_model.state().time(),
                        }),
                        None,
                    )
                    .await
                    .context("cannot add command")?;

                dbg!(s);

                Ok::<(), Error>(())
            });
        }

        sub.ack(&rcv_event).await.context("cannot ack")?;
    }
}
