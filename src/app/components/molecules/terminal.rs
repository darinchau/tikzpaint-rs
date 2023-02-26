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

/// An Rc wrapped over a string
struct CheapString {
    ptr: Rc<String>,
}

impl Display for CheapString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self.ptr)
    }
}

impl Clone for CheapString {
    fn clone(&self) -> Self {
        CheapString { ptr: Rc::clone(&self.ptr) }
    }
}

impl CheapString {
    fn new(s: String) -> Self {
        CheapString { ptr: Rc::new(s) }
    }
}

impl PartialEq for CheapString {
    fn eq(&self, other: &Self) -> bool {
        return *self.ptr == *other.ptr;
    }
}

/// Use a wrapper for vec string because we want cheap cloning
/// This is for displaying the terminal text
struct TerminalText {
    ptr: Rc<RefCell<Vec<CheapString>>>,
}

impl TerminalText {
    pub fn new() -> Self {
        TerminalText {
            ptr: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn push(&self, s: String) -> &Self {
        self.ptr.borrow_mut().push(CheapString::new(s));
        return self;
    }

    pub fn unpack_to(&self) -> Vec<CheapString> {
        let mut v = vec![];
        for st in (*self.ptr.borrow()).iter() {
            v.push(st.clone())
        }
        v
    }
}

impl Clone for TerminalText {
    fn clone(&self) -> Self {
        TerminalText {
            ptr: Rc::clone(&self.ptr),
        }
    }
}

impl PartialEq for TerminalText {
    fn eq(&self, other: &Self) -> bool {
        return *self.ptr.borrow() == *other.ptr.borrow();
    }
}

// ================================================================================================================== //
// ========================================== Implementation of a terminal ========================================== //
// ================================================================================================================== //

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TerminalType {
    GotText,
}

pub struct TerminalEvent {
    /// Header bar button is about the button that we pressed in the header bar
    pub event_type: TerminalType,
}

#[derive(Properties, PartialEq)]
pub struct TerminalProps {
    pub height: usize,
    pub text_box_height: usize,
    pub sidebar_width: usize,
    pub cb: Callback<TerminalEvent, ()>,
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

fn get_callback(props: &TerminalProps, terminal_text: UseStateHandle<TerminalText>) -> Callback<TextFieldEvent, Option<String>> {
    // Handle callback
    let parent_cb = props.cb.clone();

    return Callback::from(move |mut x: TextFieldEvent| {
        match x.event {
            TextFieldEventType::Enter(_) => {
                // Get the terminal text
                let recieved_text = x.text;

                // Process the terminal text
                let tt = (*terminal_text).push(recieved_text);
                terminal_text.set(tt.clone());

                // Emit the parent callback
                let info = TerminalEvent {
                    event_type: TerminalType::GotText
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

fn wrap_terminal_text(v: TerminalText) -> Html {
    v.unpack_to().into_iter().map(|x| {
        html!{
            <>
                {x}
                <br/>
            </>
        }
    }).collect::<Html>()
}

#[function_component(Terminal)]
pub fn terminal(props: &TerminalProps) -> Html {

    let textbox_css = text_box_css(props);
    let terminal_css = terminal_css(props);

    let terminal_text = use_state(|| TerminalText::new());
    let terminal_txt1 = terminal_text.clone();
    let text_cb = get_callback(props, terminal_txt1);

    let terminal_texts_html = wrap_terminal_text((*terminal_text).clone());

    html! {
        <>
            <div class={format!("terminal-text {terminal_css}")}>
                {terminal_texts_html}
            </div>
            <div class={format!("terminal {textbox_css}")}>
                <TextField name={"terminal"} label={""} field_type={TextFieldInputType::Search} cb={text_cb}/>
            </div>
        </>
    }
}
