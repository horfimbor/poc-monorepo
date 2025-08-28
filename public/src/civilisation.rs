pub const PUB_MONO_CIVILISATION_EVENT: &str = "PUB_MONO_CIVILISATION_EVENT";
pub const MONO_CIVILISATION_STREAM: &str = "mono_civilisation";
pub const UUID_V8_KIND: &str = "ACCOUNT";

pub const PUB_CONFIG_CIVILISATION_EVENT: &str = "PUB_CONFIG_CIVILISATION_EVENT";

#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};

use horfimbor_time::HfTimeConfiguration;
use serde::{Deserialize, Serialize};

use url::Host;

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(PUB_CONFIG_CIVILISATION_EVENT))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PubConfigCivEvent {
    AddService { game_host: Host, service_host: Host, time: HfTimeConfiguration},
    RemoveService { game_host: Host, service_host: Host },
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(PUB_MONO_CIVILISATION_EVENT))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PubCivilisationEvent {
    Created {
        game_host: Host,
        name: String,
        owner: String,
    },
}
