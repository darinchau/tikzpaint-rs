//! This is a direct implementation of a point out of the FOPoint

use crate::figures::{Coordinates, Drawable, FigureObject, FOPoint};

pub struct Point<const DIMS: usize> {
    p: FOPoint<DIMS>,
}

impl<const DIMS: usize> Point<DIMS> {
    pub fn new(x: Coordinates<DIMS>) -> Self {
        Point {
            p: FOPoint::new(x)
        }
    }
}

impl<const DIMS: usize> Drawable<DIMS> for Point<DIMS> {
    fn draw(&self) -> Vec<&dyn FigureObject<DIMS>> {
        return vec![&self.p]
    }
}