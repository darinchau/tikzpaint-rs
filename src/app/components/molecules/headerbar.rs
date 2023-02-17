//! Implementation of the top header bar of the app

use gloo::console::log;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{GetProperty, Serializable, Button, ButtonType};

pub enum HeaderBarMessage {
    Help,
    Undo,
    Redo,
    About,
}

#[derive(Properties, PartialEq)]
pub struct HeaderBarProps {
    pub height: usize,
    pub on_undo: Callback<MouseEvent, ()>,
    pub on_redo: Callback<MouseEvent, ()>,
    pub on_about: Callback<MouseEvent, ()>,
    pub on_help: Callback<MouseEvent, ()>
}

pub struct HeaderBar {
    about_icon: AttrValue,
    redo_icon: AttrValue,
    undo_icon: AttrValue,
    help_icon: AttrValue
}

const ABOUT_ICON: &'static str = include_str!("./headerbar/images/info.svg");
const REDO_ICON: &'static str = include_str!("./headerbar/images/redo.svg");
const UNDO_ICON: &'static str = include_str!("./headerbar/images/undo.svg");
const HELP_ICON: &'static str = include_str!("./headerbar/images/help.svg");

impl Component for HeaderBar {
    type Message = HeaderBarMessage;
    type Properties = HeaderBarProps;

    fn create(ctx: &Context<Self>) -> Self {
        HeaderBar {
            about_icon: AttrValue::from(ABOUT_ICON),
            redo_icon: AttrValue::from(REDO_ICON),
            undo_icon: AttrValue::from(UNDO_ICON),
            help_icon: AttrValue::from(HELP_ICON),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_undo = (&ctx.props().on_undo).clone();
        let on_redo = (&ctx.props().on_redo).clone();
        let on_about = (&ctx.props().on_about).clone();
        let on_help = (&ctx.props().on_help).clone();

        let about = Html::from_html_unchecked(self.about_icon.clone());
        let redo = Html::from_html_unchecked(self.redo_icon.clone());
        let undo = Html::from_html_unchecked(self.undo_icon.clone());
        let help = Html::from_html_unchecked(self.help_icon.clone());

        let h = (&ctx.props()).height.to_string();

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
}

