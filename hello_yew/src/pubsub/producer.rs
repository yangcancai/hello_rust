use crate::pubsub::event_bus::{EventBus, Publish};
use web_sys::HtmlInputElement;
use yew::{html::IntoPropValue, prelude::*};
use yew_agent::{Dispatched, Dispatcher};
pub struct Producer {
    event_bus: Dispatcher<EventBus>,
    input: String,
    r: NodeRef,
}
pub enum Msg {
    Input(String),
    Button,
    MouseOver,
}
impl Producer {
    fn apply_focus(&self) {
        if let Some(input) = self.r.cast::<HtmlInputElement>() {
            input.focus().unwrap();
        }
    }
}
impl Component for Producer {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            event_bus: EventBus::dispatcher(),
            input: "".to_string(),
            r: NodeRef::default(),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Input(input) => self.input = input,
            Msg::Button => {
                self.event_bus
                    .send(Publish::EventBusMsg(self.input.clone()));
            }
            Msg::MouseOver => {
                self.apply_focus();
            }
        }
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Button);
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::Input(input.value())
        });
        html! {
            <>
            <input
            type="text"
            {oninput}
            ref={self.r.clone()}
            placeholder="some message to publish"
            onmouseover={ctx.link().callback(|_|{
                Msg::MouseOver
            })}/>
            <button {onclick}>{"Publish message"}</button>
            </>
        }
    }
}
