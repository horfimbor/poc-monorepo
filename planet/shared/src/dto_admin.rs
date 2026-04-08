use crate::event::SharedPlanetAdminEvent;
#[cfg(feature = "server")]
use horfimbor_eventsource::Dto;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct PlanetAdminDto {
    nb_planet: u8,
}

impl PlanetAdminDto {
    pub fn play_event(&mut self, event: &SharedPlanetAdminEvent) {
        match event {
            SharedPlanetAdminEvent::NbPlanetUpdated(nb) => {
                self.nb_planet = *nb;
            }
        }
    }

    pub fn nb_planet(&self) -> u8 {
        self.nb_planet
    }
}

#[cfg(feature = "server")]
impl Dto for PlanetAdminDto {
    type Event = SharedPlanetAdminEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.play_event(event);
    }
}
