#![allow(non_snake_case)]
use dioxus::prelude::*;

mod task;

mod card;
mod layouts;
mod pages;
mod route;

use route::Route;

fn main() {
    #[cfg(feature = "web")]
    tracing_wasm::set_as_global_default();

    #[cfg(feature = "server")]
    tracing_subscriber::fmt::init();

    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
