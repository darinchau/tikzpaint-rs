//! The switch is a button that is either active or stale
use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Clone, Debug)]
pub enum TextFieldEventType {
    Change(Event),
    Enter(KeyboardEvent),
}

#[derive(Clone, Debug)]
pub struct TextFieldEvent {
    pub event: TextFieldEventType,
    pub text: String
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
    /// ID field of the Text field
    pub id: AttrValue,

    pub name: AttrValue,
    pub label: AttrValue,
    pub field_type: TextFieldInputType,
    /// The callback is a function called after the state is triggered but before rerender
    /// The Option String return type is to reset the text box. If the option is none, then
    /// leave the text box as is. But if it is something, then set the text box value to that
    /// new string.
    pub cb: Option<Callback<TextFieldEvent, Option<String>>>,
}

fn get_set_state(event: TextFieldEventType, state: UseStateHandle<String>, text: String, cb: Callback<TextFieldEvent, Option<String>>) {
    let text_clone = text.clone();

    let info = TextFieldEvent {
        event,
        text
    };

    if let Some(rt) = cb.emit(info) {
        state.set(rt);
    }
    else {
        state.set(text_clone);
    }
}

// These state handles and callbacks are internally all Rc's so we can sprinkle clone() everywhere
// without having to worry tooooo much.
fn get_callback(props: &TextFieldProps, state: UseStateHandle<String>) -> (Callback<Event>, Callback<KeyboardEvent>) {
    let cb = (props.cb).clone().unwrap_or(Callback::from(|_| None));

    let cb1 = cb.clone();
    let s1 = state.clone();

    let on_change = Callback::from(move |x: Event| {
        let input = x.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(elem) = input {
            get_set_state(TextFieldEventType::Change(x), s1.clone(), elem.value(), cb1.clone());
        }
    });

    let cb2 = cb.clone();
    let s2 = state.clone();

    let on_keyboard = Callback::from(move |x: KeyboardEvent| {
        let input = x.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(elem) = input {
            if x.key() == "Enter" {
                get_set_state(TextFieldEventType::Enter(x), s2.clone(), elem.value(), cb2.clone());
            }
        }
    });

    return (on_change, on_keyboard);
}

#[function_component(TextField)]
pub fn text_field(props: &TextFieldProps) -> Html {
    let name = props.name.clone();
    let name2 = props.name.clone();
    let label = props.label.clone();
    let ftype = props.field_type.to_string();

    let state = use_state(|| String::new());
    let (on_change, on_keyboard) = get_callback(props, state.clone());

    let id = props.id.clone();

    html! {
        <label for={name}>{label}
            <input id={id} type={ftype} name={name2}
            value={format!("{}", *state)}
            onchange={on_change}
            onkeydown={on_keyboard}/>
        </label>
    }
}