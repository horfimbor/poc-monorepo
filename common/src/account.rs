pub const PUB_ACCOUNT_EVENT: &str = "PUB_MONO_ACCOUNT_EVENT";
pub const ACCOUNT_STREAM: &str = "mono_account";
pub const UUID_V8_KIND: &str = "ACCOUNT";

#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::model_key::ModelKey;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(PUB_ACCOUNT_EVENT))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PubAccountEvent {
    Created(String, ModelKey),
}
