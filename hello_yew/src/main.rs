use yew::prelude::*;
mod components;
mod router;
use components::counter::Counter;
use crate::router::{Route,switch};
use yew_router::prelude::*;
#[derive(Properties, PartialEq, Clone)]
struct ListProp{
    pub name: String,
    pub data: Vec<u32>
}
#[function_component(Title)]
fn title() -> Html{
    html!{
        <h2>{"test"}</h2>
    }
}
#[function_component(List)]
fn list(prop: &ListProp) -> Html{
    html!{
    <ul class={&prop.name}>
        {
            prop.data.iter().map(|i|{
               html!{
                   <li>{i}</li>
               } 
            }).collect::<Html>()
        } 
       </ul>

    }
}

#[function_component(App)]
fn app() -> Html{
   html!{
       <>
       <Title/>
       <Counter/>
       <List data={vec![1,2,3]} name="hello"></List>
       <BrowserRouter>
       <Switch<Route> render={Switch::render(switch)}/>
       </BrowserRouter>
       </>
   } 
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
