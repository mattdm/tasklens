#![allow(non_snake_case)]
use crate::pages::PageSelector;
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Main() -> Element {
    rsx! {
        Header {}
        div { class: "pageholder",
            div {
                class: "page",
                Outlet::<Route> {}
            }
        }
        Footer {}

    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header {
            ContextSelector {},
            SearchBox {},
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer {
            AddTaskButton {},
            PageSelector {},
            MenuButton {},
        }
    }
}

#[component]
fn ContextSelector() -> Element {
    let task_context = use_signal(|| "Context".to_string());
    // TODO: make this a list of labels and loop through them....
    // TODO: click on contextcontainer to bring up context selector

    rsx! {
        div { class: "contextcontainer",
              span { class: "label",
                   "{task_context}",
              }
              span { class: "label",
                   "other",
              }
            }
    }
}

#[component]
fn SearchBox() -> Element {
    rsx! {
        div { class: "searchcontainer",
              input { class: "searchbox",
                      placeholder: "search..." }
        }
    }
}

#[component]
fn AddTaskButton() -> Element {
    rsx! {
        button { class: "iconbutton",
                 "➕"}
    }
}

#[component]
fn MenuButton() -> Element {
    rsx! {
        button { class: "iconbutton",
                 "☰"}
    }
}