use crate::nation::NationDisplay;
use civilisation_shared::dto::CivilisationDto;
use civilisation_shared::event::SharedCivilisationEvent;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client::{EventStoreProps, LoadExternalComponent};
use horfimbor_client_derive::WebComponent;
use serde::Deserialize;
use yew::prelude::*;

type CivilisationState =
    EventStoreState<CivilisationDto, SharedCivilisationEvent, CivilisationProps>;

#[derive(WebComponent)]
#[component(CivilisationState)]
#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct CivilisationProps {
    pub endpoint: AttrValue,
    pub jwt: AttrValue,
}

impl EventStoreProps for CivilisationProps {
    fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    fn path(&self) -> &str {
        "api/civilisation"
    }

    fn jwt(&self) -> &str {
        self.jwt.as_str()
    }

    fn id(&self) -> &str {
        ""
    }
}

impl AddEvent<SharedCivilisationEvent, CivilisationProps> for CivilisationDto {
    fn play_event(&mut self, event: &SharedCivilisationEvent) {
        self.play_event(event);
    }

    fn get_view(&self, props: CivilisationProps) -> Html {
        let world_part = html!(
            <>{
                self.worlds().iter().map(|world|{
                    html!(
                    <>
                        <fieldset>
                            <LoadExternalComponent
                                endpoint={world.endpoint.clone()}
                            balise={world.balise.clone()}
                            jwt={props.jwt().to_owned()}
                            id={world.id.clone()}
                        />
                        </fieldset>
                    </>
                    )
                }).collect::<Html>()

            }</>);

        html! {
            <>
                <NationDisplay
                    endpoint={props.endpoint}
                    jwt={props.jwt.clone()}
                    nation={self.nation().clone()} />
                <hr/>
                {world_part}
            </>
        }
    }
}
