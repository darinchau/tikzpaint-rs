use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext as GL, WebGlRenderingContext};
use yew::prelude::*;
use gloo::console::log;

const PI: f64 = 3.1415926535897932384626433;

/// Make a canvas state handle to not screw up Rc Refcell patterns
#[derive(PartialEq, Clone)]
pub struct CanvasStateHandle {
    ptr: Rc<RefCell<NodeRef>>,
}

macro_rules! fm {
    ($x:ident) => {
        (*(*$x.ptr).borrow_mut())
    };
}

impl CanvasStateHandle {
    pub fn new() -> Self {
        Self {
            ptr: Rc::new(RefCell::new(NodeRef::default()))
        }
    }

    pub fn get_canvas(&self) -> Option<HtmlCanvasElement> {
        fm!(self).cast::<HtmlCanvasElement>()
    }

    pub fn set_canvas(&self, nr: NodeRef) {
        fm!(self) = nr;
    }
}