use horfimbor_client::EventStoreProps;
use horfimbor_client::input::send_command;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client_derive::WebComponent;
use planet_shared::dto::PlanetDto;
use planet_shared::event::SharedPlanetEvent;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlElement};
use weblog::{console_debug, console_error, console_info, console_warn};
use yew::platform::spawn_local;
use yew::prelude::*;
use planet_shared::command::SharedPlanetCommand;

type PlanetState = EventStoreState<PlanetDto, SharedPlanetEvent, PlanetStateProps>;

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

impl AddEvent<SharedPlanetEvent, PlanetStateProps> for PlanetDto {
    fn play_event(&mut self, event: &SharedPlanetEvent) {
        self.play_event(event);
    }

    fn get_view(&self, props: PlanetStateProps) -> Html {
        let data = format!("{:?}", self);
        console_debug!(data);

        let on_create = Callback::from(move |_| {

            console_warn!("BUILD");
            // e.cancel_bubble();
            // let target: Option<EventTarget> = e.target();
            // let input = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());
            // if let Some(input) = input {
            //     console_info!(input.id());
            //     let props = props.clone();
            //     spawn_local(async move {
            //         let cmd = SharedPlanetCommand::StartConstruction {
            //             key: input.id().parse().unwrap(),
            //             time_config: None,
            //         };
            //         match send_command(&cmd, props.clone()).await {
            //             Ok(resp) => {
            //                 if resp.ok() {
            //                     console_info!("sent !");
            //                 }
            //             }
            //             Err(e) => {
            //                 console_error!(e);
            //             }
            //         }
            //     });
            // }
        });

        html! {
            <div>
                <table>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Cost"}</th>
                        <th>{"Running cost"}</th>
                        <th>{"Production"}</th>
                        <th>{"Action"}</th>
                    </tr>

                    { for self.available_building.iter().map(|(id, building)| html! {
                        <tr key={id.to_string()}>
                            <td>{&building.name}</td>
                            <td>
                                <ul>
                                { for building.construction.iter().map(|(resource, quantity)| html!{
                                    <li>{quantity}{" "}{resource}</li>
                                })}
                                <li>{building.construction_time}{" secondes"}</li>
                                </ul>
                            </td>
                            <td>
                                <ul>
                                { for building.running_cost.iter().map(|(resource, quantity)| html!{
                                    <li>{quantity}{" "}{resource}</li>
                                })}
                                </ul>
                            </td>
                            <td>
                                <ul>
                                { for building.production.iter().map(|(resource, production)| html!{
                                    <>
                                    <li>{production.quantity}{" "}{resource}</li>
                                    <li>{"( "}{production.stock}{" stock )"}</li>
                                    </>
                                })}
                                </ul>
                            </td>
                            <td>
                                <button id={id.to_string()} onclick={on_create.clone()}>{"build"}</button>
                            </td>
                        </tr>
                    }) }

                </table>
            </div>
        }
    }
}
