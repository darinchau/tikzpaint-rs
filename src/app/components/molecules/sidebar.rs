//! Implementation of the top header bar of the app

use std::error::Error;

use gloo::console::log;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{Button, ButtonType, ButtonEvent};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SideBarType {
    Point,
    Test,
    Test2,
    Test3,
}

impl SideBarType {
    pub fn to_name(&self) -> &'static str {
        match &self {
            SideBarType::Point => "Point",
            SideBarType::Test => "Test",
            SideBarType::Test2 => "Test2",
            SideBarType::Test3 => "Test3",
            _ => "Unknown"
        }
    }
}

pub struct SideBarEvent {
    /// Side bar button is about the button that we pressed in the side bar
    /// This is identical to the current type of the side bar
    pub button_type: SideBarType,

    /// Button info is the event emitted from the underlying button
    pub event: ButtonEvent,
}

#[derive(Properties, PartialEq)]
pub struct SideBarProps {
    pub id: &'static str,
    pub header_height: usize,
    pub width: usize,
    pub cb: Callback<SideBarEvent, ()>,
    pub debug: Option<bool>
}

fn wrap_callback(props: &SideBarProps, msg: SideBarType) -> Callback<ButtonEvent> {
    let button_signal_emitter = (&props.cb).clone();

    let on_button = Callback::from(move |event: ButtonEvent| {
        let t = button_signal_emitter.emit(SideBarEvent {
            button_type: msg,
            event
        });
        return t;
    });
    return on_button;
}

fn sidebar_css(props: &SideBarProps) -> String {
    let h = props.header_height;
    let w = props.width;

    let padding = 10;

    let style = Style::new(format!(r#"
        top: {h}px;
        height: calc(100% - {h}px);
        width: calc({w}px - {padding}px - {padding}px);
        padding: {padding}px;
        "#))
        .unwrap_or_else(|e| {
            log!(format!("{}", e));
            log!("Failed to load sidebar dimensions style");
            Style::new("").unwrap()
        });
    let h_style_name = style.get_class_name().to_string();
    return h_style_name;
}

fn wrap_button(props: &SideBarProps, button_type: SideBarType) -> Html {
    let cb = wrap_callback(props, button_type);
    let name = button_type.to_name();
    let id = format!("sidebar-button-{}", button_type.to_name().to_lowercase());
    html!{
        <div class={"grid-item"}>
            <Button id={id} name={name} button_type={ButtonType::Other} cb={cb}>
                {name}
            </Button>
        </div>
    }
}

#[function_component(SideBar)]
pub fn side_bar(props: &SideBarProps) -> Html {
    //Load the callbacks
    let on_point = wrap_callback(props, SideBarType::Point);
    let on_test1 = wrap_callback(props, SideBarType::Test);
    let on_test2 = wrap_callback(props, SideBarType::Test2);
    let on_test3 = wrap_callback(props, SideBarType::Test3);

    // Make the CSS
    let h_style_name = sidebar_css(props);

    let id = props.id;

    html! {
        <div id={props.id} class={format!("sidebar {}", h_style_name)}>
            <div class={"sidebar-label"}>
                {"Some label"}
            </div>
            <div class={"grid"}>
                {wrap_button(props, SideBarType::Point)}
                {wrap_button(props, SideBarType::Test)}
                {wrap_button(props, SideBarType::Test2)}
                {wrap_button(props, SideBarType::Test3)}
            </div>
        </div>
    }
}
