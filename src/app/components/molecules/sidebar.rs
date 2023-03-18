//! Implementation of the top header bar of the app

use std::error::Error;
use std::fmt::Display;

use gloo::console::log;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{Button, ButtonType, ButtonEvent};
use paste::paste;

macro_rules! sidebar_type {
    ($($x:ident), *) => {
        #[derive(Clone, Copy, PartialEq, Debug)]
        pub enum SideBarType {
            $ (
                $x,
            )*
        }

        impl SideBarType {
            pub fn to_name(&self) -> &'static str {
                match &self {
                    $ (
                        SideBarType::$x => stringify!($x),
                    ) *

                    _ => "Unknown"
                }
            }
        }
    };
}

sidebar_type! {
    Point,
    Path,
    Move
}

impl Display for SideBarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_name())
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

    Callback::from(move |event: ButtonEvent| {
        let t = button_signal_emitter.emit(SideBarEvent {
            button_type: msg,
            event
        });
        return t;
    })
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
    // Make the CSS
    let h_style_name = sidebar_css(props);

    let id = props.id;

    html! {
        <div id={props.id} class={format!("sidebar {}", h_style_name)}>
            <div class={"grid"}>
                {wrap_button(props, SideBarType::Move)}
            </div>
            <div class={"sidebar-label"}>
                {"Objects"}
            </div>
            <div class={"grid"}>
                {wrap_button(props, SideBarType::Point)}
                {wrap_button(props, SideBarType::Path)}
            </div>
        </div>
    }
}
