use crate::components::tab::Tab;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct TabsProps {
    id: String,
    tabs: Vec<TabOption>,
}

#[derive(PartialEq, Clone)]
pub struct TabOption {
    pub label: String,
    pub value: String,
    pub(crate) selected: bool,
    pub(crate) component: Element,
}
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let mut selected = use_signal(|| String::from("none"));
    rsx! {
        div { id: props.id, class: "ids-tabs-container",
            div { class: "ids-tab-scroller",
                for tab in props.tabs {
                    {
                        let tab_value = tab.value.clone();
                        rsx! {
                            Tab { 
                                key: tab_value.clone(), 
                                id: tab_value.clone(), 
                                label: tab.label, 
                                selected: tab.value == selected(),
                                onclick: move |_| selected.set(tab.value.to_string())
                            }
                        }
                    }
                }
            }
        }
    }
}

/*
<div class="ids-tabs-container">
  <div class="ids-tabs-scroller">
    <button class="ids-tab" aria-selected="true" tabIndex="-1">Tab One</button>
    <button class="ids-tab">Tab Two</button>
    <button class="ids-tab">Tab Three</button>
    <button class="ids-tab">Tab Four</button>
  </div>
</div>
 */
