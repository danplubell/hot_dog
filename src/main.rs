#![allow(non_snake_case)]

mod components;
mod backend;

use dioxus::document::Stylesheet;
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
        Stylesheet { href: asset!("assets/tailwind.css") }
        link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com"
        }
        link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            "crossorigin": ""
        }
        link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Roboto+Flex:opsz,slnt,wdth,wght,GRAD,XTRA,YOPQ,YTAS,YTDE,YTFI,YTLC,YTUC@8..144,-10..0,25..151,100..1000,-200..150,323..603,25..135,649..854,-305..-98,560..788,416..570,528..760&display=block"
        }

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

