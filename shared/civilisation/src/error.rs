use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CivilisationError {
    AlreadyCreated,
    InvalidOwner,
    AccountNameCannotBeEmpty,
    InvalidNation(String),
    WorldAlreadyAdded(String),
    WorldNotFound(String),
}

impl Display for CivilisationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyCreated => {
                write!(f, "account already created")
            }
            Self::InvalidOwner => {
                write!(f, "owner id is not a modelkey")
            }
            Self::AccountNameCannotBeEmpty => {
                write!(f, "account name cannot be empty")
            }
            Self::InvalidNation(err) => {
                write!(f, "nation is invalide: {err}")
            }
            Self::WorldAlreadyAdded(id) => {
                write!(f, "cannot add again world {id}")
            }
            Self::WorldNotFound(id) => {
                write!(f, "cannot remove not found world {id}")
            }
        }
    }
}
