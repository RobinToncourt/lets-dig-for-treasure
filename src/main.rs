#![allow(dead_code, clippy::cast_lossless)] // TODO: remove.

mod components;
mod engine;
mod net;

use log::info;

use crate::components::app::App;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    info!("Let's Dig for Treasure!");
    yew::Renderer::<App>::new().render();
}
