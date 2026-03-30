#[cfg(feature = "server")]
use crate::CIVILISATION_STATE_NAME;
#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};
use horfimbor_time::HfTimeConfiguration;
use crate::Nation;
use public_mono::Component;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(CIVILISATION_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SharedCivilisationEvent {
    SetTime(HfTimeConfiguration),
    NationUpdated(Nation),
    WorldAdded(Component),
    WorldRemoved(String),
}
