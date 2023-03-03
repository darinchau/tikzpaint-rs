//! A div that expands to nothing to keep track of the size of windows

use web_sys::HtmlElement;
use yew::prelude::*;
use gloo::console::log;
use stylist::Style;
use wasm_bindgen::prelude::*;
use crate::app::*;

#[derive(Debug, Clone)]
pub struct WindowSize {
    pub x: i32,
    pub y: i32
}

impl WindowSize {
    fn new(x: i32, y: i32) -> Self {
        return WindowSize {
            x, y
        };
    }
}

#[derive(Debug, Clone)]
pub struct WindowResizeEvent {
    pub old_size: WindowSize,
    pub new_size: WindowSize,
}

#[derive(PartialEq, Properties)]
pub struct WindowResizeListenerProps {
    pub cb: Option<Callback<WindowResizeEvent>>
}



pub const ASSUMPTION: (i32, i32) = (1920, 1080);

pub fn get_size() -> Result<(i32, i32), &'static str> {
    if let Some(window) = web_sys::window() {
        let width_ = window.inner_width()
            .ok()
            .and_then(|x| {
                let y = jsvalue_to_string(x).unwrap_or_else(|x| x);
                let res = y.parse::<i32>().ok();
                return res;
            });

        if width_.is_none() {
            return Err("Failed to parse width value - failed to get size");
        }

        let width = width_.unwrap();

        let height_ = window.inner_height()
            .ok()
            .and_then(|x| {
                let y = jsvalue_to_string(x).unwrap_or_else(|x| x);
                let res = y.parse::<i32>().ok();
                return res;
            });

        if height_.is_none() {
            return Err("Failed to parse height value - failed to get size");
        }

        let height = height_.unwrap();

        return Ok((width, height));
    }

    return Err("Failed to get windows from DOM - failed to get size");
}

/// A component that expands to nothing and bubbles a component size observer event on resize
/// Assumes 1920 x 1080 if anything fails
pub struct WindowResizeListener {
    current_x: i32,
    current_y: i32,
}

pub enum WindowResizeListenerMessage {
    ResizeEvent,
}

impl Component for WindowResizeListener {
    type Message = WindowResizeListenerMessage;
    type Properties = WindowResizeListenerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let resize_listener = Closure::wrap(Box::new(move || {
            link.send_message(WindowResizeListenerMessage::ResizeEvent);
        }) as Box<dyn FnMut()>);

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("resize", resize_listener.as_ref().unchecked_ref())
            .unwrap();

        resize_listener.forget();

        let (x, y) = get_size().unwrap_or_else(|x| {
            log!(x);
            ASSUMPTION
        });

        WindowResizeListener{
            current_x: x,
            current_y: y
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let cb = (&ctx.props().cb).clone().unwrap_or(Callback::from(|_| ()));
        match msg {
            WindowResizeListenerMessage::ResizeEvent => {
                match get_size() {
                    Ok((x, y)) => {
                        let info = WindowResizeEvent{
                            old_size: WindowSize::new(self.current_x, self.current_y),
                            new_size: WindowSize::new(x, y)
                        };
                        cb.emit(info);
                        self.current_x = x;
                        self.current_y = y;
                    },
                    Err(e) => {
                        log!(e);
                    }
                }

            }
        };

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{}
    }
}
