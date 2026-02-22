use chrono::{DateTime, Utc};
#[cfg(feature = "server")]
use horfimbor_eventsource::Dto;
use std::collections::HashMap;

use crate::event::SharedPlanetEvent;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Default)]
pub struct ResourceCalc {
    #[serde(rename = "q")]
    pub quantity: usize,
    #[serde(rename = "d")]
    pub date_time: DateTime<Utc>,
    #[serde(rename = "p")]
    pub production: isize,
    #[serde(rename = "s")]
    pub stock_capacity: usize,
}

impl ResourceCalc {
    pub fn compute_quantity(&self) -> usize {
        // TODO compute the production since date_time
        self.quantity
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
