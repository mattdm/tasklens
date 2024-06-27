#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/now")]
    #[redirect("/:.._segments", |_segments: Vec<String>| Route::Now {})]
    Now {},
    #[route("/today")]
    Today {},
}

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("Failed to initialize logger.");
    info!("starting");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Today() -> Element {
    todo!("One page at a time...")
}

#[component]
fn Now() -> Element {
    let mut text = use_signal(|| String::from("..."));

    info!("now");
    rsx! {

        sl-badge {
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

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from a badge!".to_string())
}
