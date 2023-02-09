//! The switch is a button that is either active or stale

use stylist::Style;
use yew::prelude::*;

#[derive(Clone)]
pub enum SwitchState {
    Active,
    Stale
}

pub enum SwitchMessage {
    TurnOff,
    TurnOn
}

#[derive(Properties, PartialEq)]
pub struct SwitchProperties{
    pub cb: Option<Callback<MouseEvent, ()>>,
    pub children: Children,
}

pub struct Switch {
    state: SwitchState
}

impl Component for Switch {
    type Message = SwitchMessage;
    type Properties = SwitchProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Switch { state: SwitchState::Stale }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SwitchMessage::TurnOn => self.state = SwitchState::Active,
            SwitchMessage::TurnOff => self.state = SwitchState::Stale,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let children = &ctx.props().children;
        let cb = (&ctx.props().cb)
            .clone()
            .unwrap_or(Callback::from(|_| ()));
        let link = ctx.link();
        let state = self.state.clone();
        html! {
            <div>
                <button onclick={link.callback(move |x| {
                    cb.emit(x);
                    match state {
                        SwitchState::Active => SwitchMessage::TurnOff,
                        SwitchState::Stale => SwitchMessage::TurnOn,
                    }
                })}>
                    {for children.iter()}
                </button>
            </div>
        }
    }
}
