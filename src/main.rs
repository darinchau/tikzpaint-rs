#![allow(unused)]
use gloo::console::log;
use yew::prelude::*;
use tikzpaint_rs::app::*;
use std::rc::Rc;

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

    // Create all the proportions for stuff
    let header_height: usize = 60;
    let side_bar_width: usize = 72;
    let main_canvas_height = AttrValue::from(format!("calc(100% - {}px)", header_height));
    let main_canvas_width = AttrValue::from(format!("calc(100%-{}px", side_bar_width));

    html! {
        <div>
            <HeaderBar height={header_height} on_about={on_about} on_help={on_help} on_undo={on_undo} on_redo={on_redo}/>
            <MainCanvas top_px_offset={header_height} left_px_offset={side_bar_width}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

