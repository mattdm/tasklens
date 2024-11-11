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

    // splits into title (first line without any leading #) and body (the rest without leading whitespace)
    let mut update_text = move |raw_text: String| {
        let (new_title, new_body) = match raw_text.split_once('\n') {
            Some((first_line, rest)) => (
                first_line
                    .trim_start_matches(|c: char| c == '#' || c.is_whitespace())
                    .trim_end()
                    .to_string(),
                rest.trim_start().to_string(),
            ),
            // If there's no newline, the entire input is the title
            None => (raw_text.to_string(), String::new()),
        };
        title.set(new_title);
        detail.set(new_body);
    };

    let mut check_finished = move |k: Event<KeyboardData>| {
        match (k.key(), {
            k.modifiers().contains(Modifiers::CONTROL) || k.modifiers().contains(Modifiers::SHIFT)
        }) {
            (Key::Enter, true) | (Key::Escape, _) => editing.set(false),
            (_, _) => { // nothing
            }
        }
    };

    rsx! {
        section { ondoubleclick: move |_| editing.set(true),
            class: "taskcard",
            draggable: if !editing() { "true" },
            h1 { ondoubleclick: move |_| editing.set(true),
                "{title}"
            },

            if editing() {
                textarea {
                    onmounted: move |e: MountedEvent | async move { _ = e.set_focus(true).await },
                    onkeyup: move |e: KeyboardEvent| { check_finished(e)},
                    oninput: move |e: FormEvent| { update_text(e.value()) },
                    onblur: move |_| editing.set(false),
                    rows: 20,
                    "# {title}\n\n{detail}"
                }
            },

            p { ondoubleclick: move |_| editing.set(true),
                "{detail}"
            }

        }
    }
}
