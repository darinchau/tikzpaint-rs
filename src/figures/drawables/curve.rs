//! An arbitrary curve approximated by a straight line
use crate::figures::*;
use crate::core::*;

pub struct Curve {
    p: FOPoint,
}

impl Curve {
    pub fn new(x: Coordinates) -> Self {
        todo!()
    }
}

impl Drawable for Curve {
    fn draw(&self) -> Vec<PlottableObject> {
        todo!()
    }

    fn repr(&self) -> String {
        todo!()
    }
}
