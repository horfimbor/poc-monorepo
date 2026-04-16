pub const PUB_MONO_CIVILIZATION_EVENT: &str = "PUB_MONO_CIVILIZATION_EVENT";
pub const MONO_CIVILIZATION_STREAM: &str = "mono_civilization";
pub const UUID_V8_KIND: &str = "ACCOUNT";

pub const MONO_CIVILIZATION_ADMIN_STREAM: &str = "admin_civilization";
pub const UUID_ADMIN_V8_KIND: &str = "ACCOUNT_ADMIN";

pub const PUB_CONFIG_CIVILIZATION_EVENT: &str = "PUB_CONFIG_CIVILIZATION_EVENT";

#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};

use horfimbor_time::HfTimeConfiguration;
use serde::{Deserialize, Serialize};

use url::Url;

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(PUB_CONFIG_CIVILIZATION_EVENT))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PubCivilizationAdminEvent {
    AddedService {
        name: String,
        game_host: Url,
        service_host: Url,
        balise: String,
        time: HfTimeConfiguration,
    },
    RemovedService {
        name: String,
        game_host: Url,
        service_host: Url,
    },
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(PUB_MONO_CIVILIZATION_EVENT))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PubCivilizationEvent {
    Created {
        game_host: Url,
        name: String,
        owner: String,
        user_id: String,
        time: HfTimeConfiguration,
    },
}
