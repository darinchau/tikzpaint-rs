//! Handler for drawing on HTML canvas

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

fn max(a: f64, b: f64) -> f64 {
    if a > b {a} else {b}
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {a} else {b}
}

/// Make a canvas state handle to not screw up Rc Refcell patterns
/// Every draw will consist of a translation according to the transform,
#[derive(PartialEq, Clone)]
pub struct HtmlCanvas {
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

impl HtmlCanvas {
    pub fn new(t: Rc<RefCell<Transform>>) -> Self {
        Self {
            ptr: Rc::new(RefCell::new(NodeRef::default())),
            tf: t
        }
    }

    pub fn set_canvas(&self, nr: NodeRef) {
        log!("Initializing canvas");
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
    pub fn draw_circle(&self, local_coords: Coordinates, radius: f64) -> Result<(), DrawError> {
        let (x, y) = self.tf.borrow().local_to_client(local_coords);
        let c = self.context()?;
        c.begin_path();
        c.arc(x, y, radius, 0., PI * 2.).cvt()?;
        c.stroke();

        Ok(())
    }

    /// For all the draw methods, returns () if the result is successfully drawn,
    /// otherwise returns an Err
    pub fn draw_rectangles(&self, corner_1: Coordinates, corner_2: Coordinates) -> Result<(), DrawError> {
        let (t, l) = self.tf.borrow().local_to_client(corner_1);
        let (b, r) = self.tf.borrow().local_to_client(corner_2);

        let bot = max(t, b);
        let top = min(t, b);
        let lef = min(l, r);
        let rig = max(l, r);

        self.context()?.fill_rect(top, lef, bot - top, rig - lef);

        Ok(())
    }

    /// For all the draw methods, returns () if the result is successfully drawn,
    /// otherwise returns an Err
    pub fn draw_line(&self, start: Coordinates, end: Coordinates) -> Result<(), DrawError> {
        let (x1, y1) = self.tf.borrow().local_to_client(start);
        let (x2, y2) = self.tf.borrow().local_to_client(end);

        let ctx = self.context()?;

        ctx.move_to(x1, y1);
        ctx.line_to(x2, y2);
        ctx.stroke();

        Ok(())
    }

    /// An optimized version for drawing many lines to approximate a curve
    pub fn draw_curve(&self, coords: Vec<Coordinates>) -> Result<(), DrawError> {
        todo!()
    }

    /// Resets all the contents on the canvas
    pub fn reset(&self) -> Result<(), DrawError> {
        let ctx = self.context()?;

        let a = fig!(self).cast::<HtmlCanvasElement>();
        if let Some(canvas) = a {
            ctx.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
        }

        return Err(DrawError { msg: String::from("Failed to get canvas element") });
    }
}