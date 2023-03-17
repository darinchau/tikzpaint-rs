//! Figure objects are the first layer of objects between tikz code and our code. They are responsible for handling projections.
//! Plottables must translate directly into tikz code and svg code and whatever
//! Drawable objects are high level abstractions of figure objects.

use crate::figures::*;
use crate::renderer::*;
use std::rc::Rc;
use std::any::Any;
use std::fmt::Debug;

/// A figure object is the base object (Layer 1 interface) between Tikz/SVG code and our code.
/// We have an additional layer of rust bindings to SVGs and Tikz because they are hard af to draw and manipulate
/// But Figure objects are the first layer that creates objects and is able to translate into both SVG and Tikz
pub trait FigureObject {
    /// A unique string that represents this object. This will be used to define partial eq.
    fn repr(&self) -> String;

    /// Define the construction of Tikz code from an object
    fn tikzify(&self) -> (String, Option<String>);

    /// Define the logic for which we draw the object on an Html Canvas
    fn draw_on_canvas(&self, c: HtmlCanvas) -> Result<(), DrawError>;
}

#[derive(Clone)]
/// Plottable object is like a universal wrapper around a FigureObject
pub struct PlottableObject {
    ptr: Rc<dyn FigureObject>,
}

impl FigureObject for PlottableObject {
    fn tikzify(&self) -> (String, Option<String>) {
        return self.ptr.tikzify();
    }

    fn draw_on_canvas(&self, c: HtmlCanvas) -> Result<(), DrawError> {
        return self.ptr.draw_on_canvas(c);
    }

    fn repr(&self) -> String {
        return self.ptr.repr();
    }
}

impl TikzShape for PlottableObject {
    fn draw(&self) -> (String, Option<String>) {
        return self.tikzify();
    }
}

pub trait WrapAsPlottable {
    fn wrap(self) -> PlottableObject where Self: FigureObject + Sized + Any + 'static {
        if let Some(x) = (&self as &dyn Any).downcast_ref::<PlottableObject>() {
            return x.clone();
        }
        PlottableObject {
            ptr: Rc::new(self),
        }
    }
}

impl<T: FigureObject + Sized + 'static> WrapAsPlottable for T {}

impl PartialEq for PlottableObject {
    fn eq(&self, other: &Self) -> bool {
        return self.repr() == other.repr();
    }
}

impl Debug for PlottableObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}
