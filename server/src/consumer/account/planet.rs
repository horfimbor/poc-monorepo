use crate::AccountRepository;
use account_shared::command::AccountCommand;
use anyhow::Context;
use common::Component;
use common::planet::PubPlanetEvent;
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::{Event, Stream};
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};

pub async fn handle_planet_public_event(
    event_store_db: Client,
    account_repository: AccountRepository,
) -> anyhow::Result<()> {
    let e = PubPlanetEvent::NewOwner {
        old_account_id: None,
        account_id: "".to_string(),
    };

    let stream = Stream::Event(e.event_name());
    let group_name = "mono_account_add_planet";

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
            .as_json::<PubPlanetEvent>()
            .context("cannot extract json")?;

        let PubPlanetEvent::NewOwner {
            old_account_id,
            account_id,
        } = json;

        if let Some(old_account_id) = old_account_id {
            account_repository
                .add_command(
                    &old_account_id
                        .as_str()
                        .try_into()
                        .context("cannot parse old_account_id")?,
                    AccountCommand::RemoveWorld(event.stream_id().to_string()),
                    Some(&metadata),
                )
                .await
                .context("cannot remove world")?;
        }

        account_repository
            .add_command(
                &account_id
                    .as_str()
                    .try_into()
                    .context("cannot parse account_id")?,
                AccountCommand::AddWorld(Component {
                    balise: "horfimbor-planet-state".to_string(),
                    id: event.stream_id().to_string(),
                }),
                Some(&metadata),
            )
            .await
            .context("cannot add planet to account")?;

        // todo!("check app id and then create the planet");
        sub.ack(&rcv_event).await.context("cannot ack")?;
    }
}
