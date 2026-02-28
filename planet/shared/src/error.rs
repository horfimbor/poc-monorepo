use crate::dto::Resource;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlanetError {
    InvalidOwner,
    InvalidAdminId,
    InvalidCivilisation,
    AvailableIdNotExists,
    ConstructionIdNotExists,
    BuildingIdNotExists,
    NoTimeConfig,
    NoAppHost,
    NotEnoughResources(Resource),
}

impl Display for PlanetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidOwner => {
                write!(f, "owner id is not a ModelKey")
            }
            Self::InvalidCivilisation => {
                write!(f, "civilisation id is not a ModelKey")
            }
            Self::InvalidAdminId => {
                write!(f, "admin id is not a ModelKey")
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
            PlanetError::NoTimeConfig => {
                write!(f, "Time config not loaded")
            }
            PlanetError::NoAppHost => {
                write!(f, "APP_HOST notr defined")
            }
        }
    }
}
