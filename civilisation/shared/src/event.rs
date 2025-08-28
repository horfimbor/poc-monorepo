#[cfg(feature = "server")]
use crate::CIVILISATION_STATE_NAME;
#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};

use crate::Nation;
use public_mono::Component;
use public_mono::civilisation::PubCivilisationEvent;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(CIVILISATION_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SharedCivilisationEvent {
    NationUpdated(Nation),
    WorldAdded(Component),
    WorldRemoved(String),
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(CIVILISATION_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PrvCivilisationEvent {
    NothingYet,
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", composite_state)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CivilisationEvent {
    Private(PrvCivilisationEvent),
    Shared(SharedCivilisationEvent),
    Public(PubCivilisationEvent),
}
