#[cfg(feature = "server")]
use crate::PLANET_STATE_NAME;
use crate::dto::{Building, Resource, ResourceCalc};
#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Event;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Event, EventName};
use horfimbor_time::HfTimeConfiguration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg_attr(feature = "server", derive(Event))]
#[cfg_attr(feature = "server", state(PLANET_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SharedPlanetEvent {
    UpdateResource {
        resource: Resource,
        calc: ResourceCalc,
    },
    UpdateAvailableBuilding {
        key: Uuid,
        building: Building,
    },
    RemoveAvailableBuilding {
        key: Uuid,
    },
    UpdateConstruction {
        key: Uuid,
        building: Building,
    },
    RemoveConstruction {
        key: Uuid,
    },
    UpdateRunningBuilding {
        key: Uuid,
        building: Building,
    },
    RemoveRunningBuilding {
        key: Uuid,
    },
}
