#![allow(unused)]
use gloo::console::log;
use yew::prelude::*;
use tikzpaint_rs::app::*;

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
            <HeaderBar height={100} on_about={on_about} on_help={on_help} on_undo={on_undo} on_redo={on_redo}/>
            {FILLER1}
            {FILLER2}
            {FILLER3}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
