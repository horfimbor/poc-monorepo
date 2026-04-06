#[cfg(feature = "server")]
use crate::PLANET_STATE_NAME;
#[cfg(feature = "server")]
use horfimbor_eventsource::horfimbor_eventsource_derive::Command;
#[cfg(feature = "server")]
use horfimbor_eventsource::{Command, CommandName};
use horfimbor_time::HfTimeConfiguration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg_attr(feature = "server", derive(Command))]
#[cfg_attr(feature = "server", state(PLANET_STATE_NAME))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SharedPlanetCommand {
    Create {
        account_id: String,
        admin_id: String,
        time: HfTimeConfiguration,
    },
    ChangeOwner {
        account_id: String,
    },
    StartConstruction {
        key: Uuid,
    },
    CancelConstruction {
        key: Uuid,
    },
    DestroyConstruction {
        key: Uuid,
    },
}
