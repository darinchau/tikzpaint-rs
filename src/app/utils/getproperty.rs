//! A trait for getting the internal state in my app components, because it is often cumbersome to do so

use gloo::console::log;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;

use crate::app::Serializable;

#[allow(dead_code)]
pub struct GetPropertyError {
    message: String
}

impl GetPropertyError {
    pub fn err<T>(s: String) -> Result<T, GetPropertyError> {
        Err(GetPropertyError { message: s})
    }
}

pub trait GetProperty where
Self: Component + Serializable {
    const NAME: &'static str;

    fn get_component<T: Into<Event>>(event: T) -> Result<Self, GetPropertyError> {
        if let Some(content) = get_html(event) {
            let c = content.get_elements_by_tag_name("properties")
            .item(0)
            .unwrap()
            .inner_html();

            let beginning = format!("Type={}", Self::NAME);

            if c.starts_with(&beginning) {
                let s = &c[beginning.len()..];
                if let Some(obj) = Self::from_str(s) {
                    return Ok(obj)
                }
            }

            Err(GetPropertyError{
                message: format!("The target of the event is not of type {}", Self::NAME),
            })
        }
        else {
            GetPropertyError::err(String::from("The target of the event is not a component"))
        }
    }

    fn property_html(&self) -> Html {
        html! {
            <properties hidden=true>
                {format!("Type={}", Self::NAME)}
                {self.into_str()}
            </properties>
        }
    }
}

fn get_html<T: Into<Event>>(event: T) -> Option<HtmlElement> {
    let e: Event = event.into();
    let content = e.target()
        .and_then(|t| t.dyn_into::<HtmlElement>().ok());
    return content;
}

