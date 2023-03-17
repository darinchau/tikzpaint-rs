//! A wrapper around a WebGL Canvas that we can use to draw all sorts of stuff
//! Referenced from the YewStack example on Github
//! https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/webgl

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
use yew::prelude::*;
use gloo::console::log;

use crate::app::*;
use crate::renderer::*;

#[derive(Clone, Copy)]
pub enum CanvasRendererEvent {
    SetUpDimensions{w: i32, h: i32},

}

#[derive(Properties, PartialEq)]
pub struct CanvasRendererProps {
    pub tf: Transform,
    pub id: AttrValue,
    pub canvas: HtmlCanvas,
    pub cb: Callback<CanvasRendererEvent>
}

pub struct CanvasRenderer {
    node_ref: NodeRef,
}

impl Component for CanvasRenderer {
    type Message = ();
    type Properties = CanvasRendererProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.node_ref.clone()} />
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let window_resized = old_props.tf.screen_size != ctx.props().tf.screen_size;
        if window_resized {
            self.set_canvas_dims(ctx);
            return true;
        }

        false
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // Only start the render loop if it's the first render
        if !first_render {
            return;
        }

        self.set_canvas_dims(ctx);

        ctx.props().canvas.set_canvas(self.node_ref.clone());
    }
}

impl CanvasRenderer {
    fn set_canvas_dims(&self, ctx: &Context<Self>) {
        // Get the canvas
        log!("Setting canvas dimensions");

        if let Some(canvas) = self.node_ref.cast::<HtmlCanvasElement>() {
            // Set the margins of the canvas
            let (t, r, b, l) = ctx.props().tf.margins;
            let (x, y) = ctx.props().tf.screen_size;

            let w = x - r - l;
            let h = y - t - b;

            canvas.set_height(h as u32);
            canvas.set_width(w as u32);

            let info = CanvasRendererEvent::SetUpDimensions{w, h};

            ctx.props().cb.emit(info);
        }
    }
}