use crate::PlanetRepository;
use anyhow::Context;
use horfimbor_callback_recall::SchedulerBuilder;
use horfimbor_callback_recall::database::CallBack;
use horfimbor_callback_recall::database::sqlite::open;
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Event, Stream};
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};
use planet_shared::event::SharedPlanetEvent;
use planet_state::PlanetCommand;
use planet_state::PrivatePlanetCommand::FinnishConstruction;
pub use std::time::Duration;
use uuid::Uuid;

pub async fn handle_planet_start_building(
    event_store_db: Client,
    planet_repository: PlanetRepository,
) -> anyhow::Result<()> {
    let e = SharedPlanetEvent::UpdateConstruction {
        key: Default::default(),
        building: Default::default(),
        end: Default::default(),
    };

    let db = open("test").await.context("cannot create sqlite db")?;

    let mut builder = SchedulerBuilder::new(db, Duration::from_secs(2))
        .await
        .context("cannot create builder")?;

    let planet_repository_register = planet_repository.clone();

    builder.register(e.event_name(), move |payload| {
        let planet_repository = planet_repository_register.clone();
        async move {
            let data = String::from_utf8(payload).map_err(|e| e.to_string())?;

            let Some((model_key, key)) = data.split_once(":") else {
                return Err(format!("cannot split: {data}"));
            };

            let model_key =
                ModelKey::try_from(model_key).map_err(|e| format!("bad model_key: {}", e))?;

            let key = Uuid::try_from(key).map_err(|e| format!("bad key: {}", e))?;

            let _s = planet_repository
                .clone()
                .add_command(
                    &model_key,
                    PlanetCommand::Private(FinnishConstruction { key }),
                    None,
                )
                .await
                .map_err(|e| format!("cannot add command: {}", e))?;

            Ok(())
        }
    });

    let (emitter, listener) = builder.start();

    tokio::spawn(async move {
        listener.join().await;
    });

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

    loop {
        let rcv_event = sub.next().await.context("cannot get next event")?;

        let event = rcv_event.event.as_ref().context("cannot extract event")?;

        let json = event
            .as_json::<SharedPlanetEvent>()
            .context("cannot extract json")?;

        let model_key =
            ModelKey::try_from(event.stream_id()).context("cannot convert streamId to ModelKey")?;

        if let SharedPlanetEvent::UpdateConstruction { key, end, .. } = json {
            emitter
                .schedule(CallBack::new(
                    e.event_name().to_string(),
                    Vec::from(format!("{model_key}:{key}")),
                    end,
                ))
                .await
                .context("cannot schedule event")?;
        }

        sub.ack(&rcv_event).await.context("cannot ack")?;
    }
}
