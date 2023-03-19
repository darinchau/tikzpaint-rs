//! An arbitrary curve approximated by a straight line
use std::cell::RefCell;
use std::rc::Rc;

use crate::figures::*;
use crate::core::*;

use gloo::console::log;

pub struct Curve {
    v: ScopedVec<Coordinates>,
}

impl Drawable for Curve {
    /// Draws the curve. This gets a reference of coordinates
    fn draw(&self) -> Vec<PlottableObject> {
        let mut v = vec![];
        let mut current = None;
        for x in self.v.iter() {
            if let Some(c) = current {
                v.push(FOLine::new(c, x).wrap());
            }

            current = Some(x);
        }

        return v;
    }

    fn repr(&self) -> String {
        let mut s = String::from("curve");
        for x in self.v.iter() {
            s.push_str(&format!("{}", x));
        }
        return s;
    }
}

impl Curve {
    /// Creates a new curve. We guarantee that nowhere in this curve we access the vector of coordinates mutably
    pub fn new(vx: ScopedVec<Coordinates>) -> Self {
        Self {
            v: vx
        }
    }
}
