use crate::PlanetAdminRepository;
use anyhow::Context;
use chrono::{Duration, Utc};
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Event, Stream};
use horfimbor_time::HfTimeConfiguration;
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};
use planet_admin::PlanetAdminCommand;
use public_mono::civilisation::PubConfigCivEvent;
use public_mono::planet::{PLANET_ADMIN_STREAM, UUID_ADMIN_V8_KIND};
use url::Url;

pub async fn handle_service_planet_added(
    event_store_db: Client,
    planet_repository: PlanetAdminRepository,
    current_host: Url,
) -> anyhow::Result<()> {
    let e = PubConfigCivEvent::AddService {
        game_host: Url::parse("http://localhost").context("cannot create localhost dummy event")?,
        service_host: Url::parse("http://localhost")
            .context("cannot create localhost dummy event")?,
        time: HfTimeConfiguration::new(Duration::minutes(1), Duration::seconds(1), Utc::now())
            .context("cannot create dummy time configuration")?,
    };

    let stream = Stream::Event(e.event_name());
    let group_name = "mono_planet_service_added";

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
        let metadata: Metadata = serde_json::from_slice(full_event.custom_metadata.as_ref())
            .context("cannot deserialize")?;

        let event = rcv_event.event.as_ref().context("cannot extract event")?;

        let json = event
            .as_json::<PubConfigCivEvent>()
            .context("cannot extract json")?;

        if let PubConfigCivEvent::AddService {
            game_host,
            time,
            service_host,
        } = json
            && service_host == current_host
        {
            let key = ModelKey::new_uuid_v8(
                PLANET_ADMIN_STREAM,
                UUID_ADMIN_V8_KIND,
                current_host.as_str(),
            );

            planet_repository
                .add_command(
                    &key,
                    PlanetAdminCommand::Setup(time, game_host.clone()),
                    Some(&metadata),
                )
                .await
                .context("cannot create account")?;

            sub.ack(&rcv_event).await.context("cannot ack")?;
        }
    }
}
