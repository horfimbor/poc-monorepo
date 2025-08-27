use crate::web::AuthConfig;
use crate::{built_info, web};
use chrono::prelude::*;
use rocket::response::Redirect;
use rocket::{Route, State};
use rocket_dyn_templates::{Template, context};
use std::env;
use std::time::Duration;

pub fn load_base_routes() -> Vec<Route> {
    routes![index, redirect_index_js, account, auth]
}

#[get("/")]
pub async fn index(host: &State<AuthConfig>) -> Template {
    let local: DateTime<Local> = Local::now();

    Template::render(
        "index",
        context! {
            title: format!("Hello, world! {}",local.format("%x %T")),
            endpoint: format!("{}", host.app_host ),
            key : get_wasm_key(host),
            auth_endpoint: format!("{}",  host.auth_host),
            jwt: "",
            account_name: ""
        },
    )
}

fn get_wasm_key(host: &State<AuthConfig>) -> String {
    host.app_host
        .clone()
        .chars()
        .filter(|&c| c.is_alphanumeric())
        .collect()
}

#[get("/<account_name>")]
pub async fn account(host: &State<AuthConfig>, account_name: &str) -> Template {
    Template::render(
        "index",
        context! {
            title: format!("Hi {account_name}"),
            endpoint: format!("{}", host.app_host ),
            key : get_wasm_key(host),
            auth_endpoint: format!("{}",  host.auth_host),
            jwt: "",
            account_name: account_name,
        },
    )
}

#[get("/auth?<token>")]
pub async fn auth(token: &str, host: &State<AuthConfig>) -> Result<Template, String> {
    let response = reqwest::Client::new()
        .post(format!(
            "{}/auth/single-use-token",
            host.auth_callback_host.clone()
        ))
        .form(&[("token", token), ("app_key", &host.app_key)])
        .timeout(Duration::from_millis(200))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    let claims = web::get_jwt_claims(&response)?;

    dbg!(&claims);

    Ok(Template::render(
        "index",
        context! {
            title: "Hello, connected user!",
            endpoint: format!("{}", host.app_host ),
            key : get_wasm_key(host),
            auth_endpoint: format!("{}",  host.auth_host),
            jwt: response,
            account_name: claims.account_name()
        },
    ))
}

#[get("/client/index.js")]
pub fn redirect_index_js() -> Redirect {
    let wasm_tag: &'static str = env!("WASM_TAG");
    if !wasm_tag.is_empty() {
        Redirect::temporary(format!("/client/index-{wasm_tag}.js"))
    } else {
        Redirect::temporary(format!(
            "/client/index-v{}.js",
            built_info::PKG_VERSION.replace('.', "-")
        ))
    }
}
