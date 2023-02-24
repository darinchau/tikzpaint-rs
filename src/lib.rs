#![allow(unused)]
//! Base module for Tikzpaint-rs. We try to structure the app such that
//! the "figures" module can also work as a CLI tool for Tikz figure generation

pub mod figures;
pub mod app;
use gloo::console::log;
use yew::prelude::*;
use app::*;
use std::rc::Rc;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <CanvasManager header_height={60} side_bar_width={72} figure_dims={2}/>
        </div>
    }
}
