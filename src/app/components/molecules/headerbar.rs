//! Implementation of the top header bar of the app

use gloo::console::log;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{Button, ButtonType, ButtonEvent};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HeaderBarType {
    Help,
    Undo,
    Redo,
    About,
}

pub struct HeaderBarEvent {
    /// Header bar button is about the button that we pressed in the header bar
    pub button_type: HeaderBarType,

    /// Button info is the event emitted from the underlying button
    pub event: ButtonEvent,
}

#[derive(Properties, PartialEq)]
pub struct HeaderBarProps {
    pub height: usize,
    pub cb: Callback<HeaderBarEvent, ()>,
    pub debug: Option<bool>
}

const ABOUT_ICON: &'static str = include_str!("./headerbar/images/info.svg");
const REDO_ICON: &'static str = include_str!("./headerbar/images/redo.svg");
const UNDO_ICON: &'static str = include_str!("./headerbar/images/undo.svg");
const HELP_ICON: &'static str = include_str!("./headerbar/images/help.svg");

fn wrap_callback(props: &HeaderBarProps, msg: HeaderBarType) -> Callback<ButtonEvent> {
    let button_signal_emitter = (&props.cb).clone();

    let on_button = Callback::from(move |event: ButtonEvent| {
        let t = button_signal_emitter.emit(HeaderBarEvent {
            button_type: msg,
            event
        });
        return t;
    });
    return on_button;
}

#[function_component(HeaderBar)]
pub fn header_bar(props: &HeaderBarProps) -> Html {
    //Load the callbacks
    let on_undo = wrap_callback(props, HeaderBarType::Undo);
    let on_redo = wrap_callback(props, HeaderBarType::Redo);
    let on_about = wrap_callback(props, HeaderBarType::About);
    let on_help = wrap_callback(props, HeaderBarType::Help);

    // Load the icons
    let about = Html::from_html_unchecked(AttrValue::from(ABOUT_ICON));
    let redo = Html::from_html_unchecked(AttrValue::from(REDO_ICON));
    let undo = Html::from_html_unchecked(AttrValue::from(UNDO_ICON));
    let help = Html::from_html_unchecked(AttrValue::from(HELP_ICON));

    // Make the CSS
    let h = props.height.to_string();
    let height_style = Style::new(format!("height: {}px;", h))
        .unwrap_or_else(|_| {
            log!("Failed to load headbar height style");
            Style::new("").unwrap()
        });
    let h_style_name = height_style.get_class_name();

    html! {
        <div class={format!("topnav {}", h_style_name)}>
            <Button name={"about"} button_type={ButtonType::Other} cb={on_about}>
                {about}
            </Button>
            <Button name={"help"} button_type={ButtonType::Other} cb={on_help}>
                {help}
            </Button>
            <Button name={"redo"} button_type={ButtonType::Other} cb={on_redo}>
                {redo}
            </Button>
            <Button name={"undo"} button_type={ButtonType::Other} cb={on_undo}>
                {undo}
            </Button>
        </div>
    }
}
