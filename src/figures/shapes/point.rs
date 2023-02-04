//! Implementation of a node

use crate::figures::{PlotOptions, Coordinates, FigureObject, Plot, Projection};

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

    fn project(&self, p: &Box<dyn Projection<DIMS, 2>>) -> Box<dyn Plot> {
        let new_p = (*p).call(&self.p);
        return Box::new(Point{
            p: new_p,
            option: self.option.clone()
        });
    }
}

impl Plot for Point<2> {
    fn tikzify(&self) -> String {
        format!("\\node[{}] at {} {{}}", self.options().tikzify(), self.p)
    }
}