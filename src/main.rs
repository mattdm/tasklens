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
            AddTaskButton {},
            PageSelector {},
            MenuButton {},
        }
    }
}

#[component]
fn AddTaskButton() -> Element {
    rsx! {
        button { class: "button-icon",
                 "➕"}
    }
}

#[component]
fn PageSelector() -> Element {
    rsx! {
        button { class: "selector-page",
                 "Past | Present | Future"}
    }
}

#[component]
fn MenuButton() -> Element {
    rsx! {
        button { class: "button-icon",
                 "☰"}
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer {}
    }
}

#[component]
fn Now() -> Element {
    let mut text = use_signal(|| String::from("..."));

    info!("now");
    rsx! {

        div {
            onclick: move |_| async move {
                if let Ok(data) = get_server_data().await {
                    tracing::info!("Client received: {}", data);
                    text.set(data.clone());
                    post_server_data(data).await.unwrap();
                }
            },
            "Hello",
        }

    }
}

#[component]
fn Today() -> Element {
    todo!("One page at a time...")
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
