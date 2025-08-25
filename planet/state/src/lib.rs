use horfimbor_eventsource::horfimbor_eventsource_derive::{Event, StateNamed};
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
use horfimbor_eventsource::{Event, EventName};
use planet_shared::PLANET_STATE_NAME;
use planet_shared::command::PlanetCommand;
use planet_shared::dto::PlanetDto;
use planet_shared::error::PlanetError;
use planet_shared::event::SharedPlanetEvent;
use public_mono::planet::PubPlanetEvent;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

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

    #[must_use]
    pub fn nb(&self) -> u64 {
        self.shared.nb() as u64
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
                match event {
                    SharedPlanetEvent::Created(_) => {
                        self.countdown = 25;
                    }
                    SharedPlanetEvent::Pong(_) => {}
                    SharedPlanetEvent::Boom(_) => {
                        self.countdown = 100;
                    }
                }
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
            PlanetCommand::Create { account_id } => {
                let model: Result<ModelKey, _> = account_id.as_str().try_into();

                if model.is_err() {
                    return Err(PlanetError::InvalidOwner);
                }

                Ok(vec![
                    PlanetEvent::Shared(SharedPlanetEvent::Created(0)),
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
            PlanetCommand::Ping => {
                let mut res = vec![PlanetEvent::Shared(SharedPlanetEvent::Pong(0))];
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                if timestamp % 2 == self.nb() % 2 {
                    if self.countdown == 1 {
                        res.push(PlanetEvent::Shared(SharedPlanetEvent::Boom(0)))
                    } else {
                        res.push(PlanetEvent::Private(PrvPlanetEvent::LowerCountDown(0)))
                    }
                }
                Ok(res)
            }
        }
    }
}
