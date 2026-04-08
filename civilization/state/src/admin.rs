use civilization_shared::CIVILIZATION_CONFIG_STATE_NAME;
use civilization_shared::command::CivilizationAdminCommand;
use civilization_shared::dto::CivilizationAdminDto;
use civilization_shared::error::CivilizationAdminError;
use civilization_shared::event::{CivilizationAdminEvent, SharedCivilizationAdminEvent};
use horfimbor_eventsource::horfimbor_eventsource_derive::StateNamed;
use horfimbor_eventsource::{Dto, State, StateName, StateNamed};
use public_mono::civilization::PubCivilizationAdminEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default, StateNamed)]
#[state(CIVILIZATION_CONFIG_STATE_NAME)]
pub struct CivilizationAdminState {
    dto: CivilizationAdminDto,
}

impl CivilizationAdminState {
    #[must_use]
    pub fn dto(&self) -> &CivilizationAdminDto {
        &self.dto
    }
}

impl Dto for CivilizationAdminState {
    type Event = CivilizationAdminEvent;

    fn play_event(&mut self, event: &Self::Event) {
        self.dto.play_event(event);
    }
}

impl State for CivilizationAdminState {
    type Command = CivilizationAdminCommand;
    type Error = CivilizationAdminError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            CivilizationAdminCommand::CreateServer(host) => {
                if self.dto.host().is_some() {
                    return Err(CivilizationAdminError::AlreadyCreated);
                }

                Ok(vec![CivilizationAdminEvent::Shared(
                    SharedCivilizationAdminEvent::Created(host),
                )])
            }
            CivilizationAdminCommand::AddTime(timer) => {
                if self.dto.time().is_some() {
                    return Err(CivilizationAdminError::AlreadyHaveTime);
                }

                Ok(vec![CivilizationAdminEvent::Shared(
                    SharedCivilizationAdminEvent::TimeSet(timer),
                )])
            }
            CivilizationAdminCommand::AddService {
                name,
                comp: service,
            } => {
                let (Some(game_host), Some(time)) = (self.dto.host().clone(), self.dto.time())
                else {
                    return Err(CivilizationAdminError::NotCreatedYet);
                };

                if self.dto.game_services().contains_key(&name) {
                    return Err(CivilizationAdminError::ComponentNameAlreadyExists);
                }

                if self
                    .dto()
                    .game_services()
                    .values()
                    .find(|gc| gc.url == service.url)
                    .is_some()
                {
                    return Err(CivilizationAdminError::ComponentAlreadyExists);
                };

                Ok(vec![CivilizationAdminEvent::Public(
                    PubCivilizationAdminEvent::AddedService {
                        name,
                        game_host,
                        service_host: service.url,
                        balise: service.balise,
                        time,
                    },
                )])
            }
            CivilizationAdminCommand::RemoveService(name) => {
                let Some(game_host) = self.dto.host().clone() else {
                    return Err(CivilizationAdminError::NotCreatedYet);
                };

                if let Some(comp) = self.dto.game_services().get(&name) {
                    Ok(vec![CivilizationAdminEvent::Public(
                        PubCivilizationAdminEvent::RemovedService {
                            name,
                            game_host,
                            service_host: comp.url.clone(),
                        },
                    )])
                } else {
                    Err(CivilizationAdminError::ComponentAlreadyExists)
                }
            }
        }
    }
}
