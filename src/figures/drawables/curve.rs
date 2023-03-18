//! An arbitrary curve approximated by a straight line
use std::cell::RefCell;
use std::rc::Rc;

use crate::figures::*;
use crate::core::*;

pub struct Curve {
    obj: Rc<RefCell<CurveInner>>
}

struct CurveInner {
    v: Vec<PlottableObject>,
    current: Coordinates,
    finished: bool
}

impl CurveInner {
    pub fn new(x: Coordinates) -> Self {
        Self {
            v: vec![],
            current: x,
            finished: false
        }
    }

    pub fn add(&mut self, x: Coordinates) {
        if !self.finished {
            self.v.push(FOLine::new(self.current, x).wrap());
            self.current = x;
        }
    }

    pub fn finish(&mut self) {
        self.finished = true;
    }

    fn draw(&self) -> Vec<PlottableObject> {
        if self.v.len() < 2 {
            return vec![];
        }

        todo!()
    }

    fn repr(&self) -> String {
        todo!()
    }
}

impl Drawable for Curve {
    fn draw(&self) -> Vec<PlottableObject> {
        self.obj.borrow().draw()
    }

    fn repr(&self) -> String {
        self.obj.borrow().repr()
    }
}

impl Curve {
    pub fn new(x: Coordinates) -> Self {
        Self {
            obj: Rc::new(RefCell::new(CurveInner::new(x)))
        }
    }

    pub fn add(&self, x: Coordinates) {
        self.obj.borrow_mut().add(x)
    }

    pub fn finish(&self) {
        self.obj.borrow_mut().finish()
    }
}
