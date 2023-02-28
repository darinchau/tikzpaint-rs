//! We render the axis and figure separately
use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::*;

#[derive(Properties, PartialEq)]
pub struct CanvasRendererProps {

}

#[function_component(CanvasRenderer)]
pub fn main_canvas(props: &CanvasRendererProps) -> Html {
    html! {
        <>
            <MouseSensor cb={mouse_sensor_cb} id={id}/>
        </>
    }
}
