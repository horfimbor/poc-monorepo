use horfimbor_client::EventStoreProps;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client_derive::WebComponent;
use planet_shared::dto::PlanetDto;
use planet_shared::event::PlanetEvent;
use serde::Deserialize;
use yew::prelude::*;

type PlanetState = EventStoreState<PlanetDto, PlanetEvent, PlanetStateProps>;

#[derive(WebComponent)]
#[component(PlanetState)]
#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct PlanetStateProps {
    pub endpoint: String,
    pub jwt: String,
    pub id: String,
}

impl EventStoreProps for PlanetStateProps {
    fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    fn path(&self) -> &str {
        "api/planet"
    }

    fn jwt(&self) -> &str {
        self.jwt.as_str()
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }
}

impl AddEvent<PlanetEvent, PlanetStateProps> for PlanetDto {
    fn play_event(&mut self, event: &PlanetEvent) {
        self.play_event(event);
    }

    fn get_view(&self, props: PlanetStateProps) -> Html {
        html! {
            <div>
                {self.nb()}
                <horfimbor-planet-input
                        endpoint={props.endpoint().to_owned()}
                        jwt={props.jwt().to_owned()}
                        id={props.id().to_owned()}>
                </horfimbor-planet-input>
            </div>
        }
    }
}
