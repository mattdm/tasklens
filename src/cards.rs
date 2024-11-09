#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn TaskCard() -> Element {
    let mut edittitle = use_signal(|| false);
    let mut edittext = use_signal(|| false);

    use_effect(move || println!("edittitle changed to {edittitle:?}"));
    use_effect(move || println!("edittext changed to {edittext:?}"));

    rsx! {
            section { class: "taskcard",
                h1 { onclick: move |_| edittitle.set(true), contenteditable: if edittitle() { "true" }, "Title TK" },
                p { onclick: move |_| edittext.set(true), contenteditable: if edittext() { "true" }, "Task text TK" },

        }
    }
}
