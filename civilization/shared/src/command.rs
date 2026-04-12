#[cfg(feature = "server")]
use crate::CIVILIZATION_CONFIG_STATE_NAME;
#[cfg(feature = "server")]
use crate::CIVILIZATION_STATE_NAME;
use crate::Nation;
use crate::admin::Service;
#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Command;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Command, CommandName};
use horfimbor_time::HfTimeConfiguration;
use public_mono::Component;
use serde::{Deserialize, Serialize};
use url::Url;

#[cfg_attr(feature = "server", derive(Command))]
#[cfg_attr(feature = "server", state(CIVILIZATION_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CivilizationCommand {
    Create {
        name: String,
        user_id: String,
        owner: String,
        game_host: Url,
        time: HfTimeConfiguration,
    },
    UpdateNation(Nation),
    AddWorld(Component),
    RemoveWorld(String),
}

#[cfg_attr(feature = "server", derive(Command))]
#[cfg_attr(feature = "server", state(CIVILIZATION_CONFIG_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CivilizationAdminCommand {
    CreateServer(Url),
    AddTime(HfTimeConfiguration),
    AddService { name: String, comp: Service },
    RemoveService(String),
}
