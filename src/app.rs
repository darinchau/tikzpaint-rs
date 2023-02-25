use yew::prelude::*;

mod components;
mod utils;

pub use components::atoms::switch::Switch;
pub use components::atoms::textfield::TextField;
pub use components::atoms::button::Button;
pub use components::atoms::button::ButtonType;
pub use components::atoms::button::ButtonInfo;

pub use components::molecules::headerbar::HeaderBar;
pub use components::molecules::headerbar::HeaderBarMessage;
pub use components::molecules::maincanvas::MainCanvas;

pub use utils::getproperty::GetProperty;
pub use utils::getproperty::GetPropertyError;

pub use utils::filler_text::*;

// ================================================================================== //
// ============================= Main implementation ================================ //
// ================================================================================== //

use crate::figures::*;

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

