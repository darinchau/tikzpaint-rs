use yew::prelude::*;

pub enum Message {
    AddOne,
}

pub struct Button {
    value: usize,
}

impl Component for Button {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
                <button onclick={link.callback(|_| {
                    println!("Hehe haha!");
                    Message::AddOne
                })}>{
                    self.value
                }</button>
            </>
        }
    }
}