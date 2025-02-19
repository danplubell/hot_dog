use dioxus::prelude::*;
use crate::backend::{save_dog, DogApi};
use crate::components::dog_card::DogCard;

#[component]
pub fn DogView() -> Element {


    let img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });
    rsx! {
        div { id: "dog_view_container", class: "w-full h-full flex justify-center items-center",
            DogCard {img_src}
        }
    }
}

/*
    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button { onclick: move |_| async move {
                let current = img_src.cloned().unwrap();
                img_src.restart();
                _ = save_dog(current).await;
            }, id: "save", "save!" }
        }
    }

 */