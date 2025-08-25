#[cfg(feature = "server")]
use horfimbor_eventsource::Dto;

use crate::Nation;
use crate::event::{CivilisationEvent, PrvCivilisationEvent};
use public_mono::Component;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct CivilisationDto {
    nation: Option<Nation>,
    worlds: Vec<Component>,
}

impl CivilisationDto {
    pub fn play_event(&mut self, event: &CivilisationEvent) {
        match event {
            CivilisationEvent::Private(event) => match event {
                PrvCivilisationEvent::NationUpdated(nation) => {
                    self.nation = Some(nation.clone());
                }
                PrvCivilisationEvent::WorldAdded(world) => self.worlds.push(world.clone()),
                PrvCivilisationEvent::WorldRemoved(id) => self.worlds.retain(|w| !w.id.eq(id)),
            },
            CivilisationEvent::Public(_) => {}
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
    type Event = CivilisationEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}
