#![allow(non_snake_case)]
use crate::layouts;
use crate::pages::*;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(layouts::Main)]
    #[route("/now")]
    #[redirect("/:.._segments", |_segments: Vec<String>| Route::Now {})]
    Now {},
    #[route("/ideas")]
    Ideas {},
    #[route("/backlog")]
    Backlog {},
    #[route("/ondeck")]
    OnDeck {},
    #[route("/done")]
    Done {},
    #[route("/archive")]
    Archive {},
}
