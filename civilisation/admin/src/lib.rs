#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::{Command, Event, StateNamed};
#[cfg(feature = "server")]
use horfimbor_eventsource::{Command, CommandName};
#[cfg(feature = "server")]
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};
use horfimbor_time::HfTimeConfiguration;
use public_mono::civilisation::PubCivilisationAdminEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use url::Url;

pub const CIVILISATION_CONFIG_STATE_NAME: &str = "CIVILISATION_CONFIG_STATE";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Component {
    pub url: Url,
    pub balise: String,
}

#[cfg_attr(feature = "server", derive(Command))]
#[cfg_attr(feature = "server", state(CIVILISATION_CONFIG_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CivilisationAdminCommand {
    CreateServer(Url),
    AddTime(HfTimeConfiguration),
    AddComponent { name: String, comp: Component },
    RemoveComponent(String),
}

#[derive(Error, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CivilisationAdminError {
    AlreadyCreated,
    AlreadyHaveTime,
    NotCreatedYet,
    ComponentAlreadyExists,
    ComponentNameAlreadyExists,
}

impl Display for CivilisationAdminError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyCreated => {
                write!(f, "cannot recreate civilisation")
            }
            Self::AlreadyHaveTime => {
                write!(f, "cannot change time")
            }
            Self::NotCreatedYet => {
                write!(f, "cannot add component to not created config")
            }
            Self::ComponentAlreadyExists => {
                write!(f, "component already exists")
            }
            CivilisationAdminError::ComponentNameAlreadyExists => {
                write!(f, "component name must be unique")
            }
        }
    }
}

#[cfg_attr(feature = "server", derive(StateNamed))]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "server", state(CIVILISATION_CONFIG_STATE_NAME))]
pub struct CivilisationAdminState {
    host: Option<Url>,
    time: Option<HfTimeConfiguration>,
    game_components: HashMap<String, Component>,
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(CIVILISATION_CONFIG_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PrvCivilisationAdminEvent {
    Created(Url),
    TimeSet(HfTimeConfiguration),
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", composite_state)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CivilisationAdminEvent {
    Private(PrvCivilisationAdminEvent),
    Public(PubCivilisationAdminEvent),
}

impl CivilisationAdminState {
    pub fn play_event(&mut self, event: &CivilisationAdminEvent) {
        match event {
            CivilisationAdminEvent::Private(event) => match event {
                PrvCivilisationAdminEvent::Created(host) => {
                    self.host = Some(host.clone());
                }
                PrvCivilisationAdminEvent::TimeSet(timer) => {
                    self.time = Some(*timer);
                }
            },
            CivilisationAdminEvent::Public(event) => match event {
                PubCivilisationAdminEvent::AddedService {
                    name,
                    game_host: _game_host,
                    service_host,
                    time: _time,
                    balise: tag,
                } => {
                    self.game_components.insert(
                        name.clone(),
                        Component {
                            url: service_host.clone(),
                            balise: tag.clone(),
                        },
                    );
                }
                PubCivilisationAdminEvent::RemovedService {
                    name,
                    game_host: _game_host,
                    service_host: _service_sort,
                } => {
                    self.game_components.remove(name);
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

    pub fn game_components(&self) -> &HashMap<String, Component> {
        &self.game_components
    }
}

#[cfg(feature = "server")]
impl Dto for CivilisationAdminState {
    type Event = CivilisationAdminEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event)
    }
}

#[cfg(feature = "server")]
impl State for CivilisationAdminState {
    type Command = CivilisationAdminCommand;
    type Error = CivilisationAdminError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            CivilisationAdminCommand::CreateServer(host) => {
                if self.host.is_some() {
                    return Err(CivilisationAdminError::AlreadyCreated);
                }

                Ok(vec![CivilisationAdminEvent::Private(
                    PrvCivilisationAdminEvent::Created(host),
                )])
            }
            CivilisationAdminCommand::AddTime(timer) => {
                if self.time.is_some() {
                    return Err(CivilisationAdminError::AlreadyHaveTime);
                }

                Ok(vec![CivilisationAdminEvent::Private(
                    PrvCivilisationAdminEvent::TimeSet(timer),
                )])
            }
            CivilisationAdminCommand::AddComponent { name, comp } => {
                let (Some(game_host), Some(time)) = (self.host.clone(), self.time) else {
                    return Err(CivilisationAdminError::NotCreatedYet);
                };

                if self.game_components.contains_key(&name) {
                    return Err(CivilisationAdminError::ComponentNameAlreadyExists);
                }

                if self
                    .game_components
                    .values()
                    .find(|gc| gc.url == comp.url)
                    .is_some()
                {
                    return Err(CivilisationAdminError::ComponentAlreadyExists);
                };

                Ok(vec![CivilisationAdminEvent::Public(
                    PubCivilisationAdminEvent::AddedService {
                        name,
                        game_host,
                        service_host: comp.url,
                        balise: comp.balise,
                        time,
                    },
                )])
            }
            CivilisationAdminCommand::RemoveComponent(name) => {
                let Some(game_host) = self.host.clone() else {
                    return Err(CivilisationAdminError::NotCreatedYet);
                };

                if let Some(comp) = self.game_components.get(&name) {
                    Ok(vec![CivilisationAdminEvent::Public(
                        PubCivilisationAdminEvent::RemovedService {
                            name,
                            game_host,
                            service_host: comp.url.clone(),
                        },
                    )])
                } else {
                    Err(CivilisationAdminError::ComponentAlreadyExists)
                }
            }
        }
    }
}
