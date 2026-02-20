use crate::admin::CivilisationAdminProps;
use chrono::{Duration, NaiveDateTime};
use civilisation_admin::CivilisationAdminCommand;
use horfimbor_client::input::send_command;
use horfimbor_time::HfTimeConfiguration;
use std::rc::Rc;
use web_sys::{HtmlInputElement, InputEvent};
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Eq, PartialEq, Default, Debug)]
struct TimerData {
    start_time: Option<String>,
    irl_length: Option<usize>,
    ig_length: Option<usize>,
}

enum TimerAction {
    StartTime(String),
    IrlLength(usize),
    IgLength(usize),
}

impl Reducible for TimerData {
    type Action = TimerAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TimerAction::StartTime(time) => TimerData {
                start_time: Some(time),
                irl_length: self.irl_length,
                ig_length: self.ig_length,
            }
            .into(),
            TimerAction::IrlLength(length) => TimerData {
                start_time: self.start_time.clone(),
                irl_length: Some(length),
                ig_length: self.ig_length,
            }
            .into(),
            TimerAction::IgLength(length) => TimerData {
                start_time: self.start_time.clone(),
                irl_length: self.irl_length,
                ig_length: Some(length),
            }
            .into(),
        }
    }
}

type TimerContext = UseReducerHandle<TimerData>;

#[function_component(UpdateTimer)]
pub fn update_timer(props: &CivilisationAdminProps) -> Html {
    let endpoint = props.endpoint.clone();
    let jwt = props.jwt.clone();

    let msg = use_reducer(|| TimerData {
        start_time: None,
        irl_length: None,
        ig_length: None,
    });

    html! {
        <ContextProvider<TimerContext> context={msg}>
            <StartDate /> <br/>
            <IrlLength /> <br/>
            <IgLength /> <br/>
            <TimerDataSetter {endpoint} {jwt}/>
        </ContextProvider<TimerContext>>
    }
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

#[function_component(StartDate)]
fn start_date() -> Html {
    let msg_ctx = use_context::<TimerContext>().unwrap();

    let value = msg_ctx.start_time.clone();

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        msg_ctx.dispatch(TimerAction::StartTime(input.value()));
    });

    html! {
        <label>{"Start date"}
            <input type="datetime-local"
                {oninput} {value}
                min="2025-01-01T00:00"
                />
        </label>
    }
}

#[function_component(IrlLength)]
fn irl_length() -> Html {
    let msg_ctx = use_context::<TimerContext>().unwrap();

    let value = msg_ctx.irl_length;

    let cb = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        msg_ctx.dispatch(TimerAction::IrlLength(
            input.value().parse::<usize>().unwrap_or_default(),
        ));
    });

    html! {
        <label>{"irl duration in minutes"}
                <input type="number"
                   oninput={cb} value={value.unwrap_or_default().to_string()} min="1"
                    />
            </label>
    }
}

#[function_component(IgLength)]
fn ig_length() -> Html {
    let msg_ctx = use_context::<TimerContext>().unwrap();

    let value = msg_ctx.ig_length;

    let cb = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        msg_ctx.dispatch(TimerAction::IgLength(
            input.value().parse::<usize>().unwrap_or_default(),
        ));
    });

    html! {
        <label>{"in game duration in minutes"}
            <input type="number"
               oninput={cb} value={value.unwrap_or_default().to_string()} min="1"
                />
        </label>
    }
}

#[function_component(TimerDataSetter)]
fn local_data_setter(props: &CivilisationAdminProps) -> Html {
    let msg_ctx = use_context::<TimerContext>().unwrap();

    let btn = if let Some(config) = msg_ctx.get_config() {
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
            {btn}
        </div>
    }
}
