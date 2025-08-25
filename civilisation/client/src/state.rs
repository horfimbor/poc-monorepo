use civilisation_shared::dto::CivilisationDto;
use civilisation_shared::event::CivilisationEvent;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client::{EventStoreProps, LoadExternalComponent};
use horfimbor_client_derive::WebComponent;
use serde::Deserialize;
use yew::prelude::*;

type CivilisationState =
    EventStoreState<CivilisationDto, CivilisationEvent, CivilisationStateProps>;

#[derive(WebComponent)]
#[component(CivilisationState)]
#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct CivilisationStateProps {
    pub endpoint: String,
    pub jwt: String,
}

impl EventStoreProps for CivilisationStateProps {
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

impl AddEvent<CivilisationEvent, CivilisationStateProps> for CivilisationDto {
    fn play_event(&mut self, event: &CivilisationEvent) {
        self.play_event(event);
    }

    fn get_view(&self, props: CivilisationStateProps) -> Html {
        let nation_part = match self.nation() {
            None => {
                html! {
                    <div>
                        {"No nation name yet"}
                    </div>
                }
            }
            Some(nation) => {
                html! {
                    <div>
                        <b>{&nation.name}</b><p>{&nation.description}</p>
                    </div>
                }
            }
        };

        let world_part = html!(<>{
                        self.worlds().iter().map(|world|{
                            html!(<>
                                <fieldset>
                                    <LoadExternalComponent
                                        endpoint={"http://mono.localhost:8001"}
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
                <horfimbor-account-input
                    endpoint={props.endpoint().to_owned()}
                    jwt={props.jwt().to_owned()} />
                <hr/>
                {nation_part}
                <hr/>
                {world_part}
            </>
        }
    }
}
