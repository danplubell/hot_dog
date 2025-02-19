use crate::backend::save_dog;
use dioxus::prelude::*;

#[component]
pub fn DogCard(img_src: Resource<String>) -> Element {
    rsx! {
        div {
            id: "dog-card", class: "w-[340px] flex relative",
            div { id: "ids-card", class: "ids-card",
                div { id: "ids-card-layout", class: "ids-card-layout",
                    div {
                        id: "ids-card-media", class: "ids-card-media", img { src: img_src.cloned().unwrap_or_default() }
                    },
//                    div { id: "ids-card-body", class: "ids-card-body"
//                    }
                },
                div {
                    id: "ids-card-footer", class: "ids-card-footer",
                    button { onclick: move |_| img_src.restart(), id: "ids-link-skip", class: "ids-link ids-link-block", "Skip" }
                    button { onclick: move |_| async move {
                                let current = img_src.cloned().unwrap();
                                img_src.restart();
                                _ = save_dog(current).await;
                    },id: "ids-link-save", class: "ids-link ids-link-block", "Save"}
                }
            }
        }
    }
}
/*
<div class="w-[340px] flex relative">
  <div class="ids-card">
    <div class="ids-card-layout">
      <div class="ids-card-media">
        <img src="https://picsum.photos/id/1072/315/236" width="100%" />
      </div>
      <div class="ids-card-body">
        <div>Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>
      </div>
    </div>
    <div class="ids-card-footer">
      <button class="ids-link ids-link-block" data-ids-size="md">Action Label</button>
      <button class="ids-link ids-link-block" data-ids-size="md">Action Label</button>
    </div>
  </div>
</div>
 */
