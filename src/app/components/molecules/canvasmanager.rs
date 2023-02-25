//! ================================================================================== //
//! ============================= Main implementation ================================ //
//! ================================================================================== //
//!
//! This component is responsible for the main operations of the wrapper app around tikzpaint. This calculates the svg,
//! and handles all clicks and translate them into actions on the figure.

use gloo::console::log;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::figures::*;
use crate::app::*;

/// Dictates the height of the header and the maximum width of the side bar
#[derive(PartialEq, Properties)]
pub struct CanvasManagerProps {
    pub header_height: usize,
    pub side_bar_width: usize,
    pub figure_dims: usize,
    pub debug: Option<bool>
}

struct MainCanvasSize {
    top: usize,
    left: usize,
    debug: Option<bool>
}

fn get_css(props: MainCanvasSize) -> String {
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

/// The main app is a coordinator component that coordinates all three main components
/// i.e. the header bar, the side bar, and the canvas
#[function_component(CanvasManager)]
pub fn canvas_manager(props: &CanvasManagerProps) -> Html {
    let debug_mode = is_true(props.debug);

    // Dimensions of the page
    let h = props.header_height;
    let w = props.side_bar_width;

    // Process figure
    let dims = props.figure_dims;
    let fig = Figure::new(dims);
    let fig_state = use_mut_ref(|| fig);

    let header_fig = fig_state.clone();
    let header_cb = Callback::from(move |event: HeaderBarEvent| {
        let fig = header_fig.borrow_mut();
    });

    let canvas_sensor_fig = fig_state.clone();
    let canvas_sensor_cb = Callback::from(move |event: CanvasSensorEvent| {
        let fig = canvas_sensor_fig.borrow_mut();
    });

    let sidebar_cb = Callback::from(move |event: SideBarEvent| {

    });

    // Process CSS
    let class_id = get_css(MainCanvasSize { top: h, left: w, debug: props.debug });

    html!{
        <>
            <HeaderBar height={h} cb={header_cb}/>
            <SideBar header_height={h} width={w} cb={sidebar_cb}/>
            <div class={class_id}>
                <CanvasSensor top={h} left={w} cb={canvas_sensor_cb}/>
            </div>
        </>
    }
}
