mod input;
mod state;
mod auth;

use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    state::optional::ComponentWrapper::define("horfimbor-mono-state");
    input::optional::ComponentWrapper::define("horfimbor-mono-input");
    auth::optional::ComponentWrapper::define("horfimbor-auth");
}
