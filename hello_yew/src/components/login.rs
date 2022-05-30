use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Login;

impl Component for Login {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().history().unwrap();
        let onclick = Callback::once(move |_| {
            history.push(Route::Home);
        });
        html! {
            <button {onclick}>{"To Home"}</button>
        }
    }
}
