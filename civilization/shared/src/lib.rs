use garde::Validate;
use serde::{Deserialize, Serialize};

pub mod admin;
pub mod command;
pub mod dto;
pub mod error;
pub mod event;

pub const START_VALUE: usize = 1337;

pub const CIVILIZATION_STATE_NAME: &str = "MONO_civilization_state";
pub const CIVILIZATION_CONFIG_STATE_NAME: &str = "CIVILIZATION_CONFIG_STATE";

#[derive(Validate, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Nation {
    #[garde(length(utf16, min = 3, max = 25))]
    pub name: String,
    #[garde(length(utf16, min = 15))]
    pub description: String,
}
