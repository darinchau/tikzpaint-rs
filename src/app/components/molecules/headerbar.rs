//! Implementation of the top header bar of the app

use gloo::console::log;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{Button, ButtonType, ButtonInfo};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HeaderBarMessage {
    Help,
    Undo,
    Redo,
    About,
}

#[derive(Properties, PartialEq)]
pub struct HeaderBarProps {
    pub height: usize,
    pub on_button_clicked: Callback<(MouseEvent, HeaderBarMessage), ()>,
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

fn wrap_callback<T>(x: &Callback<(MouseEvent, HeaderBarMessage), T>, msg: HeaderBarMessage) -> Callback<(MouseEvent, ButtonInfo), T> where
T: 'static {
    let button_signal_emitter = x.clone();
    let on_button = Callback::from(move |(x, _): (MouseEvent, ButtonInfo)| {
        let t = button_signal_emitter.emit((x, msg));
        return t;
    });
    return on_button;
}

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
        let on_button_clicked = &ctx.props().on_button_clicked;
        let on_undo = wrap_callback(on_button_clicked, HeaderBarMessage::Undo);
        let on_redo = wrap_callback(on_button_clicked, HeaderBarMessage::Redo);
        let on_about = wrap_callback(on_button_clicked, HeaderBarMessage::About);
        let on_help = wrap_callback(on_button_clicked, HeaderBarMessage::Help);

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
