#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::task;

#[component]
pub fn TaskCard(task_id: i32) -> Element {
    // TODO: convert to  `let mut title = use_signal(|| task::load(task_id));

    let mut task = task::load(task_id);

    let mut title = use_signal(|| task.title);
    let mut detail = use_signal(|| task.details);
    let mut editing = use_signal(|| false);

    // fn update_text(e: Event<FormData>) {
    //     detail.set(e.value());
    // }

    rsx! {
        section { ondoubleclick: move |_| editing.set(true),
            class: "taskcard",
            draggable: if !editing() { "true" },
            h1 { ondoubleclick: move |_| editing.set(true),
                "{title}"
            },

            if editing() {
                textarea {
                    onmounted: move |e| async move { _ = e.set_focus(true).await },
                    oninput: move |e| { detail.set(e.value()) },
                    onblur: move |_| editing.set(false),
                    rows: 20,
                    "# {title}\n{detail}"
                }
            },

            p { ondoubleclick: move |_| editing.set(true),
                "{detail}"
            }

        }
    }
}
