use bounce::BounceRoot;
use bounce::{Atom, use_atom};
use horfimbor_client_derive::WebComponent;
use reqwasm::http::{Request, Response};
use weblog::console_info;
use yew::platform::spawn_local;
use yew::prelude::*;

use planet_shared::command::PlanetCommand;
use planet_shared::dto::PlanetDto;

#[derive(WebComponent)]
#[component(MonoInput)]
#[derive(Default, Properties, PartialEq)]
pub struct MonoInputProps {
    pub endpoint: String,
    pub jwt: String,
    pub id: String,
}

#[derive(Eq, PartialEq, Atom, Default)]
struct LocalError {
    err: Option<String>,
}

#[function_component(ErrorDisplay)]
fn error_display() -> Html {
    let data = use_atom::<LocalError>();

    match data.err.clone() {
        None => {
            html! {}
        }
        Some(e) => {
            html! {
                <h2>
                    {e}
                </h2>
            }
        }
    }
}

#[function_component(Sender)]
fn sender(props: &MonoInputProps) -> Html {
    let err = use_atom::<LocalError>();
    let endpoint = props.endpoint.clone();
    let jwt = props.jwt.clone();
    let id = props.id.clone();

    let on_send_clicked = Callback::from(move |_| {
        let err = err.clone();

        let cmd = PlanetCommand::Ping;

        let endpoint = endpoint.clone();
        let jwt = jwt.clone();
        let id = id.clone();
        spawn_local(async move {
            let endpoint = endpoint.clone();
            match send_command(&cmd, endpoint, jwt, id).await {
                Ok(resp) => {
                    if resp.ok() {
                        console_info!("sent !");
                    }
                }
                Err(e) => {
                    err.set(LocalError { err: Some(e) });
                }
            }
        });
    });

    html! { <button id="btn-send" onclick={on_send_clicked}>{"👍"}</button> }
}

async fn send_command(
    cmd: &PlanetCommand,
    endpoint: String,
    jwt: String,
    id: String,
) -> Result<Response, String> {
    Request::post(&format!("{endpoint}/api/planet/{id}"))
        .body(serde_json::to_string(&cmd).map_err(|_| format!("cannot serialize cmd {:?}", &cmd))?)
        .header("Content-Type", "application/json")
        .header("Authorization", &jwt)
        .send()
        .await
        .map_err(|_| "fail to send command".to_string())
}

#[allow(dead_code)]
#[derive(PartialEq, Atom, Default)]
struct State {
    content: PlanetDto,
}

#[allow(dead_code)]
pub struct MonoInput {}

impl Component for MonoInput {
    type Message = ();

    type Properties = MonoInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let endpoint = ctx.props().endpoint.clone();
        let jwt = ctx.props().jwt.clone();
        let id = ctx.props().id.clone();
        html! {
            <BounceRoot>
                <div>
                    <Sender endpoint={endpoint.clone()} jwt={jwt.clone()} id={id.clone()} />
                </div>
                <div>
                    <ErrorDisplay />
                </div>
            </BounceRoot>
        }
    }
}
