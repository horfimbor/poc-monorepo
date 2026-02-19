mod component;
mod timer;

use crate::admin::component::AddComponent;
use crate::admin::timer::UpdateTimer;
use civilisation_admin::{CivilisationAdminEvent, CivilisationAdminState};
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client::{EventStoreProps, LoadExternalComponent};
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

    fn get_view(&self, props: CivilisationAdminProps) -> Html {
        let timer = match self.time() {
            None => {
                html!(<p>
                    <UpdateTimer
                        endpoint={props.endpoint().to_owned()}
                        jwt={props.jwt().to_owned()} />
                    </p>)
            }
            Some(timer) => {
                html!(<>
                    {timer.start_time().unwrap_or_default().format("%+").to_string()}
                    <br/>
                    {timer.ig_length() / 60000} {" / "} {timer.irl_length() / 60000}
                    </>)
            }
        };

        let components = html!(
            <>
                <ul>
                {self.game_components().iter().map(|(name, comp)|{
                    html!(
                        <li key={name.as_str()}>
                        {name}

                        <fieldset>
                                    <LoadExternalComponent
                                        endpoint={comp.url.to_string()}
                                    balise={comp.tag.to_string()}
                                    jwt={props.jwt().to_owned()}
                                    id={""}
                                />
                                </fieldset>

                        </li>
                    )

                }).collect::<Html>()}
                </ul>
                <AddComponent
                    endpoint={props.endpoint().to_owned()}
                    jwt={props.jwt().to_owned()} />
            </>
        );
        html!(
            <>
                <p>
                    {self.host().clone().map(|h| h.to_string()).unwrap_or_default()}
                </p>
                <p>
                    {timer}
                </p>
                <p>
                    {components}
                </p>
            </>)
    }
}
