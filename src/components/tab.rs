use dioxus::prelude::*;
#[derive(PartialEq, Clone, Props)]
pub struct TabProps {
    id: String,
    label: String,
    onclick: EventHandler<MouseEvent>,
    selected: bool,

}

pub fn Tab(props: TabProps)-> Element {
    rsx!{
        button {id: props.id, class: "ids-tab", "aria-selected": props.selected,
             onclick: props.onclick, "{props.label}"}
//        <button class="ids-tab" aria-selected="true" tabIndex="-1">Tab One</button>
    }
}