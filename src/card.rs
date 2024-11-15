#![allow(non_snake_case)]
use dioxus::prelude::*;

use comrak::markdown_to_html;

use crate::task;

#[component]
pub fn TaskCard(task_id: i64) -> Element {
    // TODO: convert to  `let mut title = use_signal(|| task::load(task_id));

    //let mut task = task::load(task_id);
    let task_future = use_resource(move || task::load(task_id));

    let task_ref = task_future.read_unchecked();
    let task_data = match &*task_ref {
        Some(Ok(t)) => t,
        Some(Err(_e)) => todo!(),
        None => &task::Task::default(),
    };

    let mut title_raw = use_signal(|| task_data.title.clone());
    let mut detail_raw = use_signal(|| task_data.detail.clone());
    let mut title_cooked = use_signal(|| task_data.title_html.clone());
    let mut detail_cooked = use_signal(|| task_data.detail_html.clone());

    let mut editing = use_signal(|| false);

    // splits into title (first line without any leading #) and body (the rest without leading whitespace)
    let mut update_text = move |raw_text: String| {
        // TODO: don't run more than once in five seconds!
        let (new_title, new_detail) = match raw_text.split_once('\n') {
            Some((first_line, rest)) => (
                first_line
                    .trim_start_matches(|c: char| c == '#' || c.is_whitespace())
                    .trim_end()
                    .to_string(),
                rest.trim_start().to_string(),
            ),
            // If there's no newline, the entire input is the title
            None => (
                raw_text
                    .trim_start_matches(|c: char| c == '#' || c.is_whitespace())
                    .trim_end()
                    .to_string(),
                String::new(),
            ),
        };
        title_raw.set(new_title);
        detail_raw.set(new_detail);
    };

    // TODO make this saved and configurable
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

    // magic! this gets called one and then again when the captured signals get changed.
    // BUG -- dioxus is warning about writing to signals during a render. I think it _might_
    // be okay but need to check.
    title_cooked.set(markdown_to_html(
        &use_memo(move || title_raw()).to_string(),
        &markdown_options,
    ));
    detail_cooked.set(markdown_to_html(
        &use_memo(move || detail_raw()).to_string(),
        &markdown_options,
    ));

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
                    onblur: move |_| editing.set(false),
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
