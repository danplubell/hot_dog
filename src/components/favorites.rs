use crate::backend::list_dogs;
use dioxus::prelude::*;
use crate::components::delete_button::DeleteButton;

#[component]
pub fn Favorites() -> Element {
    // Create a pending resource that resolves to the list of dogs from the backend
    // Wait for the favorites list to resolve with `.suspend()`
    let mut favorites_resource = use_resource(list_dogs);
    let favorites = favorites_resource.suspend()?;
    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id, url) in favorites().unwrap() {
                    // Render a div for each photo using the dog's ID as the list key
                    div {
                        key: id,
                        class: "favorite-dog",
                        img { src: "{url}" },
                        DeleteButton {delete_id: id, refresh: favorites_resource }
                    }
                }
                button{onclick: move |_| {favorites_resource.restart()}, "Refresh"}
            }
        }
    }
}
