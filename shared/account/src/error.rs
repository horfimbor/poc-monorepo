use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AccountError {
    AlreadyCreated,
    AccountNameCannotBeEmpty,
    NationNameCannotBeEmpty,
    WorldAlreadyAdded(String),
    WorldNotFound(String),
}

impl Display for AccountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyCreated => {
                write!(f, "account already created")
            }
            Self::AccountNameCannotBeEmpty => {
                write!(f, "account name cannot be empty")
            }
            Self::NationNameCannotBeEmpty => {
                write!(f, "nation name cannot be empty")
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
