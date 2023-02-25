//! We make this component purely to sense and interpret the mouse events. This is to prevent making the button class too bloated

use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{GetProperty, Serializable};

pub enum MouseSensorEvent {
    Press(usize, usize),
}

#[derive(PartialEq, Properties)]
pub struct MouseSensorProps {
    cb: Callback<MouseSensorEvent>
}

#[function_component(MouseSensor)]
pub fn mouse_sensor(props: &MouseSensorProps) -> Html {
    let cb = props.cb.clone();

    html! {
        <button hidden={true} aria-label={"mouse sensor"} type={"button"} onclick={Callback::from(move |x: MouseEvent| {
            cb.emit((x, info));
        })}>
            {for props.children.iter()}
        </button>
    }
}
