#![recursion_limit = "256"]

pub mod app;
pub mod shell;
pub mod components;
pub mod api;
pub mod routes;
pub mod context;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}