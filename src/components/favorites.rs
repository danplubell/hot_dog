use crate::backend::list_dogs;
use crate::components::delete_button::DeleteButton;
use crate::components::favorite_card::FavoriteCard;
use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    // Create a pending resource that resolves to the list of dogs from the backend
    // Wait for the favorites list to resolve with `.suspend()`
    let favorites_resource = use_resource(list_dogs);
    let favorites = favorites_resource.suspend()?;
    rsx! {
        div { id: "favorites", class: "container flex flex-row p-2 items-start flex-wrap gap-2",
            for (id, url) in favorites().unwrap() {
                FavoriteCard {img_src: "{url}", delete_id: id, refresh: favorites_resource}
            }
        }
    }
}

/*
                   div {
                       key: id,
                       class: "favorite-dog",
                       img { src: "{url}" },
                       DeleteButton {delete_id: id, refresh: favorites_resource }
                   }

*/
