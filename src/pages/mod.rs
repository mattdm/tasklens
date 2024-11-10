#![allow(non_snake_case)]
use dioxus::prelude::*;

use convert_case::{Case, Casing};

mod archive;
mod backlog;
mod done;
mod ideas;
mod now;
mod upnext;

use crate::route::Route;

pub use self::archive::Archive;
pub use self::backlog::Backlog;
pub use self::done::Done;
pub use self::ideas::Ideas;
pub use self::now::Now;
pub use self::upnext::UpNext;

#[component]
pub fn PageSelector() -> Element {
    let path: Route = use_route();

    // TODO: alternate for small screens
    rsx! {
        div { class: "pageselector",
              id: format!("{path}button").get(1..),
            for link in [Route::Ideas{},Route::Backlog{},Route::UpNext{},Route::Now{},Route::Done{},Route::Archive{}].iter() {
                Link { to: link.clone(), button { class: "pagebutton", class: if &path == link {"currentbutton"}, { format!("{link:?}").to_case(Case::Title) } } }
            }
        },

    }
}
