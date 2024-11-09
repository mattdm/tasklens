#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::*;

mod cards;
mod pages;
use pages::*;

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
fn AddTaskButton() -> Element {
    rsx! {
        button { class: "iconbutton",
                 "➕"}
    }
}

#[component]
fn PageSelector() -> Element {
    rsx! {
        div { class: "pageselector"}
            div { class: "erasection",
                //"Future",
                button { class: "pagebutton" , "Ideas"},
                button { class: "pagebutton" , "Backlog"},
                button { class: "pagebutton" , "Ready"},
            }
            div { class: "erasection",
                //"Present",
                button { class: "pagebutton" , "This Week"},
                PageButton { },
                Link { to: Route::Now {}, button { class: "pagebutton" , "Now"} },
            }
            div { class: "erasection",
                //"Past",
                button { class: "pagebutton" , "Done"},
                button { class: "pagebutton" , "Archive"},
                button { class: "pagebutton" , "Report"},
            }

    }
}

fn PageButton() -> Element {
    rsx! {
        Link { to: Route::Today {}, button { class: "pagebutton" , "Today"} },
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

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from a badge!".to_string())
}
