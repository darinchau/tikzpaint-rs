//! Figure objects are the first layer of objects between tikz code and our code. They are responsible for handling projections.
//! Plottables must translate directly into tikz code and svg code and whatever
//! Drawable objects are high level abstractions of figure objects.

use crate::figures::*;
use crate::renderer::*;
use std::rc::Rc;
use std::any::Any;
use std::fmt::Debug;

/// A plottable object is the last step before output. This is like the final state of the object to say we are about to plot stuff
/// At this point we guarantee that we only have x coordinates and y coordinates
pub trait Plottable {
    /// Define the construction of Tikz code from an object
    fn tikzify(&self) -> String;

    /// Define the logic for which we draw the object on an Html Canvas
    fn draw_on_canvas(&self, c: CanvasStateHandle) -> Result<(), DrawError>;
}

#[derive(Clone)]
/// Plottable object is like a universal wrapper around a Plottable
pub struct PlottableObject {
    ptr: Rc<dyn Plottable>,
}

impl Plottable for PlottableObject {
    fn tikzify(&self) -> String {
        return self.ptr.tikzify();
    }

    fn draw_on_canvas(&self, c: CanvasStateHandle) -> Result<(), DrawError> {
        return self.ptr.draw_on_canvas(c);
    }
}

pub trait IsFigureObject: Plottable {
    /// Returns a name of this figure object. This is useful for error checking
    fn name(&self) -> &'static str;

    /// Returns the ambient dimensions this object lives in.
    fn dims(&self) -> usize;

    /// Project every coordinate in self according to the projection p
    /// We guarantee the projection object passed to you has dimensions (self.dims() -> _)
    fn project(&self, p: Projection) -> Self where Self: Sized;
}

/// First layer of wrapper around a FO because idk why Rust doesn't allow self sized on trait objects
/// This implementation allows us to only call project on FO; other methods will defer to the implementation in IsFigureObject
trait FO where Self: IsFigureObject {
    fn project_and_wrap(&self, p: Projection) -> FigureObject;
}

impl<T: IsFigureObject + Sized + 'static> FO for T {
    fn project_and_wrap(&self, p: Projection) -> FigureObject {
        return self.project(p).wrap();
    }
}

/// A figure object is the base object (Layer 1 interface) between Tikz/SVG code and our code.
/// We have an additional layer of rust bindings to SVGs and Tikz because they are hard af to draw and manipulate
/// But Figure objects are the first layer that creates objects and is able to translate into both SVG and Tikz
pub struct FigureObject {
    ptr: Rc<dyn FO>,
    pub name: &'static str,
}

pub trait WrappableAsFigureObject {
    fn wrap(self) -> FigureObject where Self: IsFigureObject + Sized + 'static {
        let name = self.name();
        FigureObject {
            ptr: Rc::new(self),
            name
        }
    }
}

impl<T: IsFigureObject + Sized + 'static> WrappableAsFigureObject for T {}

impl Plottable for FigureObject {
    fn tikzify(&self) -> String {
        self.ptr.tikzify()
    }

    fn draw_on_canvas(&self, c: CanvasStateHandle)  -> Result<(), DrawError> {
        self.ptr.draw_on_canvas(c)
    }
}

impl FigureObject {
    /// Returns the ambient dimensions this object lives in.
    pub fn dims(&self) -> usize {
        return self.ptr.dims();
    }

    /// Project every coordinate in self according to the projection p
    /// We guarantee the projection object passed to you has dimensions (self.dims() -> _)
    pub fn project(&self, p: Projection) -> Result<FigureObject, DimensionError> {
        if p.input() != self.ptr.dims() {
            return Err(DimensionError {
                msg: format!("Expect the output dimension of the projection {} to be same as the dimension of the {} ({})", p.dims(), self.name, self.dims()),
                source: "project() from FigureObject"
            });
        }

        Ok(self.ptr.project_and_wrap(p))
    }

    /// Project this figure object to a 2-dimensional Plottable object
    pub fn plot(self) -> Result<PlottableObject, DimensionError> {
        // This is a hacky way to perform dimension checking but this reduces duplicate code in implementations
        let new_self = self.project(Identity{dims: 2}.wrap())?;
        Ok(PlottableObject { ptr: Rc::new(new_self) as Rc<dyn Plottable> })
    }
}

impl Clone for FigureObject {
    fn clone(&self) -> Self {
        FigureObject {
            ptr: Rc::clone(&self.ptr),
            name: self.name
        }
    }
}
