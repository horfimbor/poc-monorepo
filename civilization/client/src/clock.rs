use chrono::Utc;
use gloo_timers::callback::Timeout;
use horfimbor_time::{HfStatus, HfTime, HfTimeConfiguration};
use yew::{Html, Properties, function_component, html, use_state};

#[derive(Properties, PartialEq)]
pub struct ClockProps {
    pub config: HfTimeConfiguration,
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

    let (status, duration) = time.hf_status();

    let remaining_time = format!(
        "{:02}:{:02}:{:02}",
        duration.num_hours(),
        duration.num_minutes() % 60,
        duration.num_seconds() % 60
    );

    match status {
        HfStatus::Paused => {
            html!(
                <p>{"game restart in: "}{remaining_time}</p>
            )
        }
        HfStatus::Running => {
            html!(
                <p>{"game time stop in: "}{remaining_time}</p>
            )
        }
    }
}
