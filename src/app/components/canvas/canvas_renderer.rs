//! We render the axis and figure separately
//! The height
use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct CanvasRendererProps {
    pub transform: Rc<RefCell<Transform>>,
    pub children: Children
}

#[function_component(CanvasRenderer)]
pub fn main_canvas(props: &CanvasRendererProps) -> Html {
    html! {
        <div id={"canvas-renderer"}>
            {for props.children.iter()}
        </div>
    }
}
