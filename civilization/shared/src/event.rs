#[cfg(feature = "server")]
use crate::CIVILIZATION_CONFIG_STATE_NAME;
#[cfg(feature = "server")]
use crate::CIVILIZATION_STATE_NAME;
use crate::Nation;
#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};
use horfimbor_time::HfTimeConfiguration;
use public_mono::Component;
use public_mono::civilization::PubCivilizationAdminEvent;
use serde::{Deserialize, Serialize};
use url::Url;

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(CIVILIZATION_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SharedCivilizationEvent {
    SetTime(HfTimeConfiguration),
    NationUpdated(Nation),
    WorldAdded(Component),
    WorldRemoved(String),
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(CIVILIZATION_CONFIG_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SharedCivilizationAdminEvent {
    Created(Url),
    TimeSet(HfTimeConfiguration),
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", composite_state)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CivilizationAdminEvent {
    Shared(SharedCivilizationAdminEvent),
    Public(PubCivilizationAdminEvent),
}
