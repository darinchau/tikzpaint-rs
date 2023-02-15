//! The switch is a button that is either active or stale

use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{GetProperty, Serializable};

#[derive(PartialEq)]
pub enum ButtonType {
    Submit,
    Reset,
    Other
}

pub enum ButtonMessage {
    Press,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProperties{
    /// The callback is a function called right before the state change is triggered.
    /// The input parameters is the mouse event and the state of the switch **before** the press
    pub name: AttrValue,
    pub button_type: ButtonType,
    pub cb: Option<Callback<MouseEvent, ()>>,
    pub children: Children,
}

pub struct Button {
    /// Number of times pressed
    pub val: usize
}

impl Serializable for Button {
    fn into_str(&self) -> String {
        self.val.to_string()
    }

    fn from_str(s: &str) -> Option<Self> {
        if let Some(num) = str::parse::<usize>(s).ok() {
            return Some(Button {
                val: num
            });
        }

        None
    }
}

impl GetProperty for Button {
    const NAME: &'static str = "Button";
}

impl Component for Button {
    type Message = ButtonMessage;
    type Properties = ButtonProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Button { val: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ButtonMessage::Press => self.val += 1,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let children = &ctx.props().children;
        let cb = (&ctx.props().cb)
            .clone()
            .unwrap_or(Callback::from(|_| ()));
        let link = ctx.link();
        let properties = self.property_html();
        let t = match &ctx.props().button_type {
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
            ButtonType::Other => "button"
        };
        html! {
            <button type={t} onclick={link.callback(move |x| {
                cb.emit(x);
                ButtonMessage::Press
            })}>
                {properties}
                {for children.iter()}
            </button>
        }
    }
}



