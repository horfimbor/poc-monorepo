#[cfg(feature = "server")]
use horfimbor_eventsource::Dto;

use crate::event::SharedPlanetEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default, Eq)]
pub struct PlanetDto {
    nb: usize,
}

impl PlanetDto {
    pub fn play_event(&mut self, event: &SharedPlanetEvent) {
        match event {
            SharedPlanetEvent::Pong(_) => self.nb += 1,
            SharedPlanetEvent::Created(_) => self.nb = 100,
            SharedPlanetEvent::Boom(nb) => self.nb = *nb,
        }
    }

    #[must_use]
    pub fn nb(&self) -> usize {
        self.nb
    }
}

#[cfg(feature = "server")]
impl Dto for PlanetDto {
    type Event = SharedPlanetEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}
