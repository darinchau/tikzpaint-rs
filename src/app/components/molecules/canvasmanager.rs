//! ================================================================================== //
//! ============================= Main implementation ================================ //
//! ================================================================================== //

use gloo::console::log;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::figures::*;
use crate::app::{HeaderBarMessage, HeaderBar};

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
    let (header_height, side_bar_width, main_canvas_height, main_canvas_width) = {
        let h = props.header_height;
        let w = props.side_bar_width;
        let ch = AttrValue::from(format!("calc(100% - {}px)", h));
        let cw = AttrValue::from(format!("calc(100% - {}px", w));
        (h, w, ch, cw)
    };

    let dims = props.figure_dims;
    let fig = Figure::new(dims);
    let figure = use_state(|| fig);

    let cb = Callback::from(|(x, h): (MouseEvent, HeaderBarMessage)| {
        println!("{:?}", h);
    });

    html!{
        <>
            <HeaderBar height={header_height} on_button_clicked={cb}/>
            // <MainCanvas />
        </>
    }
}
