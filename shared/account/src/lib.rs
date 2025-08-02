use serde::{Deserialize, Serialize};

pub mod command;
pub mod dto;
pub mod error;
pub mod event;

pub const START_VALUE: usize = 1337;

pub const ACCOUNT_STATE_NAME: &str = "MONO_ACCOUNT_STATE";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Nation {
    pub name: String,
    pub description: String,
}
