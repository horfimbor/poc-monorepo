use civilisation_shared::command::CivilisationCommand;
use civilisation_shared::error::CivilisationError;
use civilisation_shared::event::{CivilisationEvent, PrvCivilisationEvent};
use civilisation_shared::{CIVILISATION_STATE_NAME, Nation};
use garde::Validate;
use horfimbor_eventsource::horfimbor_eventsource_derive::StateNamed;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
use public_mono::Component;
use public_mono::civilisation::PubCivilisationEvent;
use serde::{Deserialize, Serialize};
use url::Host;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, StateNamed)]
#[state(CIVILISATION_STATE_NAME)]
pub struct CivilisationState {
    private_name: String,
    owner: ModelKey,
    game_host: Host,
    nation: Option<Nation>,
    worlds: Vec<Component>,
}

impl Default for CivilisationState {
    fn default() -> Self {
        Self {
            private_name: Default::default(),
            owner: Default::default(),
            game_host: Host::Domain("localhost".to_string()),
            nation: None,
            worlds: vec![],
        }
    }
}

impl CivilisationState {
    #[must_use]
    pub fn private_name(&self) -> &str {
        &self.private_name
    }

    #[must_use]
    pub fn owner(&self) -> &ModelKey {
        &self.owner
    }

    #[must_use]
    pub fn nation(&self) -> &Option<Nation> {
        &self.nation
    }

    #[must_use]
    pub fn worlds(&self) -> &Vec<Component> {
        &self.worlds
    }
}

impl Dto for CivilisationState {
    type Event = CivilisationEvent;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            CivilisationEvent::Private(event) => match event {
                PrvCivilisationEvent::NationUpdated(nation) => {
                    self.nation = Some(nation.clone());
                }
                PrvCivilisationEvent::WorldAdded(world) => self.worlds.push(world.clone()),
                PrvCivilisationEvent::WorldRemoved(id) => self.worlds.retain(|w| !w.id.eq(id)),
            },
            CivilisationEvent::Public(event) => match event {
                PubCivilisationEvent::Created {
                    game_host,
                    name,
                    owner,
                } => {
                    self.game_host = game_host.clone();
                    self.private_name = name.clone();
                    self.owner = owner.as_str().try_into().unwrap_or_default();
                }
            },
        }
    }
}

impl State for CivilisationState {
    type Command = CivilisationCommand;
    type Error = CivilisationError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            CivilisationCommand::Create { name, owner } => {
                let model: Result<ModelKey, _> = owner.as_str().try_into();

                if model.is_err() {
                    return Err(CivilisationError::InvalidOwner);
                }

                if !self.private_name.is_empty() {
                    return Err(CivilisationError::AlreadyCreated);
                }
                if name.is_empty() {
                    return Err(CivilisationError::AccountNameCannotBeEmpty);
                }
                Ok(vec![CivilisationEvent::Public(
                    PubCivilisationEvent::Created {
                        game_host: self.game_host.clone(),
                        name,
                        owner,
                    },
                )])
            }
            CivilisationCommand::UpdateNation(nation) => {
                if let Err(e) = nation.validate() {
                    return Err(CivilisationError::InvalidNation(e.to_string()));
                }
                Ok(vec![CivilisationEvent::Private(
                    PrvCivilisationEvent::NationUpdated(nation),
                )])
            }
            CivilisationCommand::AddWorld(world) => {
                if self.worlds.iter().any(|w| w.id.eq(&world.id)) {
                    return Err(CivilisationError::WorldAlreadyAdded(world.id));
                }

                Ok(vec![CivilisationEvent::Private(
                    PrvCivilisationEvent::WorldAdded(world),
                )])
            }
            CivilisationCommand::RemoveWorld(world_id) => {
                if !self.worlds.iter().any(|w| w.id.eq(&world_id)) {
                    return Err(CivilisationError::WorldNotFound(world_id));
                }

                Ok(vec![CivilisationEvent::Private(
                    PrvCivilisationEvent::WorldRemoved(world_id),
                )])
            }
        }
    }
}
