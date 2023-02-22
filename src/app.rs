use yew::prelude::*;

mod components;
mod utils;

pub use components::atoms::switch::Switch;
pub use components::atoms::textfield::TextField;
pub use components::atoms::button::Button;
pub use components::atoms::button::ButtonType;

pub use components::molecules::headerbar::HeaderBar;
pub use components::molecules::maincanvas::MainCanvas;

pub use utils::getproperty::GetProperty;
pub use utils::getproperty::GetPropertyError;

pub use utils::filler_text::*;

// Reexport serializable since I don't want to break dependencies in this wrapper app
pub use crate::figures::Serializable;

// ================================================================================== //
// ============================= Main implementation ================================ //
// ================================================================================== //

use crate::figures::*;
/// The main app is a coordinator component that coordinates all three main components
pub struct CanvasManager {
    fig: Figure<2>,
}
