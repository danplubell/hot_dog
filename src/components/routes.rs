use crate::components::dog_view::DogView;
use crate::components::favorites::Favorites;
use crate::components::nav::NavBar;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,
}
