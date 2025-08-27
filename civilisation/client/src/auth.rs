use horfimbor_client::LoadExternalComponent;
use horfimbor_client_derive::WebComponent;
use horfimbor_jwt::{Claims, Role};
use std::ops::Not;
use weblog::{console_error, console_info, console_warn};
use yew::{Callback, Component, Context, Html, Properties, html};

#[derive(WebComponent)]
#[component(GalaxyAuth)]
#[derive(Default, Properties, PartialEq)]
pub struct AuthProps {
    endpoint: String,
    auth_endpoint: String,
    #[optionnal]
    jwt: Option<String>,
    #[optionnal]
    account_name: Option<String>,
}

pub struct GalaxyAuth {
    admin_open: bool,
}

pub enum GalaxyEvent {
    ToggleAdmin,
}

impl Component for GalaxyAuth {
    type Message = GalaxyEvent;
    type Properties = AuthProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { admin_open: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GalaxyEvent::ToggleAdmin => {
                self.admin_open = self.admin_open.not();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let target = format!(
            "{}/auth/authorize?redirect={}",
            ctx.props().auth_endpoint,
            ctx.props().endpoint
        );

        let Some(account_name) = ctx.props().account_name.clone() else {
            return html! {
                <>
                    <a href={target}>{"login"}</a>
                </>
            };
        };

        let Some(window) = web_sys::window() else {
            console_warn!("cannot access window");

            return html! {
                <>
                    <p>{"Cannot access window from Js"}</p>
                </>
            };
        };

        let Ok(Some(local_storage)) = window.local_storage() else {
            console_warn!("cannot access local_storage");
            return html! {
                <>
                    <p>{"Cannot access local_storage from Js"}</p>
                </>
            };
        };

        if let Some(jwt) = ctx.props().jwt.clone() {
            match local_storage.set_item(&account_name, &jwt) {
                Ok(_) => {}
                Err(e) => {
                    console_warn!("Cannot set value in local_storage", e);
                    return html! {
                        <>
                            <p>{"Cannot set value in local_storage"}</p>
                        </>
                    };
                }
            }

            let location = window.location();
            let Ok(pathname) = location.pathname() else {
                console_warn!("Cannot get pathname");
                return html! {
                    <>
                        <p>{"Cannot get pathname"}</p>
                    </>
                };
            };
            if pathname != format!("//{account_name}") {
                // TODO fix those //

                match location.set_href(&account_name) {
                    Ok(_) => {}
                    Err(e) => {
                        console_warn!("Cannot redirect", e);
                        return html! {
                            <>
                                <p>{"Cannot redirect"}</p>
                            </>
                        };
                    }
                }

                return html! {
                    <>
                        <p>{"redirecting"}</p>
                    </>
                };
            }
        }

        let endpoint = ctx.props().endpoint.clone();
        let Ok(Some(jwt)) = local_storage.get_item(&account_name) else {
            return html! {
                <>
                    <a href={target}>{"login"}</a>
                </>
            };
        };

        let content = html! {
                    <LoadExternalComponent
                        endpoint={endpoint.clone()}
                        balise={"horfimbor-civilisation-state"}
                        jwt={jwt.clone()}
                        id={""}
                    />
        };

        let is_admin = match Claims::from_jwt_insecure(&jwt) {
            Ok(data) => *data.roles() == Role::Admin,
            Err(e) => {
                console_error!(e.to_string());
                false
            }
        };

        if is_admin.not() {
            return content;
        }

        let admin_content = if self.admin_open {
            html! {
                <LoadExternalComponent
                    endpoint={endpoint.clone()}
                    balise={"horfimbor-civilisation-admin"}
                    jwt={jwt.clone()}
                    id={""}
                />
            }
        } else {
            content
        };

        let link = ctx.link().clone();
        let onclick = Callback::from(move |_| link.send_message(GalaxyEvent::ToggleAdmin));

        html! {
            <>
            <button {onclick}>{"toggle admin"}</button>
            {admin_content}
            </>
        }
    }
}
