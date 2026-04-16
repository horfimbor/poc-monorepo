use crate::admin::CivilizationAdminProps;
use civilization_shared::admin::Service;
use civilization_shared::command::CivilizationAdminCommand;
use horfimbor_client::input::send_command;
use std::rc::Rc;
use url::Url;
use web_sys::{HtmlInputElement, InputEvent};
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Eq, PartialEq, Default)]
struct ServiceData {
    name: Option<String>,
    host: Option<Url>,
    tag: Option<String>,
}

impl ServiceData {
    fn get_command(&self) -> Option<CivilizationAdminCommand> {
        if let (Some(name), Some(host), Some(tag)) =
            (self.name.clone(), self.host.clone(), self.tag.clone())
        {
            Some(CivilizationAdminCommand::AddService {
                name,
                comp: Service {
                    url: host,
                    balise: tag,
                },
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

impl Reducible for ServiceData {
    type Action = ComponentAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ComponentAction::Name(name) => ServiceData {
                name: Some(name),
                host: self.host.clone(),
                tag: self.tag.clone(),
            }
            .into(),
            ComponentAction::Host(host) => ServiceData {
                name: self.name.clone(),
                host: Some(host),
                tag: self.tag.clone(),
            }
            .into(),
            ComponentAction::Tag(tag) => ServiceData {
                name: self.name.clone(),
                host: self.host.clone(),
                tag: Some(tag),
            }
            .into(),
        }
    }
}

type AddServiceContext = UseReducerHandle<ServiceData>;

#[function_component(AddService)]
pub fn update_timer(props: &CivilizationAdminProps) -> Html {
    let endpoint = props.endpoint.clone();
    let jwt = props.jwt.clone();

    let msg = use_reducer(|| ServiceData {
        name: None,
        host: None,
        tag: None,
    });

    html!(
        <ContextProvider<AddServiceContext> context={msg}>
            <ComponentName /> <br/>
            <ComponenUrl /> <br/>
            <ComponenTag /> <br/>
            <AddComponentSetter {endpoint} {jwt} />
        </ContextProvider<AddServiceContext >>
    )
}

#[function_component(ComponentName)]
fn component_name() -> Html {
    let Some(msg_ctx) = use_context::<AddServiceContext>() else {
        console_error!("no context");
        return html!(<></>);
    };

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
    let Some(msg_ctx) = use_context::<AddServiceContext>() else {
        console_error!("no context");
        return html!(<></>);
    };
    let value = msg_ctx.tag.clone();

    let cb = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        msg_ctx.dispatch(ComponentAction::Tag(input.value()));
    });

    html! {
        <label>{"admin tag"}
            <input type="text"
                oninput={cb} value={value}
                />
        </label>
    }
}
#[function_component(ComponenUrl)]
fn component_url() -> Html {
    let Some(msg_ctx) = use_context::<AddServiceContext>() else {
        console_error!("no context");
        return html!(<></>);
    };
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
pub fn add_component(props: &CivilizationAdminProps) -> Html {
    let Some(msg_ctx) = use_context::<AddServiceContext>() else {
        console_error!("no context");
        return html!(<></>);
    };
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
