#![allow(non_snake_case)]
use dioxus::prelude::*;

mod archive;
mod backlog;
mod done;
mod ideas;
mod now;
mod ondeck;

use crate::route::Route;

pub use self::archive::Archive;
pub use self::backlog::Backlog;
pub use self::done::Done;
pub use self::ideas::Ideas;
pub use self::now::Now;
pub use self::ondeck::OnDeck;

#[component]
pub fn PageSelector() -> Element {
    let path: Route = use_route();
    rsx! {
        div { class: "pageselector",
              id: "{path}",
            for link in [Route::Ideas{},Route::Backlog{},Route::OnDeck{},Route::Now{},Route::Done{},Route::Archive{}].iter() {
                Link { to: link.clone(), button { class: "pagebutton", class: if &path == link {"currentbutton"}, "{link}"} }
            }
        },

    }
}
