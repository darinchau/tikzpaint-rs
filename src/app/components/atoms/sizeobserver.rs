//! A div that expands to nothing to keep track of the size of windows

use web_sys::HtmlElement;
use yew::prelude::*;
use gloo::console::log;
use stylist::Style;
use wasm_bindgen::{JsCast, prelude::Closure};

#[derive(Debug, Clone)]
pub struct Size {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Clone)]
pub struct WindowResizeEvent {
    pub old_size: Size,
    pub new_size: Size,
}

#[derive(PartialEq, Properties)]
pub struct WindowResizeListenerProps {
    pub id: AttrValue,
    pub cb: Option<Callback<WindowResizeEvent>>
}

const ASSUMPTION_X: i32 = 1920;
const ASSUMPTION_Y: i32 = 1080;

fn set_size(window_size: UseStateHandle<(i32, i32)>) {
        if let Some(window) = web_sys::window() {
        let width = match window.inner_width() {
            Ok(w) => w.dyn_into::<HtmlElement>().ok(),
            _ => None
        }
        .map(|x| {
                log!("Width inner text: ", x.inner_text());
                x.inner_text()
        })
        .map(|y| y.parse::<i32>().unwrap_or(ASSUMPTION_X))
        .unwrap_or(ASSUMPTION_X);

        let height = match window.inner_height() {
            Ok(h) => h.dyn_into::<HtmlElement>().ok(),
            _ => None
        }
        .map(|x| {
                log!("Height inner text: ", x.inner_text());
                x.inner_text()
        })
        .map(|y| y.parse::<i32>().unwrap_or(ASSUMPTION_Y))
        .unwrap_or(ASSUMPTION_Y);

        window_size.set((width, height));
    }
    else {
        log!("Failed to get windows from DOM");
    }
}

/// A component that expands to nothing and bubbles a component size observer event on resize
/// Assumes 1920 x 1080 if anything fails
#[function_component(WindowResizeListener)]
pub fn window_resize_listener(props: &WindowResizeListenerProps) -> Html {
    let window_size = use_state(|| (ASSUMPTION_X, ASSUMPTION_Y));

    if let Some(window) = web_sys::window() {
        set_size(window_size);

        // Add a resize callback event to windows
        let ws_clone = window_size.clone();

        let onresize = Closure::new(|| {
            log!("Resize event fired");
        }).as_ref().unchecked_ref();

        window.set_onresize(onresize);
    }
    else {
        log!("Failed to get windows from DOM - failed to initialize resize event");
    }

    let id = props.id.clone();

    html!{
        <div id={id}>
        </div>
    }
}
