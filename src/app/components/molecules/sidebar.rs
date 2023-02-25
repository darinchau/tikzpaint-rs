//! Implementation of the top header bar of the app

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

pub struct SideBarEvent {
    /// Side bar button is about the button that we pressed in the side bar
    /// This is identical to the current type of the side bar
    pub button_type: SideBarType,

    /// Button info is the event emitted from the underlying button
    pub event: ButtonEvent,
}

#[derive(Properties, PartialEq)]
pub struct SideBarProps {
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

#[function_component(SideBar)]
pub fn side_bar(props: &SideBarProps) -> Html {
    //Load the callbacks
    let on_point = wrap_callback(props, SideBarType::Point);
    let on_test1 = wrap_callback(props, SideBarType::Test);
    let on_test2 = wrap_callback(props, SideBarType::Test2);
    let on_test3 = wrap_callback(props, SideBarType::Test3);


    // Make the CSS
    let style = Style::new(format!(r#"top: {}px; height: calc(100% - {}px); width: {}px;"#, props.header_height, props.header_height, props.width))
        .unwrap_or_else(|e| {
            log!(format!("{}", e));
            log!("Failed to load sidebar dimensions style");
            Style::new("").unwrap()
        });
    let h_style_name = style.get_class_name();

    html! {
        <div class={format!("sidebar {}", h_style_name)}>
            <div class={"sidebar-label"}>
                {"Some label"}
            </div>
            <div class={"grid"}>
                <div class={"grid-item"}>
                    <Button name={"point"} button_type={ButtonType::Other} cb={on_point}>
                        {"Point"}
                    </Button>
                </div>
                <div class={"grid-item"}>
                    <Button name={"Test 1"} button_type={ButtonType::Other} cb={on_test1}>
                        {"Test 1"}
                    </Button>
                </div>
                <div class={"grid-item"}>
                    <Button name={"Test 2"} button_type={ButtonType::Other} cb={on_test2}>
                        {"Test 2"}
                    </Button>
                </div>
                <div class={"grid-item"}>
                    <Button name={"Test 3"} button_type={ButtonType::Other} cb={on_test3}>
                        {"Test 3"}
                    </Button>
                </div>
            </div>
        </div>
    }
}
