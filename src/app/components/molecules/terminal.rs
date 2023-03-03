//! Implementation of the top header bar of the app

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use gloo::console::log;
use stylist::Style;
use stylist::css;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::*;
use crate::figures::{CheapString, StringLike};

// ================================================================================================================== //
// ========================================== Implementation of a terminal ========================================== //
// ================================================================================================================== //

#[derive(Clone, PartialEq, Debug)]
pub enum TerminalType {
    GotText(CheapString),
}

pub struct TerminalEvent {
    /// Header bar button is about the button that we pressed in the header bar
    pub event_type: TerminalType,
}

#[derive(Properties, PartialEq)]
pub struct TerminalProps {
    pub id: &'static str,
    pub height: usize,
    pub text_box_height: usize,
    pub sidebar_width: usize,

    /// This callback should take in a terminal event, return the terminal text that we should render.
    pub cb: Callback<TerminalEvent>,

    /// The children will be rendered as terminal text
    /// Allows us to pass in terminal text and render
    /// We expect to process all terminal text in canvas manager
    pub children: Children,

    /// Enables debug mode if set to true. Currently does nothing
    pub debug: Option<bool>
}

fn text_box_css(props: &TerminalProps) -> String {
    // Set the height and width of the black box
    let h = props.height;

    // add 20 for padding
    let w = props.sidebar_width;
    let th = props.text_box_height;

    let padding = 10;

    let css_text = format!(r#"
        height: calc({th}px - {padding}px);
        width: calc(100% - {w}px - {padding}px - {padding}px);
        bottom: 0;
        right: 0;
        padding: {padding}px {padding}px {padding}px {padding}px;
    "#);

    let terminal_input_style = Style::new(css_text)
        .unwrap_or_else(|e| {
            log!(format!("{}", e));
            log!("Failed to load terminal text box dimensions style");
            Style::new("").unwrap()
        });
    let terminal_style = terminal_input_style.get_class_name().to_string();
    return terminal_style;
}

fn terminal_css(props: &TerminalProps) -> String {
    let h = props.height;
    let w = props.sidebar_width;
    let th = props.text_box_height;

    let padding = 10;

    let terminal_text_style = Style::new(format!(r#"
            height: calc({h}px - {th}px - {padding}px - {padding}px);
            width: calc(100% - {w}px - {padding}px - {padding}px);
            bottom: calc({th}px + {padding}px);
            right: 0;
            padding: {padding}px {padding}px 0px {padding}px;
        "#))
        .unwrap_or_else(|e| {
            log!(format!("{}", e));
            log!("Failed to load terminal black box dimensions style");
            Style::new("").unwrap()
        });
    let terminal_style = terminal_text_style.get_class_name().to_string();
    return terminal_style;
}

fn get_callback(props: &TerminalProps) -> Callback<TextFieldEvent, Option<String>> {
    // Handle callback
    let parent_cb = props.cb.clone();

    return Callback::from(move |mut x: TextFieldEvent| {
        match x.event {
            TextFieldEventType::Enter(_) => {
                // Get the terminal text
                let recieved_text = CheapString::new(x.text);

                // Emit the parent callback
                let info = TerminalEvent {
                    event_type: TerminalType::GotText(recieved_text.clone())
                };

                parent_cb.emit(info);

                // Set the terminal text back to nothing
                return Some(String::from(""));
            },

            TextFieldEventType::Change(_) => {
                let text = x.text;
                None
            },
        }
    })
}

#[function_component(Terminal)]
pub fn terminal(props: &TerminalProps) -> Html {

    let textbox_css = text_box_css(props);
    let terminal_css = terminal_css(props);

    let text_cb = get_callback(props);

    let id = props.id;

    html! {
        <>
            <div id={id} class={format!("terminal-text {terminal_css}")}>
                {for props.children.iter()}
            </div>
            <div id={"terminal-text-box"} class={format!("terminal {textbox_css}")}>
                <TextField id={"terminal-input"} name={"terminal"} label={""} field_type={TextFieldInputType::Search} cb={text_cb}/>
            </div>
        </>
    }
}
