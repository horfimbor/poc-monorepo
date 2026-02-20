#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::{Command, Event, StateNamed};
#[cfg(feature = "server")]
use horfimbor_eventsource::{Command, CommandName};
#[cfg(feature = "server")]
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};
use horfimbor_time::HfTimeConfiguration;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use url::Url;

pub const PLANET_CONFIG_STATE_NAME: &str = "PLANET_CONFIG_STATE";

#[cfg_attr(feature = "server", derive(Command))]
#[cfg_attr(feature = "server", state(PLANET_CONFIG_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlanetAdminCommand {
    Setup(HfTimeConfiguration, Url),
}

#[derive(Error, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlanetAdminError {
    SomeError,
}

impl Display for PlanetAdminError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SomeError => {
                write!(f, "some error found")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "server", derive(StateNamed))]
#[cfg_attr(feature = "server", state(PLANET_CONFIG_STATE_NAME))]
pub struct PlanetAdminState {
    time: Option<HfTimeConfiguration>,
    game_hosts: Option<Url>,
}

impl PlanetAdminState {
    pub fn time(&self) -> Option<HfTimeConfiguration> {
        self.time
    }

    pub fn game_hosts(&self) -> &Option<Url> {
        &self.game_hosts
    }

    pub fn play_event(&mut self, event: &PlanetAdminEvent) {
        match event {
            PlanetAdminEvent::Setup(time, host) => {
                self.time = Some(*time);
                self.game_hosts = Some(host.clone());
            }
        }
    }
}

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(PLANET_CONFIG_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlanetAdminEvent {
    Setup(HfTimeConfiguration, Url),
}

#[cfg(feature = "server")]
impl Dto for PlanetAdminState {
    type Event = PlanetAdminEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}

#[cfg(feature = "server")]
impl State for PlanetAdminState {
    type Command = PlanetAdminCommand;
    type Error = PlanetAdminError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            PlanetAdminCommand::Setup(time, host) => Ok(vec![PlanetAdminEvent::Setup(time, host)]),
        }
    }
}
