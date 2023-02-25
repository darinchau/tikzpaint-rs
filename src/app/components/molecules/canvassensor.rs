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
    dragging: bool
}

#[derive(Properties, PartialEq)]
pub struct CanvasSensorProps {
    pub top: usize,
    pub left: usize,
    pub debug: Option<bool>,
    pub svg_content: AttrValue,
    pub cb: Callback<CanvasSensorEvent>
}

fn get_css(props: &CanvasSensorProps) -> String {
    let debug_mode = is_true(props.debug);
    let topbar_height_px = props.top.to_string();
    let sidebar_width_px = props.left.to_string();

    let button_css = format!(r#"
    {{
        bottom: 0;
        right: 0;
        width: calc(100% - {}px);
        height: calc(100% - {}px);
    }}"#, sidebar_width_px, topbar_height_px);

    let svg_css = button_css.clone();

    let main_canvas_pos = Style::new(format!("& button {} & svg {}", button_css, svg_css))
        .unwrap_or_else(|_| {
            log!("Failed to load main canvas position style");
            Style::new("").unwrap()
        });

    if debug_mode {
        format!("main-canvas-debug {}", main_canvas_pos.get_class_name())
    }
    else {
        format!("main-canvas {}", main_canvas_pos.get_class_name())
    }
}

fn process_svg(s: AttrValue) -> String {
    return String::new();
}

#[function_component(CanvasSensor)]
pub fn main_canvas(props: &CanvasSensorProps) -> Html {
    // Parse main canvas dimensions
    let class_id = get_css(props);

    let cb = props.cb.clone();

    let dragging_state = use_state(|| false);
    let mouse_sensor_cb = Callback::from(move |event: MouseClickEvent| {
        match event.click_type {
            MouseClickType::MouseDown => {dragging_state.set(true)},
            MouseClickType::MouseUp => {dragging_state.set(false)},
            _ => ()
        }

        cb.emit(CanvasSensorEvent {
            dragging: *dragging_state
        });
    });

    let svg_content = process_svg(props.svg_content.clone());

    html! {
        <div class={class_id}>
            <svg>
                {svg_content}
            </svg>
            <MouseSensor cb={mouse_sensor_cb}/>
        </div>
    }
}
