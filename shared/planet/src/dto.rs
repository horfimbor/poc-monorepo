#[cfg(feature = "server")]
use horfimbor_eventsource::Dto;

use crate::event::{PlanetEvent, PrvPlanetEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct PlanetDto {
    nb: u32,
}

impl PlanetDto {
    pub fn play_event(&mut self, event: &PlanetEvent) {
        match event {
            PlanetEvent::Private(event) => match event {
                PrvPlanetEvent::Pong => self.nb += 1,
                PrvPlanetEvent::Created => self.nb = 100,
            },
            PlanetEvent::Public(_) => {}
        }
    }

    #[must_use]
    pub fn nb(&self) -> u32 {
        self.nb
    }
}

#[cfg(feature = "server")]
impl Dto for PlanetDto {
    type Event = PlanetEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}
