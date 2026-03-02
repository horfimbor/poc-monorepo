use horfimbor_client::EventStoreProps;
use horfimbor_client::input::send_command;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client_derive::WebComponent;
use planet_shared::command::SharedPlanetCommand;
use planet_shared::dto::PlanetDto;
use planet_shared::event::SharedPlanetEvent;
use serde::Deserialize;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlElement};
use weblog::{console_debug, console_error, console_info};
use yew::platform::spawn_local;
use yew::prelude::*;

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

        let on_create = Callback::from(move |e: MouseEvent| {
            e.cancel_bubble();
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());
            if let Some(input) = input {
                let Ok(key) = input.id().parse::<Uuid>() else {
                    console_error!("cannot parse uuid");
                    return;
                };
                let props = props.clone();
                spawn_local(async move {
                    let cmd = SharedPlanetCommand::StartConstruction {
                        key,
                        time_config: None,
                    };
                    match send_command(&cmd, props.clone()).await {
                        Ok(resp) => {
                            if resp.ok() {
                                console_info!("sent !");
                            }
                        }
                        Err(e) => {
                            console_error!(e);
                        }
                    }
                });
            }
        });

        html! {
            <div>
                <h3>{"Available Building"}</h3>
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
                <h3>{"Construction"}</h3>
                <table>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Cost"}</th>
                        <th>{"Running cost"}</th>
                        <th>{"Production"}</th>
                        <th>{"Action"}</th>
                    </tr>

                    { for self.construction.iter().map(|(id, building)| html! {
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
                                {"cancel TODO"}
                                // <button id={id.to_string()} onclick={on_cancel.clone()}>{"build"}</button>
                            </td>
                        </tr>
                    }) }

                </table>
            <h3>{"Building"}</h3>
                <table>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Cost"}</th>
                        <th>{"Running cost"}</th>
                        <th>{"Production"}</th>
                        <th>{"Action"}</th>
                    </tr>

                    { for self.buildings.iter().map(|(id, building)| html! {
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
                                {"delete TODO"}
                                // <button id={id.to_string()} onclick={on_create.clone()}>{"build"}</button>
                            </td>
                        </tr>
                    }) }
                </table>
            </div>
        }
    }
}
