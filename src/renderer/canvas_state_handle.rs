use std::cell::RefCell;
use std::error::Error;
use std::fmt::Debug;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
use yew::prelude::*;
use gloo::console::log;

use crate::renderer::*;

const PI: f64 = 3.1415926535897932384626433;

pub struct DrawError {
    msg: String
}

impl Debug for DrawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Draw Error -- {}", self.msg)
    }
}

trait ConvertError {
    fn cvt(self) -> Result<(), DrawError>;
}

impl ConvertError for Result<(), JsValue> {
    fn cvt(self) -> Result<(), DrawError> {
        if let Err(e) = self {
            return Err(DrawError{msg: format!("{:?}", e)});
        }

        Ok(())
    }
}

/// Make a canvas state handle to not screw up Rc Refcell patterns
/// Every draw will consist of a translation according to the transform,
#[derive(PartialEq, Clone)]
pub struct CanvasStateHandle {
    ptr: Rc<RefCell<NodeRef>>,
    tf: Rc<RefCell<Transform>>
}

macro_rules! fm {
    ($x:ident) => {
        (*(*$x.ptr).borrow_mut())
    };
}

macro_rules! fig {
    ($x:ident) => {
        (*(*$x.ptr).borrow())
    };
}

impl CanvasStateHandle {
    pub fn new(t: Rc<RefCell<Transform>>) -> Self {
        Self {
            ptr: Rc::new(RefCell::new(NodeRef::default())),
            tf: t
        }
    }

    pub fn set_canvas(&self, nr: NodeRef) {
        fm!(self) = nr;
    }

    fn context(&self) -> Result<CanvasRenderingContext2d, DrawError> {
        let a = fig!(self).cast::<HtmlCanvasElement>();
        if a.is_none() {
            return Err(DrawError { msg: String::from("Failed to get canvas element") });
        }

        let b = a.unwrap().get_context("2d");
        if let Err(e) = b {
            return Err(DrawError { msg: format!("Failed to get canvas context. Got: {:?}", e) });
        }

        let c = b.unwrap();
        if c.is_none() {
            return Err(DrawError { msg: format!("Failed to get canvas context, got no values.") });
        }

        let d = c.unwrap().dyn_into::<CanvasRenderingContext2d>();
        if let Err(e) = d {
            return Err(DrawError { msg: format!("Failed to get canvas context, got {:?}.", e) });
        }

        return Ok(d.unwrap());
    }

    /// For all the draw methods, returns () if the result is successfully drawn,
    /// otherwise returns an Err
    pub fn draw_circle(&self, local_coords: (f64, f64), radius: f64) -> Result<(), DrawError> {
        log!("Hi");
        let (a, b) = local_coords;
        let (x, y) = self.tf.borrow().local_to_client(a, b);
        let c = self.context()?;
        c.begin_path();
        c.arc(x, y, radius, 0., PI * 2.).cvt()?;
        c.stroke();

        Ok(())
    }

    /// For all the draw methods, returns () if the result is successfully drawn,
    /// otherwise returns an Err
    pub fn draw_rectangle(&self, topleft: (f64, f64), bottomright: (f64, f64)) -> Result<(), DrawError> {
        let (a, b) = topleft;
        let (c, d) = bottomright;

        if a <= c {
            return Err(DrawError { msg: format!("Top coordinates ({a}) is smaller than bottom coordinates ({c})") });
        }

        if b >= d {
            return Err(DrawError { msg: format!("Left coordinates ({b}) is bigger than right coordinates ({d})") });
        }

        let (t, l) = self.tf.borrow().local_to_client(a, b);
        let (b, r) = self.tf.borrow().local_to_client(c, d);

        self.context()?.fill_rect(t, l, b - t, r - l);

        Ok(())
    }
}