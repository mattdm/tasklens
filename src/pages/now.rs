#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::cards::*;

#[component]
pub fn Now() -> Element {
    rsx! {
        article { class: "taskcolumn",
            TaskCard {}
            TaskCard {}
            TaskCard {}
        }
    }
}
