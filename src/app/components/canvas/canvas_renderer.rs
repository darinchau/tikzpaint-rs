//! A wrapper around a WebGL Canvas that we can use to draw all sorts of stuff
//! Copied and modified from the YewStack example on Github
//! https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/webgl

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext as GL, WebGlRenderingContext};
use yew::prelude::*;
use gloo::console::log;

use crate::app::*;

#[derive(PartialEq, Clone)]
pub struct CanvasStateHandle {
    ptr: Rc<RefCell<NodeRef>>,
}

impl CanvasStateHandle {
    pub fn new() -> Self {
        Self {
            ptr: Rc::new(RefCell::new(NodeRef::default()))
        }
    }

    fn get_canvas(&self) -> Option<HtmlCanvasElement> {
        (*(*self.ptr).borrow_mut()).cast::<HtmlCanvasElement>()
    }

    fn set_canvas(&self, nr: NodeRef) {
        *(*self.ptr).borrow_mut() = nr;
    }
}

#[derive(Properties, PartialEq)]
pub struct CanvasRendererProps {
    pub tf: Transform,
    pub id: AttrValue,
    pub canvas: CanvasStateHandle,
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

        // // Wrap the canvas node ref in the canvas element in props


        // let gl: GL = canvas
        //     .get_context("webgl")
        //     .unwrap()
        //     .unwrap()
        //     .dyn_into()
        //     .unwrap();

        // Self::render_gl(gl);
    }
}

impl CanvasRenderer {
    fn set_canvas_dims(&self, ctx: &Context<Self>) {
        log!("Resetting canvas dimensions");
        // Get the canvas
        if let Some(canvas) = self.node_ref.cast::<HtmlCanvasElement>() {
            // Set the margins of the canvas
            let (t, r, b, l) = ctx.props().tf.margins;
            let (x, y) = ctx.props().tf.screen_size;

            let w = x - r - l;
            let h = y - t - b;

            canvas.set_height(h as u32);
            canvas.set_width(w as u32);
        }
    }
//     fn request_animation_frame(f: &Closure<dyn FnMut()>) {
//         window()
//             .unwrap()
//             .request_animation_frame(f.as_ref().unchecked_ref())
//             .expect("should register `requestAnimationFrame` OK");
//     }

//     fn render_gl(gl: WebGlRenderingContext) {
//         // This should log only once -- not once per frame

//         let mut timestamp = 0.0;

//         let vert_code = include_str!("./basic.vert");
//         let frag_code = include_str!("./basic.frag");

//         // This list of vertices will draw two triangles to cover the entire canvas.
//         let vertices: Vec<f32> = vec![
//             -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
//         ];
//         let vertex_buffer = gl.create_buffer().unwrap();
//         let verts = js_sys::Float32Array::from(vertices.as_slice());

//         gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
//         gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

//         let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
//         gl.shader_source(&vert_shader, vert_code);
//         gl.compile_shader(&vert_shader);

//         let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
//         gl.shader_source(&frag_shader, frag_code);
//         gl.compile_shader(&frag_shader);

//         let shader_program = gl.create_program().unwrap();
//         gl.attach_shader(&shader_program, &vert_shader);
//         gl.attach_shader(&shader_program, &frag_shader);
//         gl.link_program(&shader_program);

//         gl.use_program(Some(&shader_program));

//         // Attach the position vector as an attribute for the GL context.
//         let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
//         gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
//         gl.enable_vertex_attrib_array(position);

//         // Attach the time as a uniform for the GL context.
//         let time = gl.get_uniform_location(&shader_program, "u_time");
//         gl.uniform1f(time.as_ref(), timestamp as f32);

//         gl.draw_arrays(GL::TRIANGLES, 0, 6);

//         // Gloo-render's request_animation_frame has this extra closure
//         // wrapping logic running every frame, unnecessary cost.
//         // Here constructing the wrapped closure just once.

//         let cb = Rc::new(RefCell::new(None));

//         *cb.borrow_mut() = Some(Closure::wrap(Box::new({
//             let cb = cb.clone();
//             move || {
//                 // This should repeat every frame
//                 timestamp += 20.0;
//                 gl.uniform1f(time.as_ref(), timestamp as f32);
//                 gl.draw_arrays(GL::TRIANGLES, 0, 6);
//                 CanvasRenderer::request_animation_frame(cb.borrow().as_ref().unwrap());
//             }
//         }) as Box<dyn FnMut()>));

//         CanvasRenderer::request_animation_frame(cb.borrow().as_ref().unwrap());
//     }
}