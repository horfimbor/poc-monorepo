use crate::state::PlanetStateProps;
use gloo_timers::callback::Timeout;
use horfimbor_client::input::send_command;
use planet_shared::command::SharedPlanetCommand;
use planet_shared::dto::Building;
use std::collections::HashMap;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlElement, MouseEvent};
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
use yew::{Callback, Html, Properties, function_component, html, use_state};

#[derive(Properties, PartialEq)]
pub struct AvailableBuildingProps {
    pub state_props: PlanetStateProps,
    pub available_building: HashMap<Uuid, Building>,
}

#[function_component(AvailableBuilding)]
pub fn draw_available_buildings(props: &AvailableBuildingProps) -> Html {
    let create_error = use_state(|| None::<String>);

    if create_error.is_some() {
        let timer = create_error.clone();
        let timeout = Timeout::new(1000, move || {
            timer.set(None);
        });
        timeout.forget();
    }

    let create_error_setter = create_error.clone();

    let state_props = props.state_props.clone();

    let on_create = Callback::from(move |e: MouseEvent| {
        let create_error_setter = create_error_setter.clone();
        e.cancel_bubble();
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());
        if let Some(input) = input {
            let Ok(key) = input.id().parse::<Uuid>() else {
                console_error!("cannot parse uuid");
                return;
            };
            let state_props = state_props.clone();
            spawn_local(async move {
                let create_error_setter = create_error_setter.clone();
                let cmd = SharedPlanetCommand::StartConstruction { key };
                match send_command(&cmd, state_props.clone()).await {
                    Ok(resp) => {
                        if resp.ok() {
                            console_info!("sent !");
                        } else {
                            let log = format!(
                                "{} - {}: {:?}",
                                resp.status(),
                                resp.status_text(),
                                resp.body().map(|b| b.as_string()).unwrap_or_default()
                            );
                            create_error_setter.set(Some(log));
                        }
                    }
                    Err(e) => {
                        console_error!(e);
                    }
                }
            });
        }
    });

    html!(
        <>
            <h3>{"Available Building"}</h3>
            <table>
                <tr>
                    <th>{"Name"}</th>
                    <th>{"Cost"}</th>
                    <th>{"Running cost"}</th>
                    <th>{"Production"}</th>
                    <th>{"Action"}</th>
                </tr>

                { for props.available_building.iter().map(|(id, building)| html! {
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
                            if let Some(create_error) = create_error.as_ref() {
                                <span style="color:red">{create_error}</span>
                            }else{
                                <button id={id.to_string()} onclick={on_create.clone()}>{"build"}</button>
                            }
                        </td>
                    </tr>
                }) }
            </table>
        </>
    )
}
