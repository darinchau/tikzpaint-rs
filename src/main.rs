#![allow(unused)]
use gloo::console::log;
use yew::prelude::*;
use tikzpaint_rs::app::{Switch, TextField, GetProperty, Button, ButtonType, HeaderBar};

#[function_component(App)]
fn app() -> Html {
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

    html! {
        <div>
            <HeaderBar on_about={on_about} on_help={on_help} on_undo={on_undo} on_redo={on_redo}/>
            {""}
        <div/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
