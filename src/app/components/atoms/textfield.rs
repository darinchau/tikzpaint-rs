//! The switch is a button that is either active or stale
use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::{HtmlElement, Text};
use wasm_bindgen::JsCast;
use crate::app::{GetProperty, Serializable, GetPropertyError};
use web_sys::HtmlInputElement;

pub enum TextFieldMessage {
    Change(String),
    Enter,
    None,
}

#[derive(Properties, PartialEq)]
pub struct TextFieldProperties{
    pub name: AttrValue,
    pub label: AttrValue,
    /// The callback is a function called right before the state change is triggered.
    pub onchange: Option<Callback<(Event, String), ()>>,
    pub ontypeenter: Option<Callback<KeyboardEvent, ()>>,
}

pub struct TextField {
    pub msg: Option<String>
}

impl Serializable for TextField {
    fn from_str(s: &str) -> Option<Self> {
        if s.len() >= 1 && s.starts_with("s") {
            return Some(TextField{
                msg: Some(String::from(&s[1..]))
            });
        }

        else if s.len() >= 1 && s.starts_with("n") {
            return Some(TextField{
                msg: None
            });
        }

        return None
    }

    fn into_str(&self) -> String {
        if let Some(s) = self.msg.clone() {
            return format!("s{}", s);
        }
        return String::from("n");
    }
}

impl GetProperty for TextField {
    const NAME: &'static str = "TextField";
}

impl Component for TextField {
    type Message = TextFieldMessage;
    type Properties = TextFieldProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        TextField {
            msg: None
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = (&ctx.props().onchange)
            .clone()
            .unwrap_or(Callback::from(|_| ()));
        let onenter = (&ctx.props().ontypeenter)
            .clone()
            .unwrap_or(Callback::from(|_| ()));
        let link = ctx.link();
        let name = (&ctx.props().name).to_owned();
        let name2 = (&ctx.props().name).to_owned();
        let label = (&ctx.props().label).to_owned();
        let property = self.property_html();
        html! {
            <div>
                {property}
                <label for={name}>{label}
                    <input 
                    type="text" 
                    name={name2} 
                    onchange={link.callback(move |x: Event| {
                        let input = x.target()
                            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                        if let Some(elem) = input {
                            onchange.emit((x, elem.value()));
                            return TextFieldMessage::Change(elem.value())
                        }

                        TextFieldMessage::None
                    })}
                    onkeydown={link.callback(move |x: KeyboardEvent| {
                        if x.key() == "Enter" {
                            onenter.emit(x);
                            return TextFieldMessage::Enter;
                        }

                        TextFieldMessage::None
                    })}/>
                </label>
            </div>
        }
    }
}
