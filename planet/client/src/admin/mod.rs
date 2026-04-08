use horfimbor_client::EventStoreProps;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client_derive::WebComponent;
use planet_shared::dto_admin::PlanetAdminDto;
use planet_shared::event::SharedPlanetAdminEvent;
use serde::Deserialize;
use yew::prelude::*;

type PlanetAdmin = EventStoreState<PlanetAdminDto, SharedPlanetAdminEvent, PlanetAdminProps>;

#[derive(WebComponent)]
#[component(PlanetAdmin)]
#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct PlanetAdminProps {
    pub endpoint: AttrValue,
    pub jwt: AttrValue,
}

impl EventStoreProps for PlanetAdminProps {
    fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    fn path(&self) -> &str {
        "api/planet-admin"
    }

    fn jwt(&self) -> &str {
        self.jwt.as_str()
    }

    fn id(&self) -> &str {
        ""
    }
}

impl AddEvent<SharedPlanetAdminEvent, PlanetAdminProps> for PlanetAdminDto {
    fn play_event(&mut self, event: &SharedPlanetAdminEvent) {
        self.play_event(event);
    }

    fn get_view(&self, _props: PlanetAdminProps) -> Html {
        html!(
            <>
                <p>
                    {"view planet admin"}
                </p>
            </>)
    }
}
