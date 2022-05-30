use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::home::Home;
use crate::components::login::Login;
use crate::components::not_found::NotFound;
#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}
pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home/>},
        Route::Login => html! {<Login/>},
        Route::NotFound => html! {<NotFound/>},
    }
}
