#[cfg(feature = "server")]
use crate::ACCOUNT_STATE_NAME;
#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Command;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Command, CommandName};

use crate::Nation;
use common::Component;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Command))]
#[cfg_attr(feature = "server", state(ACCOUNT_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AccountCommand {
    Create { name: String, owner: String },
    UpdateNation(Nation),
    AddWorld(Component),
    RemoveWorld(String),
}
