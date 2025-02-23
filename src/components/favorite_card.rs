use crate::backend::delete_dog;
use dioxus::prelude::*;

#[component]
pub fn FavoriteCard(
    img_src: String,
    delete_id: usize,
    refresh: Resource<Result<Vec<(usize, String)>, ServerFnError>>,
) -> Element {
    let onclick = move |_| async move {
        _ = delete_dog(delete_id).await;
        refresh.restart();
    };

    rsx! {
        div {
            id: "dog-card", class: "w-[340px] h-[400px] flex relative",
            div { id: "ids-card", class: "ids-card justify-between w-full",
                div { id: "ids-card-layout", class: "ids-card-layout h-full",
                    div {
                        id: "ids-card-media", class: "ids-card-media h-full",
                        img { class: "h-full object-contain", src: img_src }
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
