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
    pub id: &'static str,
    pub transform: Transform,
    pub children: Children
}

#[function_component(CanvasRenderer)]
pub fn main_canvas(props: &CanvasRendererProps) -> Html {
    let (t, r, b, l) = props.transform.margins;
    let w = format!("calc(100% - {r}px - {l}px)");
    let h = format!("calc(100% - {t}px - {b}px)");
    let id = props.id;

    html! {
        <div id={id}>
            <svg width={w} height={h}>
                {for props.children.iter()}
                {""}
            </svg>
        </div>
    }
}
