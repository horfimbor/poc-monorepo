use crate::state::CivilisationProps;
use civilisation_shared::Nation;
use civilisation_shared::command::CivilisationCommand;
use garde::Validate;
use horfimbor_client::input::send_command;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Eq, PartialEq)]
struct LocalData {
    nation: Nation,
}

impl LocalData {
    fn get_command(&self) -> Option<CivilisationCommand> {
        if self.nation.validate().is_ok() {
            Some(CivilisationCommand::UpdateNation(self.nation.clone()))
        } else {
            None
        }
    }
}

enum ComponentAction {
    Name(String),
    Description(String),
}

impl Reducible for LocalData {
    type Action = ComponentAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ComponentAction::Name(name) => LocalData {
                nation: Nation {
                    name,
                    description: self.nation.description.clone(),
                },
            }
            .into(),
            ComponentAction::Description(description) => LocalData {
                nation: Nation {
                    name: self.nation.name.clone(),
                    description,
                },
            }
            .into(),
        }
    }
}

type CivilisationContext = UseReducerHandle<LocalData>;

#[function_component(CivilisationInput)]
pub fn view(props: &CivilisationProps) -> Html {
    let endpoint = props.endpoint.clone();
    let jwt = props.jwt.clone();

    let msg = use_reducer(|| LocalData {
        nation: Nation {
            name: "".to_string(),
            description: "".to_string(),
        },
    });

    html! {
        <ContextProvider<CivilisationContext> context={msg}>
            <div>
                <SetName />
                <SetDescription />
                <Sender endpoint={endpoint.clone()} jwt={jwt.clone()} />
            </div>
            <ErrorDisplay />
        </ContextProvider<CivilisationContext>>
    }
}

#[function_component(ErrorDisplay)]
fn error_display() -> Html {
    let msg_ctx = use_context::<CivilisationContext>().unwrap();

    if let Err(message) = msg_ctx.nation.validate() {
        return html! {
            <p>
                    {message.to_string()}
            </p>
        };
    }

    html! {
        <>
        </>
    }
}

#[function_component(SetName)]
fn set_name() -> Html {
    let msg_ctx = use_context::<CivilisationContext>().unwrap();

    let value = msg_ctx.nation.name.clone();

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        msg_ctx.dispatch(ComponentAction::Name(input.value()));
    });

    html! {
        <>
            <label>{"Nation name"}
                <input type="text"
                {oninput} {value} />
            </label><br/>
        </>
    }
}

#[function_component(SetDescription)]
fn set_description() -> Html {
    let msg_ctx = use_context::<CivilisationContext>().unwrap();

    let value = msg_ctx.nation.description.clone();

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        msg_ctx.dispatch(ComponentAction::Description(input.value()));
    });

    html! {
        <>
            <label>{"Nation description"}
                <input type="text"{oninput} {value} />
            </label>
        </>
    }
}

#[function_component(Sender)]
fn sender(props: &CivilisationProps) -> Html {
    let msg_ctx = use_context::<CivilisationContext>().unwrap();

    if let Some(cmd) = msg_ctx.get_command() {
        let props = props.clone();
        let cmd = cmd.clone();
        let on_send_clicked = Callback::from(move |_| {
            let props = props.clone();
            let cmd = cmd.clone();
            spawn_local(async move {
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

        return html! {
            <>
                <button onclick={on_send_clicked}>{"update"}</button>
            </>
        };
    }

    html! { <></> }
}
