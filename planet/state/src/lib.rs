use chrono::Utc;
use horfimbor_eventsource::horfimbor_eventsource_derive::{Command, Event, StateNamed};
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Command, CommandName, Event, EventName};
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};

use planet_shared::PLANET_STATE_NAME;
use planet_shared::command::SharedPlanetCommand;
use planet_shared::dto::{Building, PlanetDto, Production, Resource, ResourceCalc};
use planet_shared::error::PlanetError;
use planet_shared::error::PlanetError::*;
use planet_shared::event::SharedPlanetEvent;
use planet_shared::event::SharedPlanetEvent::*;
use public_mono::planet::PubPlanetEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, StateNamed, Default)]
#[state(PLANET_STATE_NAME)]
pub struct PlanetState {
    shared: PlanetDto,
    owner: ModelKey,
    planet_admin: ModelKey,
    countdown: usize,
}

#[derive(Event)]
#[state(PLANET_STATE_NAME)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PrvPlanetEvent {
    PlanetAdminSet(ModelKey),
}

#[derive(Event)]
#[composite_state]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PlanetEvent {
    Private(PrvPlanetEvent),
    Shared(SharedPlanetEvent),
    Public(PubPlanetEvent),
}

#[derive(Command)]
#[state(PLANET_STATE_NAME)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PrivatePlanetCommand {
    CalculateStock {
        resource: Resource,
        calc: ResourceCalc,
    },
}

#[derive(Command)]
#[state(PLANET_STATE_NAME)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PlanetCommand {
    Private(PrivatePlanetCommand),
    Shared(SharedPlanetCommand),
}

impl PlanetState {
    #[must_use]
    pub fn owner(&self) -> &ModelKey {
        &self.owner
    }

    pub fn shared(&self) -> &PlanetDto {
        &self.shared
    }

    pub fn planet_admin(&self) -> &ModelKey {
        &self.planet_admin
    }
}

impl Dto for PlanetState {
    type Event = PlanetEvent;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            PlanetEvent::Shared(event) => {
                self.shared.play_event(event);
                // match event {
                //     Created(_) => {
                //         self.countdown = 25;
                //     }
                //     Pong(_) => {}
                //     Boom(_) => {
                //         self.countdown = 100;
                //     }
                // }
            }
            PlanetEvent::Private(event) => match event {
                PrvPlanetEvent::PlanetAdminSet(model_key) => self.planet_admin = model_key.clone(),
            },
            PlanetEvent::Public(event) => match event {
                PubPlanetEvent::NewOwner {
                    endpoint: _,
                    old_account_id: _,
                    account_id,
                } => {
                    self.owner = account_id.as_str().try_into().unwrap_or_default();
                }
            },
        }
    }
}

impl State for PlanetState {
    type Command = PlanetCommand;
    type Error = PlanetError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            PlanetCommand::Shared(command) => {
                match command {
                    SharedPlanetCommand::ChangeOwner { account_id } => {
                        let model: Result<ModelKey, _> = account_id.as_str().try_into();

                        if model.is_err() {
                            return Err(InvalidOwner);
                        }

                        let Ok(app_host) = env::var("APP_HOST")else{
                            return Err(NoAppHost);
                        };

                        Ok(vec![PlanetEvent::Public(PubPlanetEvent::NewOwner {
                            endpoint: app_host,
                            old_account_id: Some(self.owner.to_string()),
                            account_id,
                        })])
                    }
                    SharedPlanetCommand::Create {
                        account_id,
                        admin_id,
                    } => {
                        let model: Result<ModelKey, _> = account_id.as_str().try_into();

                        if model.is_err() {
                            return Err(InvalidOwner);
                        }
                        let Ok(admin_id) = admin_id.as_str().try_into() else {
                            return Err(InvalidAdminId);
                        };

                        let Ok(app_host) = env::var("APP_HOST")else{
                            return Err(NoAppHost);
                        };

                        Ok(vec![
                            PlanetEvent::Private(PrvPlanetEvent::PlanetAdminSet(admin_id)),
                            PlanetEvent::Public(PubPlanetEvent::NewOwner {
                                endpoint: app_host,
                                old_account_id: None,
                                account_id,
                            }),
                            PlanetEvent::Shared(UpdateResource {
                                resource: Resource::Population,
                                calc: ResourceCalc {
                                    quantity: 10000,
                                    date_time: Utc::now(),
                                    production: 0,
                                    stock_capacity: 100000,
                                },
                            }),
                            PlanetEvent::Shared(UpdateResource {
                                resource: Resource::Electricity,
                                calc: ResourceCalc {
                                    quantity: 11000,
                                    date_time: Utc::now(),
                                    production: 0,
                                    stock_capacity: 0,
                                },
                            }),
                            PlanetEvent::Shared(UpdateAvailableBuilding {
                                key: Uuid::new_v4(),
                                building: Building {
                                    name: "Steal factory".to_string(),
                                    construction: HashMap::from([
                                        (Resource::Electricity, 998),
                                        (Resource::Population, 800),
                                    ]),
                                    construction_time: 100,
                                    running_cost: HashMap::from([
                                        (Resource::Population, 23),
                                        (Resource::Electricity, 51),
                                    ]),
                                    production: HashMap::from([(
                                        Resource::Steal,
                                        Production {
                                            quantity: 49,
                                            stock: 1000,
                                        },
                                    )]),
                                },
                            }),
                        ])
                    }
                    SharedPlanetCommand::StartConstruction { key, time_config } => {
                        let Some(building) = self.shared.available_building.get(&key) else {
                            return Err(AvailableIdNotExists);
                        };

                        let Some(time_config) = time_config else {
                            return Err(NoTimeConfig);
                        };

                        let mut events = vec![PlanetEvent::Shared(UpdateConstruction {
                            key: Uuid::new_v4(),
                            building: building.clone(),
                        })];

                        for (resource, quantity) in building.construction.iter() {
                            let Some(calc) = self.shared.resources.get(&resource) else {
                                return Err(NotEnoughResources(*resource));
                            };

                            let new_quantities = calc.compute_quantity(time_config);

                            if *quantity < new_quantities.quantity {
                                return Err(NotEnoughResources(*resource));
                            }

                            events.push(PlanetEvent::Shared(UpdateResource {
                                resource: *resource,
                                calc: new_quantities - *quantity as i64,
                            }));
                        }

                        Ok(events)
                    }
                    SharedPlanetCommand::CancelConstruction { key, time_config } => {
                        let Some(building) = self.shared.construction.get(&key) else {
                            return Err(ConstructionIdNotExists);
                        };

                        let Some(time_config) = time_config else {
                            return Err(NoTimeConfig);
                        };

                        let mut events = vec![PlanetEvent::Shared(RemoveConstruction { key })];

                        for (resource, quantity) in building.construction.iter() {
                            let calc = self
                                .shared
                                .resources
                                .get(&resource)
                                .cloned()
                                .unwrap_or_default();

                            let new_quantities = calc.compute_quantity(time_config);

                            events.push(PlanetEvent::Shared(UpdateResource {
                                resource: *resource,
                                calc: new_quantities + *quantity as i64 / 2,
                            }));
                        }

                        Ok(events)
                    }
                    SharedPlanetCommand::FinnishConstruction { key, time_config } => {
                        let Some(building) = self.shared.construction.get(&key) else {
                            return Err(ConstructionIdNotExists);
                        };

                        let Some(time_config) = time_config else {
                            return Err(NoTimeConfig);
                        };

                        let mut events = vec![
                            PlanetEvent::Shared(RemoveConstruction { key }),
                            PlanetEvent::Shared(UpdateRunningBuilding {
                                key: Uuid::new_v4(),
                                building: building.clone(),
                            }),
                        ];

                        for (resource, quantity) in building.production.iter() {
                            let calc = self
                                .shared
                                .resources
                                .get(&resource)
                                .cloned()
                                .unwrap_or_default();

                            let mut new_quantities = calc.compute_quantity(time_config);
                            new_quantities.production =
                                new_quantities.production + quantity.quantity as i64;
                            new_quantities.stock_capacity =
                                new_quantities.stock_capacity + quantity.stock;

                            events.push(PlanetEvent::Shared(UpdateResource {
                                resource: *resource,
                                calc: new_quantities,
                            }));
                        }

                        Ok(events)
                    }
                    SharedPlanetCommand::DestroyConstruction { key, time_config } => {
                        let Some(building) = self.shared.buildings.get(&key) else {
                            return Err(BuildingIdNotExists);
                        };

                        let Some(time_config) = time_config else {
                            return Err(NoTimeConfig);
                        };

                        let mut events = vec![PlanetEvent::Shared(RemoveRunningBuilding { key })];

                        for (resource, quantity) in building.production.iter() {
                            let calc = self
                                .shared
                                .resources
                                .get(&resource)
                                .cloned()
                                .unwrap_or_default();

                            let mut new_quantities = calc.compute_quantity(time_config);
                            new_quantities.production =
                                new_quantities.production - quantity.quantity as i64;
                            new_quantities.stock_capacity =
                                new_quantities.stock_capacity - quantity.stock;

                            events.push(PlanetEvent::Shared(UpdateResource {
                                resource: *resource,
                                calc: new_quantities,
                            }));
                        }

                        // TODO give back some resources ?
                        // for (resource, quantity) in building.construction.into_iter() {
                        //     let calc =
                        //         self.shared
                        //             .resources
                        //             .get(&resource)
                        //             .cloned()
                        //             .unwrap_or_default();
                        //
                        //     events.push(PlanetEvent::Shared(UpdateResource {
                        //         resource,
                        //         calc: ResourceCalc {
                        //             quantity: calc.compute_quantity() + quantity / 5,
                        //             date_time: Utc::now(),
                        //             production: calc.production,
                        //             stock_capacity: calc.stock_capacity ,
                        //         },
                        //     }));
                        // }

                        Ok(events)
                    }
                }
            }
            PlanetCommand::Private(command) => match command {
                PrivatePlanetCommand::CalculateStock { resource, calc } => {
                    Ok(vec![PlanetEvent::Shared(UpdateResource { resource, calc })])
                }
            },
        }
    }
}
