use dioxus::prelude::*;
use dioxus_heroicons::IconButton;
use dioxus_heroicons::solid::Shape;

#[derive(Props, PartialEq, Clone)]
pub struct DeleteButtonProps {
    delete_id: usize,
}
#[component]
pub fn DeleteButton() -> Element {
//    let onclick = move |_| async move {
//        _ = delete_dog(props.delete_id).await;
//    };
    rsx! {
            IconButton {
  //              onclick: move |_| println!("Clicked"),
                class: "some-css-class",
                title: "Delete it",
                size: 30,
                icon: Shape::Trash,
            }
    //        button { id: "favorite-button-delete",  class: "favorites-delete-button","Delete"}
        }
}
