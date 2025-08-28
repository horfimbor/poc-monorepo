#[cfg(feature = "server")]
use horfimbor_eventsource::Dto;

use crate::Nation;
use crate::event::{ SharedCivilisationEvent};
use public_mono::Component;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default, Eq)]
pub struct CivilisationDto {
    nation: Option<Nation>,
    worlds: Vec<Component>,
}

impl CivilisationDto {
    pub fn play_event(&mut self, event: &SharedCivilisationEvent) {
        match event {
            SharedCivilisationEvent::NationUpdated(nation) => {
                self.nation = Some(nation.clone());
            }
            SharedCivilisationEvent::WorldAdded(world) => self.worlds.push(world.clone()),
            SharedCivilisationEvent::WorldRemoved(id) => self.worlds.retain(|w| !w.id.eq(id)),
        }
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

#[cfg(feature = "server")]
impl Dto for CivilisationDto {
    type Event = SharedCivilisationEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}
