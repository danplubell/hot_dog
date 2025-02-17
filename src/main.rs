#![allow(non_snake_case)]

mod components;
mod backend;

use crate::Route::DogView;
use crate::components::*;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}

#[derive(Clone)]
pub(crate) struct TitleState(String);

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    DogView,
}

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

