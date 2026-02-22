use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::dto::Resource;

#[derive(Error, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlanetError {
    InvalidOwner,
    AvailableIdNotExists,
    ConstructionIdNotExists,
    BuildingIdNotExists,
    NotEnoughResources(Resource)
}

impl Display for PlanetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidOwner => {
                write!(f, "owner id is not a modelkey")
            }
            PlanetError::AvailableIdNotExists => {
                write!(f, "id is not in available building to construct")
            }
            PlanetError::ConstructionIdNotExists => {
                write!(f, "id is not in building being construct")
            }
            PlanetError::BuildingIdNotExists => {
                write!(f, "id is not a building")
            }
            PlanetError::NotEnoughResources(r) => {
                write!(f, "Not enough {:?} (as least)", r)
            }
        }
    }
}
