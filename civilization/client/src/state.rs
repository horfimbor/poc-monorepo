use crate::clock::Clock;
use crate::nation::NationDisplay;
use civilization_shared::dto::CivilizationDto;
use civilization_shared::event::SharedCivilizationEvent;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client::{EventStoreProps, LoadExternalComponent};
use horfimbor_client_derive::WebComponent;
use serde::Deserialize;
use yew::prelude::*;

type CivilizationState =
    EventStoreState<CivilizationDto, SharedCivilizationEvent, CivilizationProps>;

#[derive(WebComponent)]
#[component(CivilizationState)]
#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct CivilizationProps {
    pub endpoint: AttrValue,
    pub jwt: AttrValue,
}

impl EventStoreProps for CivilizationProps {
    fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    fn path(&self) -> &str {
        "api/civilization"
    }

    fn jwt(&self) -> &str {
        self.jwt.as_str()
    }

    fn id(&self) -> &str {
        ""
    }
}

impl AddEvent<SharedCivilizationEvent, CivilizationProps> for CivilizationDto {
    fn play_event(&mut self, event: &SharedCivilizationEvent) {
        self.play_event(event);
    }

    fn get_view(&self, props: CivilizationProps) -> Html {
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
                if let Some(timer) = self.time() {
                    <Clock config={timer}/>
                }else{
                    <p>{"no time yet"}</p>
                }

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
