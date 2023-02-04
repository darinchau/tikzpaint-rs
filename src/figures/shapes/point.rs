//! Implementation of a node

use crate::figures::{PlotOptions, Coordinates, FigureObject, base::figureobject::Plot};

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
            option: PlotOptions {
                color: self.options().color.clone(),
                thickness: self.options().thickness 
            }
        }
    }
}

impl<const DIMS: usize> FigureObject<DIMS> for Point<DIMS> {
    fn coordinates(&self) -> Vec<Coordinates<DIMS>> {
        vec![self.p.clone()]
    }

    fn options(&self) -> &PlotOptions {
        &self.option
    }

    fn len(&self) -> usize {
        1
    }
}

impl Plot for Point<2> {
    fn tikzify(&self) -> String {
        format!("\\node[{}] at {} {{}}", self.options().tikzify(), self.p)
    }
}