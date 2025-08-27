#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::{Command, Event, StateNamed};
#[cfg(feature = "server")]
use horfimbor_eventsource::{Command, CommandName};
#[cfg(feature = "server")]
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};
use horfimbor_time::HfTimeConfiguration;
use public_mono::civilisation::PubConfigCivEvent;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use url::Host;

pub const CIVILISATION_CONFIG_STATE_NAME: &str = "CIVILISATION_CONFIG_STATE";

#[cfg_attr(feature = "server", derive(Command))]
#[cfg_attr(feature = "server", state(CIVILISATION_CONFIG_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CivilisationAdminCommand {
    CreateServer(Host, HfTimeConfiguration),
    AddComponent(Host),
    RemoveComponent(Host),
}

#[derive(Error, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CivilisationAdminError {
    AlreadyCreated,
    NotCreatedYet,
}

impl Display for CivilisationAdminError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyCreated => {
                write!(f, "cannot recreate civilisation")
            }
            Self::NotCreatedYet => {
                write!(f, "cannot add component to not created config")
            }
        }
    }
}

#[cfg_attr(feature = "server", derive(StateNamed))]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "server", state(CIVILISATION_CONFIG_STATE_NAME))]
pub struct CivilisationAdminState {
    host: Option<Host>,
    time: Option<HfTimeConfiguration>,
    game_components: Vec<Host>,
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(CIVILISATION_CONFIG_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PrvCivilisationAdminEvent {
    Created(Host, HfTimeConfiguration),
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", composite_state)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CivilisationAdminEvent {
    Private(PrvCivilisationAdminEvent),
    Public(PubConfigCivEvent),
}

impl CivilisationAdminState {
    pub fn play_event(&mut self, event: &CivilisationAdminEvent) {
        match event {
            CivilisationAdminEvent::Private(event) => match event {
                PrvCivilisationAdminEvent::Created(host, timer) => {
                    self.host = Some(host.clone());
                    self.time = Some(timer.clone());
                }
            },
            CivilisationAdminEvent::Public(event) => {
                match event {
                    PubConfigCivEvent::AddService {
                        game_host: _game_host,
                        service_host,
                    } => self.game_components.push(service_host.clone()),
                    PubConfigCivEvent::RemoveService {
                        game_host: _game_host,
                        service_host,
                    } => self.game_components.retain(|h| *h != *service_host),
                    PubConfigCivEvent::SetTime(_) => {
                        // timer cannot be change
                    }
                }
            }
        }
    }

    pub fn host(&self) -> &Option<Host> {
        &self.host
    }

    pub fn time(&self) -> Option<HfTimeConfiguration> {
        self.time
    }

    pub fn game_components(&self) -> &Vec<Host> {
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
            CivilisationAdminCommand::CreateServer(host, timer) => {
                if self.host.is_some() {
                    return Err(CivilisationAdminError::AlreadyCreated);
                }

                Ok(vec![CivilisationAdminEvent::Private(
                    PrvCivilisationAdminEvent::Created(host, timer),
                )])
            }
            CivilisationAdminCommand::AddComponent(service_host) => {
                let (Some(game_host), Some(timer)) = (self.host.clone(), self.time.clone()) else {
                    return Err(CivilisationAdminError::NotCreatedYet);
                };

                Ok(vec![
                    CivilisationAdminEvent::Public(PubConfigCivEvent::AddService {
                        game_host,
                        service_host,
                    }),
                    CivilisationAdminEvent::Public(PubConfigCivEvent::SetTime(timer)),
                ])
            }
            CivilisationAdminCommand::RemoveComponent(service_host) => {
                let (Some(game_host), Some(timer)) = (self.host.clone(), self.time.clone()) else {
                    return Err(CivilisationAdminError::NotCreatedYet);
                };

                Ok(vec![
                    CivilisationAdminEvent::Public(PubConfigCivEvent::RemoveService {
                        game_host,
                        service_host,
                    }),
                    CivilisationAdminEvent::Public(PubConfigCivEvent::SetTime(timer)),
                ])
            }
        }
    }
}
