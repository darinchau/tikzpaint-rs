//! This file holds the base implementation of a displayable
//! If you want to define your own layer zero struct, you must implement two traits:
//! 1. FigureObject - an object that holds coordinates in DIMS dimension
//! 2. Plot - The figure will transform DIM coordinates into 2 dimensions - implement Plot to turn it into everything else to plot it on screen
use crate::app::Serializable;
use crate::figures::Coordinates;
use crate::figures::DimensionError;
use crate::figures::Hashable;
use crate::figures::PlotOptions;
use crate::figures::Projection;
use std::rc::Rc;

/// Figure Objects serve as the atomic base objects that will be translated to shapes on the Figure and
/// into (Tikz or svg) code.
pub trait FigureObject: Serializable + Hashable + Plot {
    /// Returns a list of the coordinates of the figure object
    fn coordinates(&self) -> Vec<Coordinates>;
    fn dims(&self) -> usize;

    /// Returns the plot option of the figure object
    fn options(&self) -> &PlotOptions;
    fn project(&self, p: &Box<dyn Projection>) -> Result<Box<dyn Plot>, DimensionError>;
    fn len(&self) -> usize {
        return self.coordinates().len();
    }
}

pub trait Plot {
    fn tikzify(&self) -> Result<String, DimensionError>;
    fn tikz_options(&self) -> Result<String, DimensionError>;
}

/// Drawables are high-level implementations of Figure objects. They contain methods and stuff to implement
/// drawing multiple figure objects in a particular way.
/// If we look at the requirements for a Drawable object, we see we need the draw method, sized, clone, and no lifetime parameters
pub trait Drawable: Sized + Clone + Serializable + Hashable + 'static {
    /// Returns a vector of FigureObject that we will pass to the figure to draw.
    fn draw(&self) -> Vec<dyn FigureObject>;
}
