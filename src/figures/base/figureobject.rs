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
    /// Returns a list of the coordinates of the figure object
    fn coordinates(&self) -> Vec<Coordinates<DIMS>>;

    /// Returns the plot option of the figure object
    fn options(&self) -> &PlotOptions;
    fn project(&self, p: &Box<&dyn Projection<DIMS, 2>>) -> Box<dyn Plot>;
    fn len(&self) -> usize {
        return self.coordinates().len();
    }
}

pub trait Plot: FigureObject<2> {
    fn tikzify(&self) -> String;
    fn tikz_options(&self) -> String;
}

/// Drawables are high-level implementations of Figure objects. They contain methods and stuff to implement
/// drawing multiple figure objects in a particular way.
pub trait Drawable<const DIMS: usize> {
    /// Returns a vector of FigureObject that we will pass to the figure to draw.
    fn draw(&self) -> Vec<&dyn FigureObject<DIMS>>;
}
