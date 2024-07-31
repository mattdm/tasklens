#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[layout(Main)]
    #[route("/now")]
    #[redirect("/:.._segments", |_segments: Vec<String>| Route::Now {})]
    Now {},
    #[route("/today")]
    Today {},
}

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("Failed to initialize logger.");

    // Run the server on something other than the default 8080
    let cfg = server_only!(
        dioxus::fullstack::Config::new().addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8074)))
    );

    info!("starting");
    LaunchBuilder::fullstack().with_cfg(cfg).launch(App)
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Main() -> Element {
    rsx! {
        Header {}
        div {
            class: "page",
            Outlet::<Route> {}
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
fn AddTaskButton() -> Element {
    rsx! {
        button { class: "iconbutton",
                 "➕"}
    }
}

#[component]
fn PageSelector() -> Element {
    rsx! {
        button { class: "pageselector",
                 "Past | Present | Future"}
    }
}

#[component]
fn MenuButton() -> Element {
    rsx! {
        button { class: "iconbutton",
                 "☰"}
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
fn Now() -> Element {
    print!("um");
    rsx! {
        TaskCard {}
        TaskCard {}
        TaskCard {}
    }
}

#[component]
fn Today() -> Element {
    todo!("One page at a time...")
}

#[component]
fn TaskCard() -> Element {
    rsx! {
        div { class: "taskcard", "one" }
        div { class: "taskcard", "two" }
        div { class: "taskcard", "three" }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from a badge!".to_string())
}
