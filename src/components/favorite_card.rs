use dioxus::prelude::*;
use crate::backend::delete_dog;

#[component]
pub fn FavoriteCard(img_src: String, delete_id: usize, refresh:  Resource<Result<Vec<(usize, String)>, ServerFnError>>) -> Element {
    let onclick = move |_| async move {
        _ = delete_dog(delete_id).await;
        refresh.restart();
    };

    rsx!{
                div {
            id: "dog-card", class: "w-[340px] flex relative",
            div { id: "ids-card", class: "ids-card",
                div { id: "ids-card-layout", class: "ids-card-layout",
                    div {
                        id: "ids-card-media", class: "ids-card-media", img { src: img_src }
                    },
                },
                div {
                    id: "ids-card-footer", class: "ids-card-footer",
                    button { onclick: onclick, id: "ids-link-delete", class: "ids-link ids-link-block", "Delete" }
                }
            }
        }

    }
}