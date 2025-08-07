use common::planet::PubPlanetEvent;
use horfimbor_eventsource::horfimbor_eventsource_derive::StateNamed;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
use planet_shared::PLANET_STATE_NAME;
use planet_shared::command::PlanetCommand;
use planet_shared::error::PlanetError;
use planet_shared::event::{PlanetEvent, PrvPlanetEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, StateNamed, Default)]
#[state(PLANET_STATE_NAME)]
pub struct PlanetState {
    owner: ModelKey,
    nb: u32,
}

impl PlanetState {
    #[must_use]
    pub fn owner(&self) -> &ModelKey {
        &self.owner
    }

    #[must_use]
    pub fn nb(&self) -> u32 {
        self.nb
    }
}

impl Dto for PlanetState {
    type Event = PlanetEvent;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            PlanetEvent::Private(event) => match event {
                PrvPlanetEvent::Pong(_) => self.nb += 1,
                PrvPlanetEvent::Created(_) => self.nb = 100,
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
            PlanetCommand::Create { account_id } => {
                let model: Result<ModelKey, _> = account_id.as_str().try_into();

                if model.is_err() {
                    return Err(PlanetError::InvalidOwner);
                }

                Ok(vec![
                    PlanetEvent::Private(PrvPlanetEvent::Created(0)),
                    PlanetEvent::Public(PubPlanetEvent::NewOwner {
                        old_account_id: None,
                        account_id,
                    }),
                ])
            }

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
            PlanetCommand::Ping => Ok(vec![PlanetEvent::Private(PrvPlanetEvent::Pong(0))]),
        }
    }
}
