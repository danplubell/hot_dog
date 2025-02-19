use crate::components::routes::Route;
use dioxus::prelude::*;
use crate::components::tabs::{TabOption, Tabs};

#[component]
pub fn NavBar() -> Element {
    let tabs = [TabOption { label: "Dogs".into(), value: "dogs".into(), selected: true, component: rsx!{Link { to: Route::DogView }  }  },
    TabOption {label: "Favorites".into(), value: "favorites".into(), selected: false, component: rsx!{Link {to: Route::Favorites}}}].to_vec();
    rsx! {
        Tabs { id: "tabs", tabs}
        Outlet::<Route> {}
    }
}

/*
        div { id: "title",
            Link { to: Route::DogView,
                h1 { "🌭 HotDog! " }
            }
            Link { to: Route::Favorites, id: "heart", "♥️" } // <------- add this Link
        }
        Outlet::<Route> {}

 */