use chrono::Utc;
use gloo_timers::callback::Timeout;
use horfimbor_time::HfTimeConfiguration;
use planet_shared::dto::{Resource, ResourceCalc};
use std::collections::HashMap;
use yew::{Html, Properties, function_component, html, use_state};

#[derive(Properties, PartialEq)]
pub struct ResourcesProps {
    pub resources: HashMap<Resource, ResourceCalc>,
    pub time_config: HfTimeConfiguration,
}

#[function_component(Resources)]
pub fn draw_resources(props: &ResourcesProps) -> Html {
    let state = use_state(Utc::now);
    let timer = state.clone();
    let timeout = Timeout::new(500, move || {
        timer.set(Utc::now());
    });
    timeout.forget();

    html!(
        <>
             <h3>{"Resources"}</h3>
                <table>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Stock"}</th>
                        <th>{"Production"}</th>
                        <th>{"maximal stock"}</th>
                    </tr>
                { for props.resources.iter().map(|(id, calc)|{
                    let computed = calc.compute_quantity(props.time_config, Some(*state));
                    html! {
                        <tr key={id.to_string()}>
                            <td>{id.to_string()}</td>
                            <td>{computed.quantity}</td>
                            <td>{computed.production}</td>
                            <td>{computed.stock_capacity}</td>
                        </tr>
                    }
                } )}
                </table>

        </>
    )
}
