//! The switch is a button that is either active or stale
use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::{HtmlElement, Text};
use wasm_bindgen::JsCast;
use crate::app::{GetProperty, Serializable, GetPropertyError};
use web_sys::HtmlInputElement;

pub struct TextFieldMessage {
    msg: Option<String>
}

#[derive(Properties, PartialEq)]
pub struct TextFieldProperties{
    pub name: AttrValue,
    pub label: AttrValue,
    /// The callback is a function called right before the state change is triggered.
    pub cb: Option<Callback<(Event, String), ()>>,
}

pub struct TextField {
    pub msg: String
}

impl Serializable for TextField {
    fn from_str(s: &str) -> Option<Self> {
        Some(TextField { msg: s.to_owned() })
    }

    fn into_str(&self) -> String {
        self.msg.clone()
    }
}

impl GetProperty for TextField {
    const NAME: &'static str = "TextField";

    fn get_component<T: Into<Event>>(event: T) -> Result<Self, GetPropertyError> {
        let x: Event = event.into();
        let input = x.target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(elem) = input {
            return Ok(TextField {
                msg: elem.value()
            })
        }

        GetPropertyError::err(String::from("The target component of this event is not a TextField"))
    }
}

impl Component for TextField {
    type Message = TextFieldMessage;
    type Properties = TextFieldProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        TextField {
            msg: String::new()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = (&ctx.props().cb)
            .clone()
            .unwrap_or(Callback::from(|_| ()));
        let link = ctx.link();
        let name = (&ctx.props().name).to_owned();
        let name2 = (&ctx.props().name).to_owned();
        let label = (&ctx.props().label).to_owned();
        html! {
            <>
                <label for={name}>{label}</label>
                <input type="text" name={name2} onchange={link.callback(move |x: Event| {
                    let input = x.target()
                        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                    if let Some(elem) = input {
                        cb.emit((x, elem.value()));
                        return TextFieldMessage {
                            msg: Some(elem.value())
                        }
                    }

                    TextFieldMessage {
                        msg: None
                    }
                })}/>
            </>
        }
    }
}


