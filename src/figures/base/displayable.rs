//! This file holds the base implementation of a displayable
use std::clone::Clone;
use crate::figures::Coordinates;
use crate::figures::PlotOptions;

/// Displayables serve as the atomic base objects that will be translated to shapes on the Figure and
/// into (Tikz or svg) code.
pub trait Displayable<const DIMS: usize>: Clone {
    fn tikzify(&self) -> String;
    fn coordinates(&self) -> Vec<Coordinates<DIMS>>;
    fn options(&self) -> PlotOptions;

    fn len(&self) -> usize {
        return self.coordinates().len();
    }
}