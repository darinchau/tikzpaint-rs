//! This file holds the base implementation of a displayable
//! If you want to define your own layer zero struct, you must implement two traits:
//! 1. FigureObject - an object that holds coordinates in DIMS dimension
//! 2. Plot - The figure will transform DIM coordinates into 2 dimensions - implement Plot to turn it into everything else to plot it on screen
use crate::app::Serializable;
use crate::figures::Coordinates;
use crate::figures::DimensionError;
use crate::figures::Hashable;
use crate::figures::PlotOptions;
use crate::figures::IsProjection;
use crate::figures::Projection;
use std::rc::Rc;

/// HasCoordinates will be one of the base traits
pub trait HasCoordinates: Serializable + Hashable {
    /// Returns a list of the coordinates of the figure object
    fn coordinates(&self) -> Vec<Coordinates>;
    fn dims(&self) -> usize;

    fn len(&self) -> usize {
        return self.coordinates().len();
    }
}

/// A plottable object is the last step before output. Make sure the figure object is dimension 2.
pub trait Plottable {
    fn tikzify(&self) -> String;
    fn tikz_options(&self) -> String;
}

pub trait FigureObject {
    /// Returns the plot option of the figure object
    fn options(&self) -> &PlotOptions;

    /// Project every coordinate in self according to the projection p
    /// Returns an error if the projection dimensions is not compatible with this figureobject
    fn project(&mut self, p: Projection) -> Result<(), DimensionError>;

    /// Project every coordinate in self to 2D according to the projection p
    fn project_to_plot(&self, p: &Box<&dyn IsProjection>) -> Result<Box<dyn Plottable>, DimensionError>;
}

/// Drawables are high-level implementations of Figure objects. They contain methods and stuff to implement
/// drawing multiple figure objects in a particular way.
/// If we look at the requirements for a Drawable object, we see we need the draw method, sized, clone, and no lifetime parameters
pub trait Drawable: Serializable + Hashable + 'static {
    /// Returns a vector of FigureObject that we will pass to the figure to draw.
    fn draw(&self) -> Vec<Box<dyn FigureObject>>;

    /// Returns the dimension that this drawable object lives in
    fn dims(&self) -> usize;

    /// Consumes ownership of self and returns a drawable object wrapper (a reference counted pointer to the object)
    fn wrap(self) -> DrawableWrapper;
}

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
}

impl Clone for DrawableWrapper {
    fn clone(&self) -> Self {
        DrawableWrapper {
            obj: Rc::clone(&self.obj)
        }
    }
}
