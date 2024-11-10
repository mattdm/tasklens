#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::cards::TaskCard;

#[component]
pub fn Now() -> Element {
    // "now list only allows three tasks by definition
    // let now_list = use_signal(|| [i32; 3]);
    let now_list = use_signal(|| [1, 2, 3]);

    rsx! {
        article { class: "taskcolumn",
            for task in now_list.read() {
                TaskCard { task_id: task }
            }
        }
    }
}
