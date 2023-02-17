//! Implementation of the main canvas part of the app. This is mostly a wrapper around a figure.
//! The sensor and renderer is separated as their own higher-order component.

use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{GetProperty, Button, ButtonType, Serializable};
use crate::figures::Figure;

#[derive(PartialEq, Debug, Clone)]
pub struct MousePosition(usize, usize);

#[derive(Properties, PartialEq)]
pub struct SensorCanvasProps {
    pub top: usize,
    pub left: usize,
    pub debug: Option<bool>,
    pub svg_content: Html
}

fn get_css(props: &SensorCanvasProps) -> String {
    let debug_mode = props.debug.is_some() && props.debug.unwrap();
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

// We use higher order components as a workaround for not having hooks and direct access to internal states simultaneously
#[function_component(SensorCanvasPair)]
fn sensor_canvas(props: &SensorCanvasProps) -> Html {
    let debug_mode = props.debug.is_some() && props.debug.unwrap();

    // Parse main canvas dimensions
    let class_id = get_css(props);

    // We are using a button under a canvas as a mouse sensor. Get the position data from a button underneath the svg.
    let pos_state = use_state(|| MousePosition(0, 0));
    let pos_state_getter = pos_state.clone();
    let cb = Callback::from(move |x: MouseEvent| {
        pos_state_getter.set(MousePosition(
            if x.screen_x() >= 0 {x.screen_x() as usize} else {0},
            if x.screen_y() >= 0 {x.screen_y() as usize} else {0},
        ));
    });

    let svg_content = props.svg_content.clone();

    html! {
        <div class={class_id}>
            <svg>
                {svg_content}
            </svg>
            <Button name={"canvas sensor"} button_type={ButtonType::Other} cb={cb}>
                {format!("{:?}", *pos_state)}
            </Button>
        </div>
    }
}

// Implementation of main canvas
#[derive(PartialEq, Debug, Clone)]
pub enum ClickType {
    Point,
}
#[derive(PartialEq, Debug, Clone)]
pub struct MainCanvasMessage {
    position: MousePosition,
    click: ClickType,
}

#[derive(Properties, PartialEq)]
pub struct MainCanvasProps {
    pub top_px_offset: usize,
    pub left_px_offset: usize,
    pub debug: Option<bool>
}

pub struct MainCanvas {

}


impl Component for MainCanvas {
    type Message = MainCanvasMessage;
    type Properties = MainCanvasProps;

    fn create(ctx: &Context<Self>) -> Self {
        MainCanvas {  }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let top = (&ctx.props()).top_px_offset;
        let left = (&ctx.props()).left_px_offset;
        let deb = (&ctx.props()).debug;

        // TODO: write rendering magic - convert figure into svg
        let svg_content = html!();

        html! {
            <SensorCanvasPair top={top} left={left} debug={deb} svg_content={svg_content}/>
        }
    }
}
