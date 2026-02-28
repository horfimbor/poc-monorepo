use crate::event::SharedPlanetEvent;
use chrono::{DateTime, Utc};
#[cfg(feature = "server")]
use horfimbor_eventsource::Dto;
use horfimbor_time::HfTimeConfiguration;
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash, Copy)]
pub enum Resource {
    #[serde(rename = "e")]
    Electricity,
    #[serde(rename = "p")]
    Population,
    #[serde(rename = "s")]
    Steal,
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::Electricity => {
                write!(f, "Electricity")
            }
            Resource::Population => {
                write!(f, "Population")
            }
            Resource::Steal => {
                write!(f, "Steal")
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default, Eq)]
pub struct Production {
    #[serde(rename = "q")]
    pub quantity: usize,
    #[serde(rename = "s")]
    pub stock: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default, Eq)]
pub struct Building {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "c")]
    pub construction: HashMap<Resource, usize>,
    #[serde(rename = "ct")]
    pub construction_time: usize,
    #[serde(rename = "rc")]
    pub running_cost: HashMap<Resource, usize>,
    #[serde(rename = "p")]
    pub production: HashMap<Resource, Production>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq)]
pub struct ResourceCalc {
    #[serde(rename = "q")]
    pub quantity: usize,
    #[serde(rename = "d")]
    pub date_time: DateTime<Utc>,
    #[serde(rename = "p")]
    pub production: i64,
    #[serde(rename = "s")]
    pub stock_capacity: usize,
}

impl Default for ResourceCalc {
    fn default() -> Self {
        Self {
            quantity: 0,
            date_time: Utc::now(),
            production: 0,
            stock_capacity: 0,
        }
    }
}

impl ResourceCalc {
    pub fn compute_quantity(&self, time_config: HfTimeConfiguration) -> Self {
        let date_time = Utc::now();

        let new_quantity = min(
            time_config.diff_hf_millis(self.date_time, date_time) * self.production,
            self.stock_capacity as i64,
        );

        Self {
            quantity: max(new_quantity, 0) as usize,
            date_time,
            production: self.production,
            stock_capacity: self.stock_capacity,
        }
    }
}

impl Add<i64> for ResourceCalc {
    type Output = Self;
    fn add(self, rhs: i64) -> Self::Output {
        let quantity = max(
            0,
            min((self.quantity as i64 + rhs) as usize, self.stock_capacity),
        );

        Self {
            quantity,
            date_time: self.date_time,
            production: self.production,
            stock_capacity: self.stock_capacity,
        }
    }
}

impl Sub<i64> for ResourceCalc {
    type Output = Self;

    fn sub(self, rhs: i64) -> Self::Output {
        let quantity = max(
            0,
            min((self.quantity as i64 - rhs) as usize, self.stock_capacity),
        );
        Self {
            quantity,
            date_time: self.date_time,
            production: self.production,
            stock_capacity: self.stock_capacity,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default, Eq)]
pub struct PlanetDto {
    #[serde(rename = "rs")]
    pub resources: HashMap<Resource, ResourceCalc>,
    #[serde(rename = "av")]
    pub available_building: HashMap<Uuid, Building>,
    #[serde(rename = "c")]
    pub construction: HashMap<Uuid, Building>,
    #[serde(rename = "b")]
    pub buildings: HashMap<Uuid, Building>,
}

impl PlanetDto {
    pub fn play_event(&mut self, event: &SharedPlanetEvent) {
        match event {
            SharedPlanetEvent::UpdateResource { resource, calc } => {
                self.resources.insert(*resource, calc.clone());
            }
            SharedPlanetEvent::UpdateAvailableBuilding { key, building } => {
                self.available_building.insert(*key, building.clone());
            }
            SharedPlanetEvent::RemoveAvailableBuilding { key } => {
                self.available_building.remove(key);
            }
            SharedPlanetEvent::UpdateConstruction { key, building } => {
                self.construction.insert(*key, building.clone());
            }
            SharedPlanetEvent::RemoveConstruction { key } => {
                self.construction.remove(key);
            }
            SharedPlanetEvent::UpdateRunningBuilding { key, building } => {
                self.buildings.insert(*key, building.clone());
            }
            SharedPlanetEvent::RemoveRunningBuilding { key } => {
                self.buildings.remove(key);
            }
        }
    }
}

#[cfg(feature = "server")]
impl Dto for PlanetDto {
    type Event = SharedPlanetEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}
