use crate::admin::CivilisationAdminProps;
use crate::input::ErrorDisplay;
use bounce::{Atom, BounceRoot, use_atom};
use chrono::{Duration, NaiveDateTime};
use civilisation_admin::CivilisationAdminCommand;
use horfimbor_client::input::send_command;
use horfimbor_time::HfTimeConfiguration;
use web_sys::{HtmlInputElement, InputEvent};
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
use yew::{Callback, Html, TargetCast, function_component, html};

#[derive(Eq, PartialEq, Atom, Default, Debug)]
struct TimerData {
    start_time: Option<String>,
    irl_length: Option<usize>,
    ig_length: Option<usize>,
}

impl TimerData {
    fn get_config(&self) -> Option<HfTimeConfiguration> {
        let (Some(start_time), Some(irl_length), Some(ig_length)) =
            (self.start_time.clone(), self.irl_length, self.ig_length)
        else {
            return None;
        };

        let Ok(start) = NaiveDateTime::parse_from_str(&start_time, "%Y-%m-%dT%H:%M") else {
            return None;
        };

        let config = HfTimeConfiguration::new(
            Duration::minutes(irl_length as i64),
            Duration::minutes(ig_length as i64),
            start.and_utc(),
        );

        match config {
            Ok(config) => Some(config),
            Err(e) => {
                console_error!(e.to_string());
                None
            }
        }
    }
}

#[function_component(UpdateTimer)]
pub fn update_timer(props: &CivilisationAdminProps) -> Html {
    let endpoint = props.endpoint.clone();
    let jwt = props.jwt.clone();
    html! {
        <BounceRoot>
            <div>
                <TimerDataSetter {endpoint} {jwt}/>
            </div>
            <div>
                <ErrorDisplay />
            </div>
        </BounceRoot>
    }
}

#[function_component(TimerDataSetter)]
fn local_data_setter(props: &CivilisationAdminProps) -> Html {
    let data = use_atom::<TimerData>();

    let start_change = {
        let data = data.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            console_info!(input.value());

            data.set(TimerData {
                start_time: Some(input.value()),
                irl_length: data.irl_length,
                ig_length: data.ig_length,
            });
        })
    };
    let irl_change = {
        let data = data.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            data.set(TimerData {
                start_time: data.start_time.clone(),
                irl_length: Some(input.value().parse::<usize>().unwrap_or_default()),
                ig_length: data.ig_length,
            });
        })
    };
    let ig_change = {
        let data = data.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            data.set(TimerData {
                start_time: data.start_time.clone(),
                irl_length: data.irl_length,
                ig_length: Some(input.value().parse::<usize>().unwrap_or_default()),
            });
        })
    };

    let btn = if let Some(config) = data.get_config() {
        let props = props.clone();
        let on_set_clicked = Callback::from(move |_| {
            let cmd = CivilisationAdminCommand::AddTime(config);
            let props = props.clone();

            spawn_local(async move {
                let props = props.clone();

                match send_command(&cmd, props).await {
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
        html!(<button onclick={on_set_clicked}>{"set timer"}</button>)
    } else {
        html!(<p>{"incomplete timer"}</p>)
    };

    html! {
        <div>
            <label>{"Start date"}
                <input type="datetime-local"
                    oninput={start_change} value={data.start_time.clone()}
                    min="2025-01-01T00:00"
                    />
            </label>
        <label>{"irl duration in minutes"}
                <input type="number"
                   oninput={irl_change} value={data.irl_length.unwrap_or_default().to_string()}
                    min="1"
                    />
            </label>
        <label>{"in game duration in minutes"}
                <input type="number"
                    oninput={ig_change} value={data.ig_length.unwrap_or_default().to_string()}
                    min="1"
                    />
            </label>
            {btn}
        </div>
    }
}
