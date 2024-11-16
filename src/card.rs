#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::task;

use comrak::markdown_to_html;

use time::OffsetDateTime;

/// process the raw markdown input to html
async fn cook(raw: String) -> String {
    let mut markdown_options = comrak::Options::default();
    markdown_options.parse.smart = true;
    markdown_options.parse.relaxed_tasklist_matching = true;
    markdown_options.parse.relaxed_autolinks = true;
    markdown_options.render.escape = true;
    markdown_options.render.list_style = comrak::ListStyleType::Star;
    markdown_options.render.escaped_char_spans = true;
    markdown_options.extension.strikethrough = true;
    markdown_options.extension.autolink = true;
    markdown_options.extension.tasklist = true;
    markdown_options.extension.footnotes = true;
    markdown_options.extension.spoiler = true;

    markdown_to_html(t.raw, &markdown_options)
}

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

    let read_future = use_server_future(|| task::read(task_id))?;
    let task_data = match &*(read_future.read_unchecked()) {
        Some(Ok(t)) => task::TaskTable {
            raw: t.raw,
            ..Default::default()
        },
        Some(Err(e)) => task::TaskTable {
            raw: format!("# Error\n\n`{e:}`"),
            ..Default::default()
        },
        None => task::TaskTable {
            raw: format!("# Loading\n\n_Please wait."),
            ..Default::default()
        },
    };

    let mut raw = use_signal(|| task_data.raw.clone());
    let mut html = use_signal(|| String::new());

    let mut editing = use_signal(|| false);

    // cook the text (and later, save it here?)
    // TODO: don't cook more than every second, don't save more than once every 5

    let cook_future = use_server_future(|| task::cook(format!("{raw}")))?;
    let cooked = match &*(cook_future.read_unchecked()) {
        Some(Ok(c)) => *c,
        Some(Err(e)) => format!("<h1>Error</h1>\n\n<code>{e:?}</code>\n"),
        None => format!("<h1>Loading...</h1>\n\nPlease wait.\n"),
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
                            task::update( task::TaskInfo { task_id: task_id, raw: "{raw}", html: "{cooked}" }).await.unwrap();
                        },
                    rows: 20,
                    "{raw}"
                }
            },

            span { ondoubleclick: move |_| editing.set(true),
                class: "taskdetail",
                dangerous_inner_html: "{html}" // this is fine because comrak does html sanitization
            }

        }
    }
}
