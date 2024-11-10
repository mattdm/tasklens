#![allow(non_snake_case)]
use dioxus::prelude::*;
use tracing::*;

mod task;

mod card;
mod layouts;
mod pages;
mod route;

use route::Route;

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

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from a badge!".to_string())
}
