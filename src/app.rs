mod components;
mod utils;
mod core;

use components::atoms::switch::{Switch, SwitchEvent, SwitchState};
use components::atoms::text_field::{TextField, TextFieldEvent, TextFieldEventType, TextFieldInputType};
use components::atoms::button::{Button, ButtonType, ButtonEvent};
use components::atoms::mouse_sensor::{MouseSensor, MouseClickEvent, MouseClickType};
use components::atoms::size_observer::{WindowSize, WindowResizeListener, WindowResizeEvent, get_size, ASSUMPTION};

use components::canvas::canvas_sensor::{CanvasSensor, CanvasSensorEvent};
pub use components::canvas::canvas_manager::CanvasManager;
use components::canvas::canvas_renderer::{CanvasRenderer, CanvasRendererEvent};

use components::molecules::headerbar::{HeaderBar, HeaderBarType, HeaderBarEvent};
use components::molecules::sidebar::{SideBar, SideBarEvent, SideBarType};
use components::molecules::terminal::{Terminal, TerminalEvent, TerminalEventType, TerminalResetType, TerminalResetEvent};

use crate::app::core::figure_object_complex::{FigureComplex, FigureObjectComplex};
use crate::app::core::parser::{parse, initialize_parser, ParserError, ParserErrorType};
use crate::app::core::terminal_text_renderer::{TerminalTextRenderer};

use utils::util::*;

pub fn initialize_app() {
    initialize_parser()
}