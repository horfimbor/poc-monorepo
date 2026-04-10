mod admin;
mod available_building;
mod state;
mod visual;

use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    yew::set_event_bubbling(false);
    admin::optional::ComponentWrapper::define("horfimbor-planet-admin");
    state::optional::ComponentWrapper::define("horfimbor-planet-state");
    visual::optional::ComponentWrapper::define("horfimbor-planet-visual");
}
