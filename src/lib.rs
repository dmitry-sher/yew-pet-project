#![recursion_limit = "512"]

mod app;
mod utils;
mod routes;

use yew::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();
    utils::set_panic_hook();
    web_logger::init();
    App::<routes::Model>::new().mount_to_body();
    yew::run_loop();
    Ok(())
}
