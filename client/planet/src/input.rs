use bounce::BounceRoot;
use bounce::{Atom, use_atom};
use horfimbor_client::EventStoreProps;
use horfimbor_client::input::send_command;
use horfimbor_client_derive::WebComponent;
use planet_shared::command::PlanetCommand;
use planet_shared::dto::PlanetDto;
use serde::Deserialize;
use weblog::console_info;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(WebComponent)]
#[component(MonoInput)]
#[derive(Default, Properties, PartialEq, Deserialize, Debug, Clone)]
pub struct MonoInputProps {
    pub endpoint: String,
    pub jwt: String,
    pub id: String,
}

impl EventStoreProps for MonoInputProps {
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
    let props = props.clone();

    let on_send_clicked = Callback::from(move |_| {
        let err = err.clone();

        let cmd = PlanetCommand::Ping;
        let props = props.clone();

        spawn_local(async move {
            match send_command(&cmd, props).await {
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
