#![allow(non_snake_case)]

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
#[component]
fn App() -> Element {
    use_context_provider(|| TitleState("HotDog".to_string()));
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
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
#[component]
fn DogView() -> Element {
    
    
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });
    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button { onclick: move |_| img_src.restart(), id: "save", "save!" }
        }

    }
}
/*
this example uses a signal for the img_src
fn DogView() -> Element {
    let mut img_src = use_signal(|| "".to_string());
    
    let skip = move |evt| {};
    let fetch_new = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();

        img_src.set(response.message); // fire signal
    };
    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: fetch_new, id: "save", "save!" }
        }

    }
}

 */