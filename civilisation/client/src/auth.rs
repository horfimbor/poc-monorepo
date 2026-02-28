use horfimbor_client::LoadExternalComponent;
use horfimbor_client_derive::WebComponent;
use horfimbor_jwt::{Claims, Role};
use std::ops::Not;
use chrono::Utc;
use weblog::{console_warn};
use yew::{Component, Context, Html, Properties, html};

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

pub struct GalaxyAuth {}

impl Component for GalaxyAuth {
    type Message = ();
    type Properties = AuthProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let target = format!(
            "{}/auth/authorize?redirect={}",
            ctx.props().auth_endpoint,
            ctx.props().endpoint
        );

        let login_needed = html! {
            <>
                <a href={target.clone()}>{"login"}</a>
            </>
        };

        let Some(account_name) = ctx.props().account_name.clone() else {
            console_warn!("no accound name");
            return login_needed;
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
            if pathname != format!("/{account_name}") {

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

                return login_needed;
            }
        }

        let endpoint = ctx.props().endpoint.clone();
        let Ok(Some(jwt)) = local_storage.get_item(&account_name) else {

            console_warn!("wrong jwt");
            return login_needed;
        };

        let Ok(claims) = Claims::from_jwt_insecure(&jwt)else{

            console_warn!("invalid claims");
            return login_needed;
        };

        if Utc::now().timestamp() > claims.expiration_at() as i64 {

            console_warn!("token expired");
            return login_needed;
        }

        let content = html! {
                    <LoadExternalComponent
                        endpoint={endpoint.clone()}
                        balise={"horfimbor-civilisation-state"}
                        jwt={jwt.clone()}
                        id={""}
                    />
        };

        let is_admin = *claims.roles() == Role::Admin;

        if is_admin.not() {
            return content;
        }

        let admin_content = html! {
            <>
                <LoadExternalComponent
                    endpoint={endpoint.clone()}
                    balise={"horfimbor-civilisation-admin"}
                    jwt={jwt.clone()}
                    id={""}
                />
            </>
        };

        html! {
            <div>
                {admin_content}
                <hr/>
                {content}
            </div>
        }
    }
}
