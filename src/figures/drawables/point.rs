//! This is a direct implementation of a point out of the FOPoint

use crate::figures::*;
use crate::core::*;

/// A point without anything else
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
    fn draw(&self) -> Vec<PlottableObject> {
        return vec![self.p.clone().wrap()];
    }

    fn repr(&self) -> String {
        format!("point{}", self.p.point().to_string())
    }
}
