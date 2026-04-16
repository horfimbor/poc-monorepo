use crate::{PlanetAdminRepository, PlanetRepository, get_admin_id};
use anyhow::Context;
use chrono::{Duration, Utc};
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::repository::Repository;
use horfimbor_eventsource::{Event, Stream};
use horfimbor_time::HfTimeConfiguration;
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};
use planet_shared::command::SharedPlanetCommand;
use planet_state::PlanetCommand;
use public_mono::civilization::PubCivilizationEvent;
use public_mono::planet::PLANET_STREAM;
use std::env;
use url::Url;

pub async fn handle_account_public_event_for_planet(
    event_store_db: Client,
    planet_repository: PlanetRepository,
    planet_admin_repository: PlanetAdminRepository,
    current_host: Url,
) -> anyhow::Result<()> {
    let e = PubCivilizationEvent::Created {
        game_host: Url::parse("http://localhost").context("cannot create localhost dummy event")?,
        name: "".to_string(),
        owner: "".to_string(),
        user_id: "".to_string(),
        time: HfTimeConfiguration::new(Duration::minutes(2), Duration::minutes(1), Utc::now())?,
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
            .as_json::<PubCivilizationEvent>()
            .context("cannot extract json")?;

        match json {
            PubCivilizationEvent::Created {
                time,
                owner,
                user_id,
                ..
            } => {
                let app_id = env::var("APP_ID").context("APP_ID is missing")?;
                let audience = app_id
                    .as_str()
                    .try_into()
                    .context("audience is not a ModelKey")?;

                let admin_id = get_admin_id(&audience, &current_host);

                let admin = planet_admin_repository
                    .get_model(&admin_id)
                    .await
                    .context("cannot load admin")?;
                let admin = admin.state();

                for _ in 0..admin.dto().nb_planet() {
                    let planet_id = ModelKey::new_uuid_v7(PLANET_STREAM);

                    planet_repository
                        .add_command(
                            &planet_id,
                            PlanetCommand::Shared(SharedPlanetCommand::Create {
                                owner: owner.clone(),
                                user_id: user_id.clone(),
                                admin_id: admin_id.to_string(),
                                time,
                            }),
                            Some(&metadata),
                        )
                        .await
                        .context("cannot create planet")?;
                }
            }
        }

        sub.ack(&rcv_event).await.context("cannot ack")?;
    }
}
