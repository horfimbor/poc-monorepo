use crate::admin::CivilisationAdminProps;
use civilisation_admin::{CivilisationAdminCommand, Component};
use horfimbor_client::input::send_command;
use std::rc::Rc;
use url::Url;
use web_sys::{HtmlInputElement, InputEvent};
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Eq, PartialEq, Default)]
struct ComponentData {
    name: Option<String>,
    host: Option<Url>,
    tag: Option<String>,
}

impl ComponentData {
    fn get_command(&self) -> Option<CivilisationAdminCommand> {
        if let (Some(name), Some(host), Some(tag)) =
            (self.name.clone(), self.host.clone(), self.tag.clone())
        {
            Some(CivilisationAdminCommand::AddComponent {
                name,
                comp: Component { url: host, tag },
            })
        } else {
            None
        }
    }
}

enum ComponentAction {
    Name(String),
    Host(Url),
    Tag(String),
}

impl Reducible for ComponentData {
    type Action = ComponentAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ComponentAction::Name(name) => ComponentData {
                name: Some(name),
                host: self.host.clone(),
                tag: self.tag.clone(),
            }
            .into(),
            ComponentAction::Host(host) => ComponentData {
                name: self.name.clone(),
                host: Some(host),
                tag: self.tag.clone(),
            }
            .into(),
            ComponentAction::Tag(tag) => ComponentData {
                name: self.name.clone(),
                host: self.host.clone(),
                tag: Some(tag),
            }
            .into(),
        }
    }
}

type AddComponentContext = UseReducerHandle<ComponentData>;

#[function_component(AddComponent)]
pub fn update_timer(props: &CivilisationAdminProps) -> Html {
    let endpoint = props.endpoint.clone();
    let jwt = props.jwt.clone();

    let msg = use_reducer(|| ComponentData {
        name: None,
        host: None,
        tag: None,
    });

    html!(
        <ContextProvider<AddComponentContext> context={msg}>
            <ComponentName /> <br/>
            <ComponenUrl /> <br/>
            <ComponenTag /> <br/>
            <AddComponentSetter {endpoint} {jwt} />
        </ContextProvider<AddComponentContext>>
    )
}

#[function_component(ComponentName)]
fn component_name() -> Html {
    let msg_ctx = use_context::<AddComponentContext>().unwrap();

    let value = msg_ctx.name.clone();

    let cb = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        msg_ctx.dispatch(ComponentAction::Name(input.value()));
    });

    html! {
        <label>{"name"}
            <input type="text"
                oninput={cb} value={value}
                />
        </label>
    }
}
#[function_component(ComponenTag)]
fn component_tag() -> Html {
    let msg_ctx = use_context::<AddComponentContext>().unwrap();

    let value = msg_ctx.tag.clone();

    let cb = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        msg_ctx.dispatch(ComponentAction::Tag(input.value()));
    });

    html! {
        <label>{"tag"}
            <input type="text"
                oninput={cb} value={value}
                />
        </label>
    }
}
#[function_component(ComponenUrl)]
fn component_url() -> Html {
    let msg_ctx = use_context::<AddComponentContext>().unwrap();

    let value = if let Some(host) = msg_ctx.host.clone() {
        host.to_string()
    } else {
        "".to_string()
    };

    let cb = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        if let Ok(host) = Url::parse(&input.value()) {
            msg_ctx.dispatch(ComponentAction::Host(host));
        }
    });

    html! {
        <label>{"url"}
            <input type="url"
                oninput={cb} {value}
                />
        </label>
    }
}

#[function_component(AddComponentSetter)]
pub fn add_component(props: &CivilisationAdminProps) -> Html {
    let msg_ctx = use_context::<AddComponentContext>().unwrap();

    let add_button = if let Some(cmd) = msg_ctx.get_command() {
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
        html!(<button onclick={on_send_clicked}>{"add component"}</button>)
    } else {
        html!(<p>{"incomplete component"}</p>)
    };

    html! {
        <>
            {add_button}
        </>
    }
}
