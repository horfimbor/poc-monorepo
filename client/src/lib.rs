mod account;
mod auth;
mod planet;

use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    account::state::optional::ComponentWrapper::define("horfimbor-account-state");
    account::input::optional::ComponentWrapper::define("horfimbor-account-input");
    planet::state::optional::ComponentWrapper::define("horfimbor-planet-state");
    planet::input::optional::ComponentWrapper::define("horfimbor-planet-input");

    auth::optional::ComponentWrapper::define("horfimbor-auth");
}
