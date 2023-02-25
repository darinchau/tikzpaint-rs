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
use crate::app::{HeaderBarType, HeaderBar, HeaderBarEvent, CanvasSensor, CanvasSensorEvent};

/// Dictates the height of the header and the maximum width of the side bar
#[derive(PartialEq, Properties)]
pub struct CanvasManagerProps {
    pub header_height: usize,
    pub side_bar_width: usize,
    pub figure_dims: usize,
}

/// The main app is a coordinator component that coordinates all three main components
/// i.e. the header bar, the side bar, and the canvas
#[function_component(CanvasManager)]
pub fn canvas_manager(props: &CanvasManagerProps) -> Html {
    // Dimensions of the page
    let h = props.header_height;
    let w = props.side_bar_width;

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

    html!{
        <>
            <HeaderBar height={h} cb={header_cb}/>
            <CanvasSensor top={h} left={w} svg_content={""} cb={canvas_sensor_cb}/>
        </>
    }
}
