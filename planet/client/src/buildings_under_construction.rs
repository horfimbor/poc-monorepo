use crate::state::PlanetStateProps;
use chrono::{DateTime, Utc};
use gloo_timers::callback::Timeout;
use horfimbor_client::input::send_command;
use horfimbor_time::{HfTime, HfTimeConfiguration};
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
pub struct UnderConstructionBuildingProps {
    pub state_props: PlanetStateProps,
    pub under_construction_building: HashMap<Uuid, (Building, DateTime<Utc>)>,
    pub time_config: HfTimeConfiguration,
}

#[function_component(UnderConstructionBuilding)]
pub fn draw_under_construction_buildings(props: &UnderConstructionBuildingProps) -> Html {
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
                let cmd = SharedPlanetCommand::CancelConstruction { key };
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
            <h3>{"Under Construction Building"}</h3>
            <table>
                <tr>
                    <th>{"Name"}</th>
                    <th>{"Remaining Time"}</th>
                    <th>{"Action"}</th>
                </tr>

                { for props.under_construction_building.iter().map(|(id, (building, end))| html! {
                    <tr key={id.to_string()}>
                        <td>{&building.name}</td>
                        <td>
                            <Clock config={props.time_config} end={*end} />
                        </td>
                        <td>
                            if let Some(create_error) = create_error.as_ref() {
                                <span style="color:red">{create_error}</span>
                            }else{
                                <button id={id.to_string()} onclick={on_create.clone()}>{"cancel"}</button>
                            }
                        </td>
                    </tr>
                }) }
            </table>
        </>
    )
}

#[derive(Properties, PartialEq)]
pub struct ClockProps {
    pub config: HfTimeConfiguration,
    pub end: DateTime<Utc>,
}

#[function_component(Clock)]
pub fn draw_clock(props: &ClockProps) -> Html {
    let state = use_state(Utc::now);

    let timer = state.clone();
    let timeout = Timeout::new(500, move || {
        timer.set(Utc::now());
    });
    timeout.forget();

    let time = HfTime::new(*state, props.config);

    let (duration, hf_duration) = time.remaining(props.end);

    let remaining_time = format!(
        "{:02}:{:02}:{:02}",
        duration.num_hours(),
        duration.num_minutes() % 60,
        duration.num_seconds() % 60
    );
    let hf_remaining_time = format!(
        "{:02}:{:02}:{:02}",
        hf_duration.num_hours(),
        hf_duration.num_minutes() % 60,
        hf_duration.num_seconds() % 60
    );

    html!(
        <p>{remaining_time} { "("}{hf_remaining_time}{")"}</p>
    )
}
