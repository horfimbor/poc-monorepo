mod admin;
mod auth;
mod input;
mod state;

use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    state::optional::ComponentWrapper::define("horfimbor-civilisation-state");
    admin::optional::ComponentWrapper::define("horfimbor-civilisation-admin");
    auth::optional::ComponentWrapper::define("horfimbor-auth");
}
