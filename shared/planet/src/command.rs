#[cfg(feature = "server")]
use crate::PLANET_STATE_NAME;
#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Command;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Command, CommandName};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Command))]
#[cfg_attr(feature = "server", state(PLANET_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlanetCommand {
    Create { account_id: String },
    ChangeOwner { account_id: String },
    Ping,
}
