use horfimbor_client::EventStoreProps;
use horfimbor_client::input::send_command;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client_derive::WebComponent;
use planet_shared::command::SharedPlanetAdminCommand;
use planet_shared::dto_admin::PlanetAdminDto;
use planet_shared::event::SharedPlanetAdminEvent;
use serde::Deserialize;
use web_sys::HtmlInputElement;
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
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

    fn get_view(&self, props: PlanetAdminProps) -> Html {
        let on_change = Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let Ok(nb) = input.value().parse() else {
                console_error!("cannot parse input");
                return;
            };

            let cmd = SharedPlanetAdminCommand::UpdateNbStartPlanet(nb);
            let spawn_props = props.clone();
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
            <>
                <p>
                    {"Number of starting planet : "}{self.nb_planet().to_string()} <br/>
                    <input type="number" oninput={on_change} />
                </p>
            </>
        )
    }
}
