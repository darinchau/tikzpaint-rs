//! This file holds the base implementation of a displayable
use crate::figures::Coordinates;
use crate::figures::PlotOptions;

/// Figure Objects serve as the atomic base objects that will be translated to shapes on the Figure and
/// into (Tikz or svg) code.
pub trait FigureObject<const DIMS: usize> {
    fn coordinates(&self) -> Vec<Coordinates<DIMS>>;
    fn options(&self) -> &PlotOptions;

    fn len(&self) -> usize {
        return self.coordinates().len();
    }
}

/// Plottable objects are figure objects with 2D coordinates. 
pub trait Plot: FigureObject<2> {
    fn tikzify(&self) -> String;
}

/// Drawables are high-level implementations of Figure objects. They contain methods and stuff to implement
/// drawing multiple figure objects in a particular way
pub trait Drawable<const DIMS: usize> {
    fn draw(&self) -> Vec<&dyn FigureObject<DIMS>>;
}