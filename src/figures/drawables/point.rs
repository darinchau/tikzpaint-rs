//! This is a direct implementation of a point out of the FOPoint

use crate::figures::*;

pub struct Point {
    p: FOPoint,
}

impl Point {
    pub fn new(x: Coordinates) -> Self {
        Point {
            p: FOPoint::new(x),
        }
    }
}

impl Drawable for Point {
    fn draw(&self) -> Vec<FigureObject> {
        return vec![self.p.clone().wrap()];
    }

    fn dims(&self) -> usize {
        return self.p.dims();
    }

    fn repr(&self) -> String {
        format!("point{}", self.p.point().to_string())
    }
}
