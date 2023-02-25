//! We make this component purely to sense and interpret the mouse events. This is to prevent making the button class too bloated

use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;

#[derive(PartialEq, Clone)]
pub struct MouseClickEvent {
    pub click_type: MouseClickType,
    pub client_pos: (i32, i32),
    pub screen_pos: (i32, i32)
}

#[derive(PartialEq, Clone, Copy)]
pub enum MouseClickType {
    AuxilaryClick,
    LeftClick,
    RightClick,
    DoubleLeftClick,
    MouseDown,
    MouseEnter,
    MouseMove,
    MouseOut,
    MouseUp,
    MouseLeave,
    MouseOver,
}

#[derive(PartialEq, Properties)]
pub struct MouseSensorProps {
    cb: Callback<MouseClickEvent>
}

fn get_callback(props: &MouseSensorProps, t: MouseClickType) -> Callback<MouseEvent> {
    let cb = props.cb.clone();
    let y = t.clone();
    Callback::from(move |event: MouseEvent| {
        let (x, y) = (event.client_x(), event.client_y());
        let (sx, sy) = (event.screen_x(), event.screen_y());
        cb.emit(MouseClickEvent {
            click_type: t,
            client_pos: (x, y),
            screen_pos: (sx, sy)
        });
    })
}

#[function_component(MouseSensor)]
pub fn mouse_sensor(props: &MouseSensorProps) -> Html {
    html! {
        <button hidden={true} aria-label={"mouse sensor"} type={"button"}
            onauxclick={get_callback(props, MouseClickType::AuxilaryClick)}
            onclick={get_callback(props, MouseClickType::LeftClick)}
            oncontextmenu={get_callback(props, MouseClickType::RightClick)}
            ondblclick={get_callback(props, MouseClickType::DoubleLeftClick)}
            onmousedown={get_callback(props, MouseClickType::MouseDown)}
            onmouseenter={get_callback(props, MouseClickType::MouseEnter)}
            onmouseleave={get_callback(props, MouseClickType::MouseLeave)}
            onmousemove={get_callback(props, MouseClickType::MouseMove)}
            onmouseout={get_callback(props, MouseClickType::MouseOut)}
            onmouseover={get_callback(props, MouseClickType::MouseOver)}
            onmouseup={get_callback(props, MouseClickType::MouseUp)}
            >
        </button>
    }
}
