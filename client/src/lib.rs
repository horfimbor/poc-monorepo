mod account;
mod auth;

use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    account::state::optional::ComponentWrapper::define("horfimbor-account-state");
    account::input::optional::ComponentWrapper::define("horfimbor-account-input");
    auth::optional::ComponentWrapper::define("horfimbor-auth");
}
