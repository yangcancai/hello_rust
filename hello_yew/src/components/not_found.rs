use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;
pub struct NotFound;

impl Component for NotFound {
    type Message = ();
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <Link<Route> to={Route::Home}>{"To Home"}</Link<Route>>
            <p> {"Not Found 404"}</p>
            </>
        }
    }
}
