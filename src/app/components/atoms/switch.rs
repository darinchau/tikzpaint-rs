//! The switch is a button that is either active or stale

use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::app::{GetProperty, Serializable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwitchState {
    Active,
    Stale
}

#[derive(Debug)]
pub struct SwitchInfo {
    _state: UseStateHandle<SwitchState>,
}

impl SwitchInfo {
    pub fn get_state(&self) -> SwitchState {
        let state = self._state.clone();
        return (&*state).clone();
    }

    pub fn set_state(&mut self, state: SwitchState) {
        self._state.set(state);
    }
}

#[derive(Properties, PartialEq)]
pub struct SwitchProperties{
    pub name: AttrValue,

    /// We will change the active state for you if you want to set it manually somewhere else
    pub active: Option<SwitchState>,

    /// The callback is a function called right before the state change is triggered.
    /// The input parameters is the mouse event and the state of the switch **after** the press
    pub cb: Option<Callback<(MouseEvent, SwitchInfo), ()>>,
    pub children: Children,
}

#[function_component(Switch)]
pub fn switch(props: &SwitchProperties) -> Html {
    let cb = props.cb.clone().unwrap_or(Callback::from(|_| ()));

    let state = use_state(|| SwitchState::Stale);
    if let Some(s) = props.active {
        state.set(s);
    }

    let state_construct = state.clone();

    html! {
        <button type={"button"} aria-label={"switch"} onclick={Callback::from(move |x| {
            let info = SwitchInfo {
                _state: state_construct.clone()
            };
            cb.emit((x, info));
        })}>
            {for props.children.iter()}
        </button>
    }
}
