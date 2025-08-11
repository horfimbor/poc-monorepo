#[cfg(feature = "server")]
use crate::CIVILISATION_STATE_NAME;
#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Command;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Command, CommandName};

use crate::Nation;
use public_mono::Component;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Command))]
#[cfg_attr(feature = "server", state(CIVILISATION_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CivilisationCommand {
    Create { name: String, owner: String },
    UpdateNation(Nation),
    AddWorld(Component),
    RemoveWorld(String),
}
