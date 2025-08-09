use horfimbor_client_derive::WebComponent;
use weblog::{console_info, console_warn};
use yew::{Component, Context, Html, Properties, html};

#[derive(WebComponent)]
#[component(GalaxyAuth)]
#[derive(Default, Properties, PartialEq)]
pub struct GalaxyAuthProps {
    endpoint: String,
    auth_endpoint: String,
    #[optionnal]
    jwt: Option<String>,
    #[optionnal]
    account_name: Option<String>,
}

#[allow(dead_code)]
pub struct GalaxyAuth {}

impl Component for GalaxyAuth {
    type Message = ();
    type Properties = GalaxyAuthProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        console_info!("CHANGE");
        true
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

        html! {
            <horfimbor-account-state endpoint={{endpoint.clone()}} jwt={{jwt.clone()}}></horfimbor-account-state>
        }
    }
}
