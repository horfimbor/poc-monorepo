use crate::{PlanetRepository, consumer};
use anyhow::Context;
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Event, Stream};
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};
use planet_shared::command::SharedPlanetCommand;
use planet_state::PlanetCommand;
use public_mono::civilisation::{PubCivilisationAdminEvent, PubCivilisationEvent};
use public_mono::planet::PLANET_STREAM;
use url::Url;

pub async fn handle_account_public_event_for_planet(
    event_store_db: Client,
    planet_repository: PlanetRepository,
    current_host: Url,
) -> anyhow::Result<()> {
    let e = PubCivilisationEvent::Created {
        game_host: Url::parse("http://localhost").context("cannot create localhost dummy event")?,
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

        let json = event
            .as_json::<PubCivilisationEvent>()
            .context("cannot extract json")?;

        match json {
            PubCivilisationEvent::Created { game_host, .. } => {
                let planet_id = ModelKey::new_uuid_v7(PLANET_STREAM);

                let admin_id = consumer::generate_admin_id(&game_host, &current_host);

                planet_repository
                    .add_command(
                        &planet_id,
                        PlanetCommand::Shared(SharedPlanetCommand::Create {
                            account_id: event.stream_id().to_string(),
                            admin_id,
                        }),
                        Some(&metadata),
                    )
                    .await
                    .context("cannot create planet")?;
            }
        }

        sub.ack(&rcv_event).await.context("cannot ack")?;
    }
}
