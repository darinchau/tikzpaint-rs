//! The switch is a button that is either active or stale
use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::{HtmlElement, Text};
use wasm_bindgen::JsCast;
use crate::app::{GetProperty, Serializable, GetPropertyError};
use web_sys::HtmlInputElement;

#[derive(Clone, Debug)]
pub enum TextFieldEvent {
    Change(Event),
    Enter(KeyboardEvent),
}

#[derive(Clone, Debug)]
pub struct TextFieldInfo {
    pub event: TextFieldEvent,
    _state: UseStateHandle<String>
}

impl TextFieldInfo {
    pub fn get_text(&self) -> String {
        return (&*self._state).clone();
    }

    pub fn set_text(&mut self, text: String) {
        self._state.set(text);
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum TextFieldInputType {
    Button,
    Checkbox,
    Color,
    Date,
    DateTimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
}

impl TextFieldInputType {
    pub fn to_string(&self) -> &'static str {
        match self {
            TextFieldInputType::Button => {"button"},
            TextFieldInputType::Checkbox => {"checkbox"},
            TextFieldInputType::Color => {"color"},
            TextFieldInputType::Date => {"date"},
            TextFieldInputType::DateTimeLocal => {"datetime-local"},
            TextFieldInputType::Email => {"email"},
            TextFieldInputType::File => {"file"},
            TextFieldInputType::Hidden => {"hidden"},
            TextFieldInputType::Image => {"image"},
            TextFieldInputType::Month => {"month"},
            TextFieldInputType::Number => {"number"},
            TextFieldInputType::Password => {"password"},
            TextFieldInputType::Radio => {"radio"},
            TextFieldInputType::Range => {"range"},
            TextFieldInputType::Reset => {"reset"},
            TextFieldInputType::Search => {"search"},
            TextFieldInputType::Submit => {"submit"},
            TextFieldInputType::Tel => {"tel"},
            TextFieldInputType::Text => {"text"},
            TextFieldInputType::Time => {"time"},
            TextFieldInputType::Url => {"url"},
            TextFieldInputType::Week => {"week"},
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TextFieldProps{
    pub name: AttrValue,
    pub label: AttrValue,
    pub field_type: TextFieldInputType,
    /// The callback is a function called after the state is triggered but before rerender
    pub cb: Option<Callback<TextFieldInfo, ()>>,
}

#[function_component(TextField)]
pub fn text_field(props: &TextFieldProps) -> Html {
    let cb = props.cb.clone().unwrap_or(Callback::from(|_| ()));
    let cb2 = cb.clone();

    let name = props.name.clone();
    let name2 = props.name.clone();
    let label = props.label.clone();
    let ftype = props.field_type.to_string();

    let state = use_state(|| String::new());
    let state_1 = state.clone();

    html! {
        <label for={name}>{label}
            <input type={ftype} name={name2}
            onchange={Callback::from(move |x: Event| {
                let input = x.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(elem) = input {
                    let info = TextFieldInfo {
                        event: TextFieldEvent::Change(x),
                        _state: state.clone(),
                    };

                    cb.emit(info);
                }
            })}

            onkeydown={Callback::from(move |x: KeyboardEvent| {
                if x.key() == "Enter" {
                    let info = TextFieldInfo {
                        event: TextFieldEvent::Enter(x),
                        _state: state_1.clone(),
                    };

                    cb2.emit(info);
                }
            })}/>
        </label>
    }
}