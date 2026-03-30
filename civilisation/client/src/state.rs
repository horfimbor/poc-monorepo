use crate::clock::Clock;
use crate::nation::NationDisplay;
use chrono::{Duration, NaiveDateTime};
use civilisation_shared::dto::CivilisationDto;
use civilisation_shared::event::SharedCivilisationEvent;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client::{EventStoreProps, LoadExternalComponent};
use horfimbor_client_derive::WebComponent;
use horfimbor_time::HfTimeConfiguration;
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

        let Ok(start) = NaiveDateTime::parse_from_str("2026-01-01T12:00", "%Y-%m-%dT%H:%M") else {
            return html! {
                <p>{"parse_error"}</p>
            };
        };

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
