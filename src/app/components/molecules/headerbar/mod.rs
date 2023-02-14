//! Implementation of the top header bar of the app

use gloo::console::log;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{GetProperty, Serializable, Button, ButtonType};

pub enum HeaderBarMessage {
    Undo,
    Redo,
    About,
}

#[derive(Properties, PartialEq)]
pub struct HeaderBarProps {
    on_undo: Callback<MouseEvent, ()>,
    on_redo: Callback<MouseEvent, ()>,
    on_about: Callback<MouseEvent, ()>
}

pub struct HeaderBar {
    sty: Style,
}

const STYLES: &'static str = include_str!("headerbar.css");

impl Component for HeaderBar {
    type Message = HeaderBarMessage;
    type Properties = HeaderBarProps;

    fn create(ctx: &Context<Self>) -> Self {
        HeaderBar {  
            sty: Style::new(STYLES).unwrap_or_else(|_| {
                log!("Failed to load header bar style properties.");
                Style::new(css!()).unwrap()
            })
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_undo = (&ctx.props().on_undo).clone();
        let on_redo = (&ctx.props().on_redo).clone();
        let on_about = (&ctx.props().on_about).clone();

        let sty = self.sty.clone();

        html! {
            <div class={sty}>
                <Button name={"undo"} button_type={ButtonType::Other} cb={on_undo}>
                    {"Undo"}
                </Button>
            </div>
        }
    }
}

