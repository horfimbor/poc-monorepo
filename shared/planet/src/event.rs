#[cfg(feature = "server")]
use crate::PLANET_STATE_NAME;
#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(PLANET_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SharedPlanetEvent {
    Created(usize),
    Pong(usize),
    Boom(usize),
}
