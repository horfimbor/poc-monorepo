use account_shared::command::AccountCommand;
use account_shared::error::AccountError;
use account_shared::event::{AccountEvent, PrvAccountEvent};
use account_shared::{ACCOUNT_STATE_NAME, Nation};
use common::Component;
use common::account::PubAccountEvent;
use horfimbor_eventsource::horfimbor_eventsource_derive::StateNamed;
use horfimbor_eventsource::model_key::ModelKey;
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, StateNamed, Default)]
#[state(ACCOUNT_STATE_NAME)]
pub struct AccountState {
    private_name: String,
    owner: ModelKey,
    nation: Option<Nation>,
    worlds: Vec<String>,
}

impl AccountState {
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
    pub fn worlds(&self) -> &Vec<String> {
        &self.worlds
    }
}

impl Dto for AccountState {
    type Event = AccountEvent;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            AccountEvent::Private(event) => match event {
                PrvAccountEvent::NationUpdated(nation) => {
                    self.nation = Some(nation.clone());
                }
                PrvAccountEvent::WorldAdded(world) => self.worlds.push(world.clone()),
                PrvAccountEvent::WorldRemoved(id) => todo!(), // self.worlds.retain(|w| !w.id.eq(id)),
            },
            AccountEvent::Public(event) => match event {
                PubAccountEvent::Created { name, owner } => {
                    self.private_name = name.clone();
                    self.owner = owner.as_str().try_into().unwrap_or_default();
                }
            },
        }
    }
}

impl State for AccountState {
    type Command = AccountCommand;
    type Error = AccountError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            AccountCommand::Create { name, owner } => {
                let model: Result<ModelKey, _> = owner.as_str().try_into();

                if model.is_err() {
                    return Err(AccountError::InvalidOwner);
                }

                if !self.private_name.is_empty() {
                    return Err(AccountError::AlreadyCreated);
                }
                if name.is_empty() {
                    return Err(AccountError::AccountNameCannotBeEmpty);
                }
                Ok(vec![AccountEvent::Public(PubAccountEvent::Created {
                    name,
                    owner,
                })])
            }
            AccountCommand::UpdateNation(nation) => {
                if nation.name.is_empty() {
                    return Err(AccountError::NationNameCannotBeEmpty);
                }
                Ok(vec![AccountEvent::Private(PrvAccountEvent::NationUpdated(
                    nation,
                ))])
            }
            AccountCommand::AddWorld(world) => {
                // if self.worlds.iter().any(|w| w.id.eq(&world.id)) {
                //     return Err(AccountError::WorldAlreadyAdded(world.id));
                // }

                Ok(vec![AccountEvent::Private(PrvAccountEvent::WorldAdded(
                    world.id,
                ))])
            }
            AccountCommand::RemoveWorld(world_id) => {
                // if !self.worlds.iter().any(|w| w.id.eq(&world_id)) {
                //     return Err(AccountError::WorldNotFound(world_id));
                // }

                Ok(vec![AccountEvent::Private(PrvAccountEvent::WorldRemoved(
                    world_id,
                ))])
            }
        }
    }
}
