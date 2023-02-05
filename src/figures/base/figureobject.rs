//! This file holds the base implementation of a displayable
//! If you want to define your own layer zero struct, you must implement two traits:
//! 1. FigureObject - an object that holds coordinates in DIMS dimension
//! 2. Plot - The figure will transform DIM coordinates into 2 dimensions - implement Plot to turn it into everything else to plot it on screen
use crate::figures::Coordinates;
use crate::figures::PlotOptions;
use crate::figures::Projection;

/// Figure Objects serve as the atomic base objects that will be translated to shapes on the Figure and
/// into (Tikz or svg) code.
pub trait FigureObject<const DIMS: usize> {
    fn coordinates(&self) -> Vec<Coordinates<DIMS>>;
    fn options(&self) -> &PlotOptions;
    fn project(&self, p: &Box<&dyn Projection<DIMS, 2>>) -> Box<dyn Plot>;
    fn len(&self) -> usize {
        return self.coordinates().len();
    }
}

pub trait Plot: FigureObject<2> {
    fn tikzify(&self) -> String;
}

/// Drawables are high-level implementations of Figure objects. They contain methods and stuff to implement
/// drawing multiple figure objects in a particular way.
pub trait Drawable<const DIMS: usize> {
    /// We allow the draw method to return a reference because then we are allowed to cache the return value
    fn draw(&self) -> &Vec<Box<dyn FigureObject<DIMS>>>;
}
