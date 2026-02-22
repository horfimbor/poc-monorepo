use horfimbor_eventsource::horfimbor_eventsource_derive::{Event, StateNamed};
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
use horfimbor_eventsource::{Event, EventName};
use planet_shared::PLANET_STATE_NAME;
use planet_shared::command::PlanetCommand;
use planet_shared::dto::{
    Building, PlanetDto, Production, Resource, ResourceCalc,
};
use planet_shared::error::PlanetError;
use planet_shared::error::PlanetError::*;
use planet_shared::event::SharedPlanetEvent;
use planet_shared::event::SharedPlanetEvent::*;
use public_mono::planet::PubPlanetEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, StateNamed, Default)]
#[state(PLANET_STATE_NAME)]
pub struct PlanetState {
    shared: PlanetDto,
    owner: ModelKey,
    countdown: usize,
}

#[derive(Event)]
#[state(PLANET_STATE_NAME)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PrvPlanetEvent {
    LowerCountDown(usize),
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

impl PlanetState {
    #[must_use]
    pub fn owner(&self) -> &ModelKey {
        &self.owner
    }

    pub fn shared(&self) -> &PlanetDto {
        &self.shared
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
                PrvPlanetEvent::LowerCountDown(_) => {
                    self.countdown -= 1;
                }
            },
            PlanetEvent::Public(event) => match event {
                PubPlanetEvent::NewOwner {
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
            PlanetCommand::ChangeOwner { account_id } => {
                let model: Result<ModelKey, _> = account_id.as_str().try_into();

                if model.is_err() {
                    return Err(PlanetError::InvalidOwner);
                }
                Ok(vec![PlanetEvent::Public(PubPlanetEvent::NewOwner {
                    old_account_id: Some(self.owner.to_string()),
                    account_id,
                })])
            }
            PlanetCommand::Create { account_id } => {
                let model: Result<ModelKey, _> = account_id.as_str().try_into();

                if model.is_err() {
                    return Err(PlanetError::InvalidOwner);
                }

                Ok(vec![
                    PlanetEvent::Public(PubPlanetEvent::NewOwner {
                        old_account_id: None,
                        account_id,
                    }),
                    PlanetEvent::Shared(UpdateResource {
                        resource: Resource::Population,
                        calc: ResourceCalc {
                            quantity: 10000,
                            date_time: Default::default(),
                            production: 0,
                            stock_capacity: 100000,
                        },
                    }),
                    PlanetEvent::Shared(UpdateResource {
                        resource: Resource::Electricity,
                        calc: ResourceCalc {
                            quantity: 11000,
                            date_time: Default::default(),
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
            PlanetCommand::StartConstruction { key } => {
                let Some(building) = self.shared.available_building.get(&key) else {
                    return Err(AvailableIdNotExists);
                };

                let mut events = vec![PlanetEvent::Shared(UpdateConstruction {
                    key: Uuid::new_v4(),
                    building: building.clone(),
                })];

                for (resource, quantity) in building.construction.iter() {
                    let Some(calc) = self.shared.resources.get(&resource) else {
                        return Err(NotEnoughResources(*resource));
                    };

                    if *quantity < calc.compute_quantity() {
                        return Err(NotEnoughResources(*resource));
                    }

                    events.push(PlanetEvent::Shared(UpdateResource {
                        resource: *resource,
                        calc: ResourceCalc {
                            quantity: calc.compute_quantity() - quantity,
                            date_time: Default::default(), // TODO
                            production: calc.production,
                            stock_capacity: calc.stock_capacity,
                        },
                    }));
                }

                Ok(events)
            }
            PlanetCommand::CancelConstruction { key } => {
                let Some(building) = self.shared.construction.get(&key) else {
                    return Err(ConstructionIdNotExists);
                };

                let mut events = vec![PlanetEvent::Shared(RemoveConstruction { key })];


                for (resource, quantity) in building.construction.iter() {

                    let calc =
                        self.shared
                            .resources
                            .get(&resource).cloned()
                            .unwrap_or_default();

                    events.push(PlanetEvent::Shared(UpdateResource {
                        resource: *resource,
                        calc: ResourceCalc {
                            quantity: calc.compute_quantity() + quantity / 2,
                            date_time: Default::default(), // TODO
                            production: calc.production,
                            stock_capacity: calc.stock_capacity,
                        },
                    }));
                }

                Ok(events)
            }
            PlanetCommand::FinnishConstruction { key } => {
                let Some(building) = self.shared.construction.get(&key) else {
                    return Err(ConstructionIdNotExists);
                };

                let mut events = vec![
                    PlanetEvent::Shared(RemoveConstruction { key }),
                    PlanetEvent::Shared(UpdateRunningBuilding {
                        key: Uuid::new_v4(),
                        building: building.clone(),
                    }),
                ];

                for (resource, quantity) in building.production.iter() {
                    let calc =
                        self.shared
                            .resources
                            .get(&resource)
                            .cloned()
                            .unwrap_or_default();

                    events.push(PlanetEvent::Shared(UpdateResource {
                        resource: *resource,
                        calc: ResourceCalc {
                            quantity: calc.compute_quantity(),
                            date_time: Default::default(), // TODO
                            production: calc.production + (quantity.quantity as isize),
                            stock_capacity: calc.stock_capacity + quantity.stock,
                        },
                    }));
                }

                Ok(events)
            }
            PlanetCommand::DestroyConstruction { key } => {
                let Some(building) = self.shared.buildings.get(&key) else {
                    return Err(BuildingIdNotExists);
                };


                let mut events = vec![
                    PlanetEvent::Shared(RemoveRunningBuilding { key }),
                ];

                for (resource, quantity) in building.production.iter() {
                    let calc =
                        self.shared
                            .resources
                            .get(&resource)
                            .cloned()
                            .unwrap_or_default();

                    events.push(PlanetEvent::Shared(UpdateResource {
                        resource: *resource,
                        calc: ResourceCalc {
                            quantity: calc.compute_quantity(),
                            date_time: Default::default(), // TODO
                            production: calc.production - (quantity.quantity as isize),
                            stock_capacity: calc.stock_capacity - quantity.stock,
                        },
                    }));
                }


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
                //             date_time: Default::default(), // TODO
                //             production: calc.production,
                //             stock_capacity: calc.stock_capacity ,
                //         },
                //     }));
                // }

                Ok(events)
            }
        }
    }
}
