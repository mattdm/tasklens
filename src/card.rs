#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::task;

#[component]
pub fn TaskCard(task_id: i64) -> Element {
    // TODO: convert to  `let mut title = use_signal(|| task::load(task_id));

    //let mut task = task::load(task_id);
    // let task_future = use_server_future(move || task::read(task_id))?;
    // let task_ref = task_future.read_unchecked();
    // let task_data = match &*task_ref {
    //     Some(Ok(t)) => t,
    //     Some(Err(_e)) => &task::TaskInfo::default(), // FIXME: handle error
    //     None => unreachable!("This shouldn't happen."),
    // };

    //let result = use_server_future(|| task::load(task_id));
    let result = use_server_future(|| task::read(task_id))?;
    let foo = result.value();
    // let whygodwhy = result.read_unchecked();
    // let task_data = match &*whygodwhy {
    //     Some(Ok(t)) => task::TaskInfo {
    //         task_id: t.task_id,
    //         raw: t.raw,
    //         html: t.html,
    //     },
    //     Some(Err(e)) => task::TaskInfo {
    //         task_id: 0,
    //         raw: format!("Error: {e:}"),
    //         html: format!("<h1>Error</h1>\n\n<code>{e:?}</code>\n"),
    //     },
    //     None => task::TaskInfo {
    //         task_id: 0,
    //         raw: format!("Loading"),
    //         html: format!("<h1>Loading...</h1>\n\nPlease wait.\n"),
    //     },
    // };

    let mut raw = use_signal(|| task_data.raw.clone());
    let mut html = use_signal(|| task_data.html.clone());

    let mut editing = use_signal(|| false);

    // cook the text (and later, save it here?)
    // TODO: don't cook more than every second, don't save more than once every 5
    let mut update_text = move |raw_text: String| {
        // TODO: don't run more than once in five seconds!
        match use_server_future(|| task::cook(raw)).value() {
            Some(Ok(t)) => html.set(t.html),
            Some(Err(e)) => html.set(format!("<h1>Error</h1>\n\n<code>{e:?}</code>\n")),
            None => {}
        };
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
                class: "tasktitle",
                dangerous_inner_html: "{title_cooked}" // this is fine because comrak does html sanitization
            },

            if editing() {
                textarea {
                    onmounted: move |e: MountedEvent | async move { _ = e.set_focus(true).await },
                    onkeyup: move |e: KeyboardEvent| { check_finished(e)},
                    oninput: move |e: FormEvent| { update_text(e.value()) },
                    onblur: move |_| async move { editing.set(false);
                            task::update( task_id, title_raw(), detail_raw(), title_cooked(), detail_cooked()).await.unwrap();
                        },
                    rows: 20,
                    "# {title_raw}\n\n{detail_raw}\n"
                }
            },

            span { ondoubleclick: move |_| editing.set(true),
                class: "taskdetail",
                dangerous_inner_html: "{detail_cooked}" // this is fine because comrak does html sanitization
            }

        }
    }
}
