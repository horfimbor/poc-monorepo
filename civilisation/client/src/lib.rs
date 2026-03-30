mod admin;
mod auth;
mod clock;
mod nation;
mod state;

use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    yew::set_event_bubbling(false);
    state::optional::ComponentWrapper::define("horfimbor-civilisation-state");
    admin::optional::ComponentWrapper::define("horfimbor-civilisation-admin");
    auth::optional::ComponentWrapper::define("horfimbor-auth");
}
