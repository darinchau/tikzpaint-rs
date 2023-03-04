#![allow(unused)]
//! Base module for Tikzpaint-rs. We try to structure the app such that
//! the "figures" module can also work as a CLI tool for Tikz figure generation

pub mod figures;
pub mod app;
pub mod renderer;
use yew::prelude::*;
use app::CanvasManager;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <CanvasManager header_height={60} side_bar_width={190} terminal_height={150} figure_dims={2} debug={true}/>
        </>
    }
}
