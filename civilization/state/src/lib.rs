pub mod admin;

use civilization_shared::CIVILIZATION_STATE_NAME;
use civilization_shared::command::CivilizationCommand;
use civilization_shared::dto::CivilizationDto;
use civilization_shared::error::CivilizationError;
use civilization_shared::event::SharedCivilizationEvent;
use garde::Validate;
use horfimbor_eventsource::horfimbor_eventsource_derive::{Event, StateNamed};
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Dto, Event, EventName, State, StateName, StateNamed};
use public_mono::civilization::PubCivilizationEvent;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, StateNamed, Default)]
#[state(CIVILIZATION_STATE_NAME)]
pub struct CivilizationState {
    private_name: String,
    owner: ModelKey,
    game_host: Option<Url>,
    shared: CivilizationDto,
}

impl CivilizationState {
    #[must_use]
    pub fn private_name(&self) -> &str {
        &self.private_name
    }

    #[must_use]
    pub fn owner(&self) -> &ModelKey {
        &self.owner
    }

    #[must_use]
    pub fn shared(&self) -> &CivilizationDto {
        &self.shared
    }
}

#[derive(Event)]
#[state(CIVILIZATION_STATE_NAME)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PrvCivilizationEvent {
    NothingYet,
}

#[derive(Event)]
#[composite_state]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CivilizationEvent {
    Private(PrvCivilizationEvent),
    Shared(SharedCivilizationEvent),
    Public(PubCivilizationEvent),
}

impl Dto for CivilizationState {
    type Event = CivilizationEvent;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            CivilizationEvent::Shared(event) => {
                self.shared.play_event(event);
            }
            CivilizationEvent::Public(event) => match event {
                PubCivilizationEvent::Created {
                    game_host,
                    name,
                    owner,
                    ..
                } => {
                    self.game_host = Some(game_host.clone());
                    self.private_name = name.clone();
                    self.owner = owner.as_str().try_into().unwrap_or_default();
                }
            },
            CivilizationEvent::Private(_) => {}
        }
    }
}

impl State for CivilizationState {
    type Command = CivilizationCommand;
    type Error = CivilizationError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            CivilizationCommand::Create {
                name,
                user_id,
                owner,
                game_host,
                time,
            } => {
                let model: Result<ModelKey, _> = owner.as_str().try_into();

                if model.is_err() {
                    return Err(CivilizationError::InvalidOwner);
                }

                if !self.private_name.is_empty() {
                    return Err(CivilizationError::AlreadyCreated);
                }
                if name.is_empty() {
                    return Err(CivilizationError::AccountNameCannotBeEmpty);
                }

                Ok(vec![
                    CivilizationEvent::Public(PubCivilizationEvent::Created {
                        game_host,
                        name,
                        owner,
                        user_id,
                        time,
                    }),
                    CivilizationEvent::Shared(SharedCivilizationEvent::SetTime(time)),
                ])
            }
            CivilizationCommand::UpdateNation(nation) => {
                if let Err(e) = nation.validate() {
                    return Err(CivilizationError::InvalidNation(e.to_string()));
                }
                Ok(vec![CivilizationEvent::Shared(
                    SharedCivilizationEvent::NationUpdated(nation),
                )])
            }
            CivilizationCommand::AddWorld(world) => {
                if self.shared.worlds().iter().any(|w| w.id.eq(&world.id)) {
                    return Err(CivilizationError::WorldAlreadyAdded(world.id));
                }

                Ok(vec![CivilizationEvent::Shared(
                    SharedCivilizationEvent::WorldAdded(world),
                )])
            }
            CivilizationCommand::RemoveWorld(world_id) => {
                if !self.shared.worlds().iter().any(|w| w.id.eq(&world_id)) {
                    return Err(CivilizationError::WorldNotFound(world_id));
                }

                Ok(vec![CivilizationEvent::Shared(
                    SharedCivilizationEvent::WorldRemoved(world_id),
                )])
            }
        }
    }
}
