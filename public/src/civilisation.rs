pub const PUB_ACCOUNT_EVENT: &str = "PUB_MONO_CIVILISATION_EVENT";
pub const MONO_CIVILISATION_STREAM: &str = "mono_civilisation";
pub const UUID_V8_KIND: &str = "ACCOUNT";

#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(PUB_ACCOUNT_EVENT))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PubAccountEvent {
    Created { name: String, owner: String },
}
