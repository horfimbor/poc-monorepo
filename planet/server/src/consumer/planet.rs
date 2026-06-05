use crate::PlanetStateCache;
use anyhow::Context;
use horfimbor_callback_recall::database::{CallBack, Pool};
use horfimbor_callback_recall::{SchedulerBuilder, SchedulerEmitter};
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::StateRepository;
use horfimbor_eventsource::{Event, EventName, Stream};
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};
use planet_shared::event::SharedPlanetEvent;
use planet_state::PrivatePlanetCommand::FinnishConstruction;
use planet_state::{PlanetCommand, PlanetState};
use uuid::Uuid;

pub async fn handle_planet_start_building<P>(
    event_store_db: Client,
    emitter: SchedulerEmitter<P>,
    event_name: EventName,
) -> anyhow::Result<()>
where
    P: Pool,
{
    let stream = Stream::Event(event_name);
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
                    event_name.to_string(),
                    Vec::from(format!("{model_key}:{key}")),
                    end,
                ))
                .await
                .context("cannot schedule event")?;
        }

        sub.ack(&rcv_event).await.context("cannot ack")?;
    }
}

pub fn listen_planet_start_building<P: Pool>(
    planet_repository: &StateRepository<PlanetState, PlanetStateCache>,
    builder: &mut SchedulerBuilder<P>,
) -> EventName {
    let e = SharedPlanetEvent::UpdateConstruction {
        key: Default::default(),
        building: Default::default(),
        end: Default::default(),
    };
    let event_name = e.event_name();

    let planet_repository_register = planet_repository.clone();

    builder.register(event_name, move |payload| {
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
    event_name
}
