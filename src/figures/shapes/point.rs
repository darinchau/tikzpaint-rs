//! Implementation of a node

use crate::figures::{Displayable, PlotOptions, Coordinates};

pub struct Point<const DIMS: usize> {
    p: Coordinates<DIMS>,
    option: PlotOptions
}

impl<const DIMS: usize> Point<DIMS> {
    pub fn new(c: Coordinates<DIMS>) -> Point<DIMS> {
        Point {
            p: c,
            option: PlotOptions::new()
        }
    }
}

impl <const DIMS: usize> Clone for Point<DIMS> {
    fn clone(&self) -> Self {
        Point{
            p: self.p.clone(),
            option: self.option.clone()
        }
    }
}

impl<const DIMS: usize> Displayable<DIMS> for Point<DIMS> {
    fn tikzify(&self) -> String {
        format!("\\node[{}] at {} {{}}", self.options().tikzify(), self.p)
    }

    fn coordinates(&self) -> Vec<Coordinates<DIMS>> {
        vec![self.p.clone()]
    }

    fn options(&self) -> PlotOptions {
        self.option.clone()
    }

    fn len(&self) -> usize {
        1
    }
}