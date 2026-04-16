use crate::state::CivilizationProps;
use civilization_shared::Nation;
use civilization_shared::command::CivilizationCommand;
use garde::Validate;
use horfimbor_client::input::send_command;
use serde::Deserialize;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct NationProps {
    pub endpoint: AttrValue,
    pub jwt: AttrValue,
    pub nation: Option<Nation>,
}

#[function_component(NationDisplay)]
pub fn display_nation(props: &NationProps) -> Html {
    let msg_nation = use_reducer(|| NationData {
        endpoint: props.endpoint.clone(),
        jwt: props.jwt.clone(),
        edit_mode: false,
        edit_nation: Nation {
            name: "".to_string(),
            description: "".to_string(),
        },
    });

    let display = if let Some(nation) = props.nation.clone() {
        html!(<div>
                    <b>{&nation.name}</b><p>{&nation.description}</p>
                </div>)
    } else {
        html!(<div>
                    {"No name yet"}
                </div>)
    };

    html!( <>
                {display}
            <ContextProvider<NationContext> context={msg_nation}>
                <NationDesc/>
                <Toggle />
            </ContextProvider<NationContext>>

    </>)
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct NationData {
    pub endpoint: AttrValue,
    pub jwt: AttrValue,
    pub edit_mode: bool,
    pub edit_nation: Nation,
}

impl NationData {
    pub fn get_command(&self) -> Option<CivilizationCommand> {
        if self.edit_nation.validate().is_ok() {
            Some(CivilizationCommand::UpdateNation(self.edit_nation.clone()))
        } else {
            None
        }
    }
}

pub enum ComponentAction {
    ChangeName(String),
    ChangeDescription(String),
    ToggleEdition,
}

pub type NationContext = UseReducerHandle<NationData>;

impl Reducible for NationData {
    type Action = ComponentAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut reduced = (*self).clone();
        match action {
            ComponentAction::ChangeName(name) => {
                reduced.edit_nation.name = name;
                reduced
            }
            .into(),
            ComponentAction::ChangeDescription(description) => {
                reduced.edit_nation.description = description;
                reduced
            }
            .into(),
            ComponentAction::ToggleEdition => {
                reduced.edit_mode = !reduced.edit_mode;
                reduced
            }
            .into(),
        }
    }
}

#[function_component(NationDesc)]
pub fn nation_view() -> Html {
    let Some(msg_ctx) = use_context::<NationContext>() else {
        console_error!("no context");
        return html!(<></>);
    };
    if msg_ctx.edit_mode {
        html! {
            <>
                <div>
                    <SetName />
                    <SetDescription />
                    <Sender />
                </div>
                <ErrorDisplay />
            </>
        }
    } else {
        html! {
        <></>
            }
    }
}

#[function_component(ErrorDisplay)]
fn error_display() -> Html {
    let Some(msg_ctx) = use_context::<NationContext>() else {
        console_error!("no context");
        return html!(<></>);
    };
    if let Err(message) = msg_ctx.edit_nation.validate() {
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
    let Some(msg_ctx) = use_context::<NationContext>() else {
        console_error!("no context");
        return html!(<></>);
    };
    let value = msg_ctx.edit_nation.name.clone();

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        msg_ctx.dispatch(ComponentAction::ChangeName(input.value()));
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

#[function_component(Toggle)]
fn toggle() -> Html {
    let Some(msg_ctx) = use_context::<NationContext>() else {
        console_error!("no context");
        return html!(<></>);
    };
    let onclick = Callback::from(move |_| {
        msg_ctx.dispatch(ComponentAction::ToggleEdition);
    });

    html! {
        <>
            <button type="text"
                {onclick}  > {"edit"}
            </button>
        </>
    }
}

#[function_component(SetDescription)]
fn set_description() -> Html {
    let Some(msg_ctx) = use_context::<NationContext>() else {
        console_error!("no context");
        return html!(<></>);
    };
    let value = msg_ctx.edit_nation.description.clone();

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        msg_ctx.dispatch(ComponentAction::ChangeDescription(input.value()));
    });

    html! {
        <>
            <label>{"Nation description"}
                <input type="text" {oninput} {value} />
            </label>
        </>
    }
}

#[function_component(Sender)]
fn sender() -> Html {
    let Some(msg_ctx) = use_context::<NationContext>() else {
        console_error!("no context");
        return html!(<></>);
    };
    let props = CivilizationProps {
        endpoint: msg_ctx.endpoint.clone(),
        jwt: msg_ctx.jwt.clone(),
    };

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
