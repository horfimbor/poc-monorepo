use serde::{Deserialize, Serialize};
use planet_shared::PLANET_CONFIG_STATE_NAME;
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
use planet_shared::dto_admin::PlanetAdminDto;
use planet_shared::event::SharedPlanetAdminEvent;
use horfimbor_eventsource::{Command, CommandName, Event, EventName};
use horfimbor_eventsource::horfimbor_eventsource_derive::{Command, Event, StateNamed};
use planet_shared::command::SharedPlanetAdminCommand;
use planet_shared::error::PlanetAdminError;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, StateNamed, Default)]
#[state(PLANET_CONFIG_STATE_NAME)]
pub struct PlanetAdminState {
    dto: PlanetAdminDto
}

impl PlanetAdminState {
    pub fn dto(&self) -> &PlanetAdminDto {
        &self.dto
    }
}

#[derive(Event)]
#[composite_state]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PlanetAdminEvent {
    Shared(SharedPlanetAdminEvent),
}

#[derive(Command)]
#[state(PLANET_CONFIG_STATE_NAME)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PlanetAdminCommand {
    Shared(SharedPlanetAdminCommand),
}


impl Dto for PlanetAdminState {
    type Event = PlanetAdminEvent;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            PlanetAdminEvent::Shared(event) => {
                self.dto.play_event(event);
            }
        }
    }
}

impl State for PlanetAdminState{
    type Command = PlanetAdminCommand;
    type Error = PlanetAdminError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            PlanetAdminCommand::Shared(command) => {
                match command {
                    SharedPlanetAdminCommand::UpdateNbStartPlanet(nb) => {
                        Ok(vec![PlanetAdminEvent::Shared(SharedPlanetAdminEvent::NbPlanetUpdated(nb))])
                    }
                    SharedPlanetAdminCommand::Create => {
                        Ok(vec![PlanetAdminEvent::Shared(SharedPlanetAdminEvent::NbPlanetUpdated(1))])
                    }
                }
            }
        }
    }
}