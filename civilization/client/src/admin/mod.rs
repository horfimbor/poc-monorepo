mod component;
mod timer;

use crate::admin::component::AddService;
use crate::admin::timer::UpdateTimer;
use civilization_shared::dto::CivilizationAdminDto;
use civilization_shared::event::CivilizationAdminEvent;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client::{EventStoreProps, LoadExternalComponent};
use horfimbor_client_derive::WebComponent;
use serde::Deserialize;
use yew::prelude::*;

type CivilizationAdmin =
    EventStoreState<CivilizationAdminDto, CivilizationAdminEvent, CivilizationAdminProps>;

#[derive(WebComponent)]
#[component(CivilizationAdmin)]
#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct CivilizationAdminProps {
    pub endpoint: AttrValue,
    pub jwt: AttrValue,
}

impl EventStoreProps for CivilizationAdminProps {
    fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    fn path(&self) -> &str {
        "api/civilization-admin"
    }

    fn jwt(&self) -> &str {
        self.jwt.as_str()
    }

    fn id(&self) -> &str {
        ""
    }
}

impl AddEvent<CivilizationAdminEvent, CivilizationAdminProps> for CivilizationAdminDto {
    fn play_event(&mut self, event: &CivilizationAdminEvent) {
        self.play_event(event);
    }

    fn get_view(&self, props: CivilizationAdminProps) -> Html {
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
                {self.game_services().iter().map(|(name, comp)|{
                    html!(
                        <li key={name.as_str()}>
                        {name}

                        <fieldset>
                                    <LoadExternalComponent
                                        endpoint={comp.url.to_string()}
                                    balise={comp.balise.to_string()}
                                    jwt={props.jwt().to_owned()}
                                    id={""}
                                />
                                </fieldset>

                        </li>
                    )

                }).collect::<Html>()}
                </ul>
                <AddService
                    endpoint={props.endpoint().to_owned()}
                    jwt={props.jwt().to_owned()} />
            </>
        );

        html!(
            <>

               <ToggleAdmin >
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
                </>
               </ToggleAdmin>
            </>)
    }
}

#[derive(Properties, PartialEq)]
pub struct ToggleAdminProps {
    pub children: Html,
}
#[component]
fn ToggleAdmin(props: &ToggleAdminProps) -> Html {
    let admin_open = use_state(|| false);

    let onclick = {
        let admin_open = admin_open.clone();
        Callback::from(move |_| admin_open.set(!*admin_open))
    };

    html! {
        <fieldset>
             <button onclick={onclick}>{"toggle admin"}</button>
            if *admin_open {
                  {props.children.clone()}
            }
        </fieldset>
    }
}
