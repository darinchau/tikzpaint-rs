//! Implementation of the canvas sensor. This component does two things:
//! - Sense and interprets all button clicks
//! - Renders the svg

use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{MouseSensor, MouseClickEvent, MouseClickType, is_true};
use crate::figures::Figure;

#[derive(PartialEq, Clone)]
pub struct CanvasSensorEvent {
    pub dragging: bool,
    pub mouse_click_event: MouseClickEvent
}

#[derive(Properties, PartialEq)]
pub struct CanvasSensorProps {
    pub id: AttrValue,
    pub top: usize,
    pub left: usize,
    pub debug: Option<bool>,
    pub cb: Callback<CanvasSensorEvent>
}

#[function_component(CanvasSensor)]
pub fn main_canvas(props: &CanvasSensorProps) -> Html {
    // Parse main canvas dimensions
    let cb = props.cb.clone();

    let dragging_state = use_state(|| false);
    let mouse_sensor_cb = Callback::from(move |event: MouseClickEvent| {
        match event.click_type {
            MouseClickType::MouseDown => {dragging_state.set(true)},
            MouseClickType::MouseUp => {dragging_state.set(false)},
            _ => ()
        }

        cb.emit(CanvasSensorEvent {
            dragging: *dragging_state,
            mouse_click_event: event,
        });
    });

    let id = props.id.clone();

    html! {
        <>
            <MouseSensor cb={mouse_sensor_cb} id={id}/>
        </>
    }
}
