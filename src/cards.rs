#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn TaskCard() -> Element {
    rsx! {
        div { class: "taskcard", "TK: Task Card" }
    }
}
