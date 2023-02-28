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

#[derive(PartialEq)]
pub struct Transform {
    pub origin: (i32, i32),
    pub basis: Vec<(i32, i32)>
}

impl Transform {
    pub fn initialize_at_middle(top: i32, left: i32, bottom: i32, right: i32) {

    }

    pub fn to_world_coords(&self, client_x: i32, client_y: i32) {

    }
}

/// Dictates the height of the header and the maximum width of the side bar
#[derive(PartialEq, Properties)]
pub struct CanvasManagerProps {
    pub header_height: usize,
    pub side_bar_width: usize,
    pub terminal_height: usize,
    pub figure_dims: usize,
    pub debug: Option<bool>
}

fn get_css(props: &CanvasManagerProps) -> String {
    let debug_mode = is_true(props.debug);
    let h = props.header_height;
    let w = props.side_bar_width;
    let th = props.terminal_height;

    let button_css = format!(r#"
    {{
        bottom: {th}px;
        right: 0;
        width: calc(100% - {w}px);
        height: calc(100% - {h}px - {th}px);
    }}"#);

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
    let th = props.terminal_height;

    // Process figure and callbacks
    let dims = props.figure_dims;
    let fig = Figure::new(dims);
    let fig_state = use_mut_ref(|| fig);

    // We need to keep track of the world coordinates and figure coordinates conversion.
    // so we basically need to keep track of the transforms of this world. We need to keep track of
    // the position of (0, 0), (0, 1) and (1, 0)

    // Pass a unique ID down to the mouse sensor and use get_element_by_ID


    // Handles header bar events
    let header_fig = fig_state.clone();
    let header_cb = Callback::from(move |event: HeaderBarEvent| {
        let fig = header_fig.borrow_mut();
    });

    // Handles main canvas sensor events
    let canvas_sensor_fig = fig_state.clone();
    let canvas_sensor_cb = Callback::from(move |event: CanvasSensorEvent| {
        let fig = canvas_sensor_fig.borrow_mut();

        // We need to spawn a point. Thus we need do perform the following:
        // 1. Get the coordinates of the click. Transform that into the canvas coordinates
        // 2. Spawn a point at the canvas coordinates
        // 3. Pass the figure to the renderer and perform the rendering of the svg

        let mouse_event = event.mouse_click_event.mouse_event;
        let button = mouse_event.target().and_then(|b| b.dyn_into::<HtmlElement>().ok());

        let (x, y) = event.mouse_click_event.client_pos;
    });

    let sidebar_cb = Callback::from(move |event: SideBarEvent| {

    });

    let terminal_fig = fig_state.clone();
    let terminal_cb = Callback::from(move |event: TerminalEvent| {

    });

    // Process CSS
    let class_id = get_css(props);

    html!{
        <>
            <HeaderBar height={h} cb={header_cb}/>
            <SideBar header_height={h} width={w} cb={sidebar_cb}/>
            <Terminal height={th} text_box_height={37} sidebar_width={w} cb={terminal_cb}/>
            <div class={class_id}>
                <CanvasSensor top={h} left={w} cb={canvas_sensor_cb} id={"canvas-sensor"}/>
            </div>
        </>
    }
}
