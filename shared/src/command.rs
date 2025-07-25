#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Command;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Command, CommandName};

#[cfg(feature = "server")]
use crate::MONO_STATE_NAME;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Delay {
    pub delay: usize,
    pub to_add: usize,
}

#[cfg_attr(feature = "server", derive(Command))]
#[cfg_attr(feature = "server", state(MONO_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MonoCommand {
    Delayed(Delay),
    Finalize(usize),
    Add(usize),
    Reset,
}
