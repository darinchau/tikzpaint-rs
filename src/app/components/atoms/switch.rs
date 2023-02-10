//! The switch is a button that is either active or stale

use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{GetProperty, Serializable};

#[derive(Clone, Debug)]
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
    /// The callback is a function called right before the state change is triggered.
    /// The input parameters is the mouse event and the state of the switch **before** the press
    pub cb: Option<Callback<MouseEvent, ()>>,
    pub children: Children,
}

pub struct Switch {
    pub state: SwitchState
}

impl Switch {
    pub fn new() -> Switch {
        Switch { state: SwitchState::Stale }
    }
}

impl Serializable for Switch {
    fn into_str(&self) -> String {
        match self.state {
            SwitchState::Active => String::from("A"),
            SwitchState::Stale => String::from("S"),
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        if s == "A" {
            return Some(Switch {
                state: SwitchState::Active
            });
        }
        else if s == "S" {
            return Some(Switch {
                state: SwitchState::Stale
            });
        }
        else {
            None
        }
    }
}

impl GetProperty for Switch {
    const NAME: &'static str = "Switch";
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
        let properties = self.property_html();
        html! {
            <button onclick={link.callback(move |x| {
                cb.emit(x);
                match state {
                    SwitchState::Active => SwitchMessage::TurnOff,
                    SwitchState::Stale => SwitchMessage::TurnOn,
                }
            })}>
                {properties}
                {for children.iter()}
            </button>
        }
    }
}


