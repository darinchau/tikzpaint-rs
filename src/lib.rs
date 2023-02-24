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
    let on_about = Callback::from(|x: MouseEvent| {
        log!("Pressed about");
    });
    let on_help = Callback::from(|x: MouseEvent| {
        log!("Pressed help");
    });
    let on_undo = Callback::from(|x: MouseEvent| {
        log!("Pressed undo");
    });
    let on_redo = Callback::from(|x: MouseEvent| {
        log!("Pressed redo");
    });

    // Create all the proportions for stuff
    let header_height: usize = 60;
    let side_bar_width: usize = 72;
    let main_canvas_height = AttrValue::from(format!("calc(100% - {}px)", header_height));
    let main_canvas_width = AttrValue::from(format!("calc(100%-{}px", side_bar_width));

    html! {
        <div>
            <HeaderBar height={header_height} on_about={on_about} on_help={on_help} on_undo={on_undo} on_redo={on_redo}/>
        </div>
    }
}
