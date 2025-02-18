#![allow(non_snake_case)]

mod components;
mod backend;

use dioxus::prelude::*;
use crate::components::routes::Route;
use dioxus::logger::tracing::{Level};


static CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::logger::init(Level::DEBUG).expect("logger failed to init");
    dioxus::launch(App);
}

#[derive(Clone)]
pub(crate) struct TitleState(String);


#[component]
fn App() -> Element {
    use_context_provider(|| TitleState("HotDog".to_string()));
    rsx! {
        document::Stylesheet { href: CSS }
       Router::<Route> {}
    }
}

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        div { id: "title",
            h1 { "{title.0}" }
        }
    }
}

