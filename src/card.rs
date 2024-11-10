#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn TaskCard(task_id: i32) -> Element {
    let mut title = use_signal(|| format!("Task {task_id}"));
    let mut detail = use_signal(|| "Detail TK".to_string());
    let mut edittitle = use_signal(|| false);
    let mut editdetail = use_signal(|| false);

    use_effect(move || println!("edittitle changed to {edittitle:?}"));
    use_effect(move || println!("edittext changed to {editdetail:?}"));

    rsx! {
            section { class: "taskcard",
                draggable: "true",
                if edittitle() {
                    input { onmounted: move |e| async move { _ = e.set_focus(true).await },
                            oninput: move |e| title.set(e.value()),
                            onblur: move |_| edittitle.set(false),
                            value: "{title}"
                        }
                } else {
                    h1 { ondoubleclick: move |_| edittitle.set(true),
                        "{title}"
                    }
                },
                if editdetail() {
                    textarea {onmounted: move |e| async move { _ = e.set_focus(true).await },
                        oninput: move |e| detail.set(e.value()),
                        onblur: move |_| editdetail.set(false),
                        "{detail}"
                    }
                }
                else {
                    p { ondoubleclick: move |_| editdetail.set(true),
                       "{detail}"
                    }
                },

        }
    }
}
