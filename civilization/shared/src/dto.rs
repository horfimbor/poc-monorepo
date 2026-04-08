use crate::Nation;
use crate::admin::Service;
use crate::event::{CivilizationAdminEvent, SharedCivilizationAdminEvent, SharedCivilizationEvent};
#[cfg(feature = "server")]
use horfimbor_eventsource::Dto;
use horfimbor_time::HfTimeConfiguration;
use public_mono::Component;
use public_mono::civilization::PubCivilizationAdminEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default, Eq)]
pub struct CivilizationDto {
    nation: Option<Nation>,
    worlds: Vec<Component>,
    time: Option<HfTimeConfiguration>,
}

impl CivilizationDto {
    pub fn play_event(&mut self, event: &SharedCivilizationEvent) {
        match event {
            SharedCivilizationEvent::NationUpdated(nation) => {
                self.nation = Some(nation.clone());
            }
            SharedCivilizationEvent::WorldAdded(world) => self.worlds.push(world.clone()),
            SharedCivilizationEvent::WorldRemoved(id) => self.worlds.retain(|w| !w.id.eq(id)),
            SharedCivilizationEvent::SetTime(config) => self.time = Some(*config),
        }
    }

    #[must_use]
    pub fn nation(&self) -> &Option<Nation> {
        &self.nation
    }

    #[must_use]
    pub fn worlds(&self) -> &Vec<Component> {
        &self.worlds
    }

    #[must_use]
    pub fn time(&self) -> Option<HfTimeConfiguration> {
        self.time
    }
}

#[cfg(feature = "server")]
impl Dto for CivilizationDto {
    type Event = SharedCivilizationEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct CivilizationAdminDto {
    host: Option<Url>,
    time: Option<HfTimeConfiguration>,
    game_services: HashMap<String, Service>,
}

impl CivilizationAdminDto {
    pub fn play_event(&mut self, event: &CivilizationAdminEvent) {
        match event {
            CivilizationAdminEvent::Shared(event) => match event {
                SharedCivilizationAdminEvent::Created(host) => {
                    self.host = Some(host.clone());
                }
                SharedCivilizationAdminEvent::TimeSet(timer) => {
                    self.time = Some(*timer);
                }
            },
            CivilizationAdminEvent::Public(event) => match event {
                PubCivilizationAdminEvent::AddedService {
                    name,
                    game_host: _game_host,
                    service_host,
                    time: _time,
                    balise: tag,
                } => {
                    self.game_services.insert(
                        name.clone(),
                        Service {
                            url: service_host.clone(),
                            balise: tag.clone(),
                        },
                    );
                }
                PubCivilizationAdminEvent::RemovedService {
                    name,
                    game_host: _game_host,
                    service_host: _service_sort,
                } => {
                    self.game_services.remove(name);
                }
            },
        }
    }

    pub fn host(&self) -> &Option<Url> {
        &self.host
    }

    pub fn time(&self) -> Option<HfTimeConfiguration> {
        self.time
    }

    pub fn game_services(&self) -> &HashMap<String, Service> {
        &self.game_services
    }
}

#[cfg(feature = "server")]
impl Dto for CivilizationAdminDto {
    type Event = CivilizationAdminEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}
