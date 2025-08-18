use bounce::BounceRoot;
use bounce::{Atom, use_atom};
use garde::Validate;
use horfimbor_client_derive::WebComponent;
use reqwasm::http::{Request, Response};
use web_sys::HtmlInputElement;
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
use yew::prelude::*;

use civilisation_shared::Nation;
use civilisation_shared::command::CivilisationCommand;
use civilisation_shared::dto::CivilisationDto;

#[derive(WebComponent)]
#[component(MonoInput)]
#[derive(Default, Properties, PartialEq)]
pub struct MonoInputProps {
    pub endpoint: String,
    pub jwt: String,
}

#[derive(Eq, PartialEq, Atom, Default)]
struct LocalData {
    name: String,
    description: String,
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

#[function_component(LocalDataSetter)]
fn local_data_setter() -> Html {
    let data = use_atom::<LocalData>();

    let on_name_input = {
        let data = data.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            data.set(LocalData {
                name: input.value(),
                description: data.description.clone(),
            });
        })
    };

    let on_description_input = {
        let data = data.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            data.set(LocalData {
                name: data.name.clone(),
                description: input.value(),
            });
        })
    };

    html! {
        <div>
            <label>{"Nation name"}
                <input type="text" oninput={on_name_input} value={data.name.clone()} />
            </label><br/>
            <label>{"Nation description"}
                <input type="text" oninput={on_description_input} value={data.description.clone()} />
            </label>
        </div>
    }
}

#[function_component(Sender)]
fn sender(props: &MonoInputProps) -> Html {
    let data = use_atom::<LocalData>();
    let err = use_atom::<LocalError>();
    let endpoint = props.endpoint.clone();
    let jwt = props.jwt.clone();

    let nation = Nation {
        name: data.name.clone(),
        description: data.description.clone(),
    };

    if let Err(e) = nation.validate() {
        let message = format!("invalid nation : {e}");
        return html!( <div>{message}</div>);
    }

    let on_send_clicked = Callback::from(move |_| {
        let err = err.clone();
        let nation = nation.clone();

        let cmd = CivilisationCommand::UpdateNation(nation);

        let endpoint = endpoint.clone();
        let jwt = jwt.clone();
        spawn_local(async move {
            let endpoint = endpoint.clone();
            match send_command(&cmd, endpoint, jwt).await {
                Ok(resp) => {
                    if resp.ok() {
                        console_info!("sent !");
                        let content = resp.text().await;
                        match content {
                            Ok(response) => {
                                if !response.is_empty() {
                                    err.set(LocalError {
                                        err: Some(response),
                                    });
                                }
                            }
                            Err(e) => {
                                console_error!(e.to_string())
                            }
                        }
                    }
                }
                Err(e) => {
                    err.set(LocalError { err: Some(e) });
                }
            }
        });
    });

    html! { <button id="btn-send" onclick={on_send_clicked}>{"Send"}</button> }
}

async fn send_command(
    cmd: &CivilisationCommand,
    endpoint: String,
    jwt: String,
) -> Result<Response, String> {
    Request::post(&format!("{endpoint}/api/civilisation"))
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
    content: CivilisationDto,
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
        html! {
            <BounceRoot>
                <div>
                    <LocalDataSetter />
                    <Sender endpoint={endpoint.clone()} jwt={jwt.clone()} />
                </div>
                <div>
                    <ErrorDisplay />
                </div>
            </BounceRoot>
        }
    }
}
