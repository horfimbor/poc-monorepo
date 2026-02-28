mod admin;
mod state;

use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    yew::set_event_bubbling(false);
    admin::optional::ComponentWrapper::define("horfimbor-planet-admin");
    state::optional::ComponentWrapper::define("horfimbor-planet-state");
}
