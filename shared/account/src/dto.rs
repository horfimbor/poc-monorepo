#[cfg(feature = "server")]
use horfimbor_eventsource::Dto;

use crate::Nation;
use crate::event::{AccountEvent, PrvAccountEvent};
use common::Component;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct AccountDto {
    nation: Option<Nation>,
    worlds: Vec<String>,
}

impl AccountDto {
    pub fn play_event(&mut self, event: &AccountEvent) {
        match event {
            AccountEvent::Private(event) => match event {
                PrvAccountEvent::NationUpdated(nation) => {
                    self.nation = Some(nation.clone());
                }
                PrvAccountEvent::WorldAdded(world) => self.worlds.push(world.clone()),
                PrvAccountEvent::WorldRemoved(id) => todo!(), //self.worlds.retain(|w| !w.id.eq(id)),
            },
            AccountEvent::Public(_) => {}
        }
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

#[cfg(feature = "server")]
impl Dto for AccountDto {
    type Event = AccountEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}
