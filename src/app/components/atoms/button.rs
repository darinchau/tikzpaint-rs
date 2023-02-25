//! The switch is a button that is either active or stale

use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;

#[derive(PartialEq)]
pub enum ButtonType {
    Submit,
    Reset,
    Other
}

#[derive(Clone)]
pub struct ButtonInfo {
    num_times_pressed: usize
}

#[derive(Properties, PartialEq)]
pub struct ButtonProperties{
    /// The callback is a function called right before the state change is triggered.
    /// It passes a mouse event and the number of times a button is pressed + 1 (i.e. what it would be after the press)
    pub cb: Option<Callback<(MouseEvent, ButtonInfo), ()>>,
    pub name: AttrValue,
    pub button_type: ButtonType,
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let cb = props.cb.clone().unwrap_or(Callback::from(|_| ()));
    let button_type = match props.button_type {
        ButtonType::Submit => "submit",
        ButtonType::Reset => "reset",
        ButtonType::Other => "button"
    };

    let num_times_pressed = use_state(|| 0_usize);
    let ntp = num_times_pressed.clone();

    let name = props.name.clone();

    html! {
        <button aria-label={name} type={button_type} onclick={Callback::from(move |x: MouseEvent| {
            let before_press_num_times = &*ntp;
            ntp.set(before_press_num_times + 1);
            let info = ButtonInfo {
                num_times_pressed: before_press_num_times + 1
            };
            cb.emit((x, info));
        })}>
            {for props.children.iter()}
        </button>
    }
}
