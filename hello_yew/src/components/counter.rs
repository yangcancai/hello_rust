use yew::prelude::*;

pub struct Counter{
    n: u32
}
#[derive(Debug)]
pub enum Msg {
   Add(u32),
   None
}
impl Component for Counter{
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self{
        Self{n:0}
    }
   fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
      log::debug!("update:{}, msg={:?}",self.n, msg);
       if let Msg::Add(i) = msg{
                self.n += i;
           if self.n % 2 == 0{
                true
           }else{
                false
           }
       }else{
           false
       }
       
    }
    fn view(&self, ctx: &Context<Self>) -> Html{
        log::debug!("view:{}",self.n);
        let link = ctx.link(); 
        let onclick = link.callback(|_|Msg::Add(1));
        let cb = link.callback(|msg: Msg| msg);
         cb.emit(Msg::None);
                html!{
            <>
              <p>{"kkkk"}</p>
              <button onclick={onclick}>
              {"+1"}
              </button>
              <p>{self.n}</p>
            </>
        }
    }
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        log::debug!("rendered:{}",self.n);
    }
}