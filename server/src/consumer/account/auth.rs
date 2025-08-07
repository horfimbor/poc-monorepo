use crate::AccountRepository;
use account_shared::command::AccountCommand;
use anyhow::{Context, anyhow};
use common::account::{MONO_ACCOUNT_STREAM, UUID_V8_KIND};
use horfimbor_eventsource::helper::create_subscription;
use horfimbor_eventsource::metadata::Metadata;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Event, Stream};
use kurrentdb::{Client, SubscribeToPersistentSubscriptionOptions};
use public_account_event::PubAccountEvent;
use std::env;

pub async fn handle_account_public_event(
    event_store_db: Client,
    repository: AccountRepository,
) -> anyhow::Result<()> {
    let current_app_id = env::var("APP_ID").map_err(|_| anyhow!("APP_ID is missing"))?;

    let e = PubAccountEvent::AccountCreated {
        user_id: ModelKey::default(),
        app_id: ModelKey::default(),
        name: "".to_string(),
    };

    let stream = Stream::Event(e.event_name());
    let group_name = "mono_account_event";

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
            .as_json::<PubAccountEvent>()
            .context("cannot extract json")?;

        if let PubAccountEvent::AccountCreated {
            user_id,
            app_id,
            name,
        } = json
            && current_app_id == app_id.to_string()
        {
            let key = ModelKey::new_uuid_v8(MONO_ACCOUNT_STREAM, UUID_V8_KIND, event.stream_id());
            repository
                .add_command(
                    &key,
                    AccountCommand::Create {
                        name,
                        owner: user_id.to_string(),
                    },
                    Some(&metadata),
                )
                .await
                .context("cannot create account")?;
        };

        // todo!("check app id and then create the account");
        sub.ack(&rcv_event).await.context("cannot ack")?;
    }
}
