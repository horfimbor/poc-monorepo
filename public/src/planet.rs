pub const PUB_PLANET_EVENT: &str = "PUB_MONO_PLANET_EVENT";
pub const PLANET_STREAM: &str = "mono_planet";
pub const PLANET_ADMIN_STREAM: &str = "mono_planet_admin";
pub const UUID_V8_KIND: &str = "PLANET";
pub const UUID_ADMIN_V8_KIND: &str = "PLANET_ADMIN";

#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(PUB_PLANET_EVENT))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PubPlanetEvent {
    NewOwner {
        old_account_id: Option<String>,
        account_id: String,
    },
}
