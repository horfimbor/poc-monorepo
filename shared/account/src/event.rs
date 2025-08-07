#[cfg(feature = "server")]
use crate::ACCOUNT_STATE_NAME;
#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};

use crate::Nation;
use common::Component;
use common::account::PubAccountEvent;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(ACCOUNT_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PrvAccountEvent {
    NationUpdated(Nation),
    WorldAdded(Component),
    WorldRemoved(String),
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", composite_state)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AccountEvent {
    Private(PrvAccountEvent),
    Public(PubAccountEvent),
}
