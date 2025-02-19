use dioxus::logger::tracing::debug;
use dioxus::prelude::*;
use dioxus_heroicons::IconButton;
use dioxus_heroicons::solid::Shape;
use crate::backend::delete_dog;

#[derive(Props, PartialEq, Clone)]
pub struct DeleteButtonProps {
    delete_id: usize,
    refresh:  Resource<Result<Vec<(usize, String)>, ServerFnError>>,
}
#[component]
pub fn DeleteButton(mut props: DeleteButtonProps) -> Element {
    let onclick = move |_| async move {
        _ = delete_dog(props.delete_id).await;
        props.refresh.restart();
    };
    rsx! {
            IconButton {
                onclick: onclick,
                class: "some-css-class",
                title: "Delete it",
                size: 30,
                icon: Shape::Trash,
            }
    //        button { id: "favorite-button-delete",  class: "favorites-delete-button","Delete"}
        }
}
