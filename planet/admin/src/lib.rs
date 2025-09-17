use horfimbor_eventsource::horfimbor_eventsource_derive::{Command, Event, StateNamed};
use horfimbor_eventsource::{Command, CommandName};
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
use horfimbor_eventsource::{Event, EventName};
use horfimbor_time::HfTimeConfiguration;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use url::Url;

pub const PLANET_CONFIG_STATE_NAME: &str = "PLANET_CONFIG_STATE";

#[derive(Command)]
#[state(PLANET_CONFIG_STATE_NAME)]
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, StateNamed, Default)]
#[state(PLANET_CONFIG_STATE_NAME)]
pub struct PlanetAdminState {
    time: Option<HfTimeConfiguration>,
    game_hosts: Option<Url>,
}

#[derive(Event)]
#[state(PLANET_CONFIG_STATE_NAME)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlanetAdminEvent {
    Setup(HfTimeConfiguration, Url),
}

impl Dto for PlanetAdminState {
    type Event = PlanetAdminEvent;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            PlanetAdminEvent::Setup(time, host) => {
                self.time = Some(*time);
                self.game_hosts = Some(host.clone());
            }
        }
    }
}

impl State for PlanetAdminState {
    type Command = PlanetAdminCommand;
    type Error = PlanetAdminError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            PlanetAdminCommand::Setup(time, host) => Ok(vec![PlanetAdminEvent::Setup(time, host)]),
        }
    }
}
