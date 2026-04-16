mod add_service;
mod timer;

use crate::admin::add_service::AddService;
use crate::admin::timer::UpdateTimer;
use civilization_shared::command::CivilizationAdminCommand;
use civilization_shared::dto::CivilizationAdminDto;
use civilization_shared::event::CivilizationAdminEvent;
use horfimbor_client::input::send_command;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client::{EventStoreProps, LoadExternalComponent};
use horfimbor_client_derive::WebComponent;
use serde::Deserialize;
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
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

        let props = props.clone();
        let components = html!(
            <>
                <ul>
                {self.game_services().iter().map(|(name, comp)|{
                    let service_props = props.clone();
                    let service_name = name.clone();
                    let on_click_delete = Callback::from(move |_| {
                        let cmd = CivilizationAdminCommand::RemoveService(service_name.clone());
                        let spawn_props = service_props.clone();
                        spawn_local(async move {
                            match send_command(&cmd, spawn_props.clone()).await {
                                Ok(resp) => {
                                    if resp.ok() {
                                        console_info!("Sent !");
                                    }
                                }
                                Err(e) => {
                                    console_error!(e);
                                }
                            }
                        });
                    });

                    html!(
                        <li key={name.as_str()}>
                        {name} <button onclick={on_click_delete}>{"remove"}</button>

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
                <p>
                    <AddService
                    endpoint={props.endpoint().to_owned()}
                    jwt={props.jwt().to_owned()} />
                </p>
            </>)
    }
}
