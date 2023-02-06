//! The canvas is the main window component that is responsible for plotting and display of the figure

use yew::prelude::*;
#[allow(unused_imports)]
use web_sys::{
    Element,
    EventTarget,
    HtmlElement,
    HtmlTextAreaElement,
    Node,
};

pub struct Canvas {
    click: MouseClickInfo,
}


pub struct MouseClickInfo {
    initialized: bool,
    coord_x: i32,
    coord_y: i32,
}

impl MouseClickInfo {
    fn new() -> Self {
        MouseClickInfo {
            initialized: false,
            coord_x: 0,
            coord_y: 0
        }
    }
}

fn update_coordinates(event: MouseEvent) -> MouseClickInfo {
    let x = event.client_x();
    let y = event.client_y();
    MouseClickInfo {
        initialized: true,
        coord_x: x,
        coord_y: y
    }
}

impl Component for Canvas {
    type Message = MouseClickInfo;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            click: MouseClickInfo::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.click = msg;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
                <div class={classes!("canvas")} onclick={link.callback(|x| {
                    update_coordinates(x)
                })}>
                    <p>{ format!("{}, {}", &self.click.coord_x, &self.click.coord_y) }</p>
                </div>
            </>
        }
    }
}
