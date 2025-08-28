use civilisation_admin::{CivilisationAdminEvent, CivilisationAdminState};
use horfimbor_client::EventStoreProps;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client_derive::WebComponent;
use serde::Deserialize;
use yew::prelude::*;

type CivilisationAdmin =
    EventStoreState<CivilisationAdminState, CivilisationAdminEvent, CivilisationAdminProps>;

#[derive(WebComponent)]
#[component(CivilisationAdmin)]
#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct CivilisationAdminProps {
    pub endpoint: String,
    pub jwt: String,
}

impl EventStoreProps for CivilisationAdminProps {
    fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    fn path(&self) -> &str {
        "api/civilisation/admin"
    }

    fn jwt(&self) -> &str {
        self.jwt.as_str()
    }

    fn id(&self) -> &str {
        ""
    }
}

impl AddEvent<CivilisationAdminEvent, CivilisationAdminProps> for CivilisationAdminState {
    fn play_event(&mut self, event: &CivilisationAdminEvent) {
        self.play_event(event);
    }

    fn get_view(&self, _props: CivilisationAdminProps) -> Html {
        html!(<p>
            {self.host().clone().map(|h| h.to_string())}
            </p>)
    }
}
