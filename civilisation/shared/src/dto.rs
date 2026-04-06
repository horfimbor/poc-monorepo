use crate::Nation;
use crate::event::SharedCivilisationEvent;
#[cfg(feature = "server")]
use horfimbor_eventsource::Dto;
use horfimbor_time::HfTimeConfiguration;
use public_mono::Component;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default, Eq)]
pub struct CivilisationDto {
    nation: Option<Nation>,
    worlds: Vec<Component>,
    time: Option<HfTimeConfiguration>,
}

impl CivilisationDto {
    pub fn play_event(&mut self, event: &SharedCivilisationEvent) {
        match event {
            SharedCivilisationEvent::NationUpdated(nation) => {
                self.nation = Some(nation.clone());
            }
            SharedCivilisationEvent::WorldAdded(world) => self.worlds.push(world.clone()),
            SharedCivilisationEvent::WorldRemoved(id) => self.worlds.retain(|w| !w.id.eq(id)),
            SharedCivilisationEvent::SetTime(config) => self.time = Some(*config),
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

    #[must_use]
    pub fn time(&self) -> Option<HfTimeConfiguration> {
        self.time
    }
}

#[cfg(feature = "server")]
impl Dto for CivilisationDto {
    type Event = SharedCivilisationEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}
