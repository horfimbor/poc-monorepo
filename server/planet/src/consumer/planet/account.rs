use crate::PlanetRepository;
use anyhow::Context;
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Event, Stream};
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};
use planet_shared::command::PlanetCommand;
use public_mono::account::PubAccountEvent;
use public_mono::planet::PLANET_STREAM;

pub async fn handle_account_public_event_for_planet(
    event_store_db: Client,
    planet_repository: PlanetRepository,
) -> anyhow::Result<()> {
    let e = PubAccountEvent::Created {
        name: "".to_string(),
        owner: "".to_string(),
    };

    let stream = Stream::Event(e.event_name());
    let group_name = "mono_planet_new_account";

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

        for _ in 0..3 {
            let planet_id = ModelKey::new_uuid_v7(PLANET_STREAM);

            planet_repository
                .add_command(
                    &planet_id,
                    PlanetCommand::Create {
                        account_id: event.stream_id().to_string(),
                    },
                    Some(&metadata),
                )
                .await
                .context("cannot create planet")?;
        }

        sub.ack(&rcv_event).await.context("cannot ack")?;
    }
}
