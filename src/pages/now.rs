#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::cards::TaskCard;

#[component]
pub fn Now() -> Element {
    let now_list = use_signal(|| vec![1, 2, 3]);

    rsx! {
        article { class: "taskcolumn",
            for task in now_list.iter() {
                TaskCard { task_id: *task }
            }
        }
    }
}
