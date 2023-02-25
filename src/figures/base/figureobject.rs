//! This file holds the base implementation of a displayable
//! If you want to define your own layer zero struct, you must implement two traits:
//! 1. FigureObject - an object that holds coordinates in DIMS dimension
//! 2. Plot - The figure will transform DIM coordinates into 2 dimensions - implement Plot to turn it into everything else to plot it on screen
use crate::figures::Serializable;
use crate::figures::Coordinates;
use crate::figures::DimensionError;
use crate::figures::Hashable;
use crate::figures::PlotOptions;
use crate::figures::IsProjection;
use crate::figures::Projection;
use std::rc::Rc;

/// A plottable object is the last step before output. This is like the final state of the object to say we are about to plot stuff
pub trait Plottable {
    /// Define the construction of Tikz code from an object
    fn tikzify(&self) -> String;
}

/// A figure object is the base object (Layer 0 interface) between Tikz/SVG code and our code.
pub trait FigureObject: Serializable + Plottable {
    /// Returns a list of the coordinates of the figure object
    fn coordinates(&self) -> Vec<Coordinates>;

    fn dims(&self) -> usize;

    fn len(&self) -> usize {
        return self.coordinates().len();
    }

    /// Returns the plot option of the figure object
    fn options(&self) -> &PlotOptions;

    /// Project every coordinate in self according to the projection p
    /// Returns an error if the projection dimensions is not compatible with this figureobject
    fn project(&mut self, p: Projection) -> Result<(), DimensionError>;

    /// Project every coordinate in self to 2D according to the projection p. This is like an initialization method for us
    fn project_to_plot(&self, p: Projection) -> Result<Box<dyn Plottable>, DimensionError>;
}

/// Drawables are high-level implementations of Figure objects. They contain methods and stuff to implement
/// drawing multiple figure objects in a particular way.
/// If we look at the requirements for a Drawable object, we see we need the draw method, sized, clone, and no lifetime parameters
pub trait Drawable: Serializable + 'static {
    /// Returns a vector of FigureObject that we will pass to the figure to draw.
    fn draw(&self) -> Vec<Box<dyn FigureObject>>;

    /// Returns the dimension that this drawable object lives in
    fn dims(&self) -> usize;
}

pub trait WrappableAsDrawable {
    /// Consumes ownership of self and returns a drawable object wrapper (a reference counted pointer to the object)
    fn wrap(self) -> DrawableWrapper where
    Self: Sized + Drawable {
        return DrawableWrapper {
            obj: Rc::new(self)
        };
    }
}

impl<T: Drawable + Sized> WrappableAsDrawable for T {}

/// Drawable wrappers are reference counted smart pointers to the object itself.
pub struct DrawableWrapper {
    obj: Rc<dyn Drawable>
}

impl DrawableWrapper {
    /// This method is identical to obj.wrap() (as Drawable)
    pub fn new<T: Drawable>(obj: T) -> Self {
        obj.wrap()
    }

    pub fn draw(&self) -> Vec<Box<dyn FigureObject>> {
        return self.obj.draw();
    }

    pub fn dims(&self) -> usize {
        return self.obj.dims();
    }
}

impl Clone for DrawableWrapper {
    fn clone(&self) -> Self {
        DrawableWrapper {
            obj: Rc::clone(&self.obj)
        }
    }
}
