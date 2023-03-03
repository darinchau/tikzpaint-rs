/// This is used for passing the transform into the object without actually passing the transform into the object,
/// We try to make independent the 'figure' parts of the app and 'app' part of the app

use std::rc::Rc;

/// This is used for passing the transform into the object without actually passing the transform into the object,
/// We try to make independent the 'figure' parts of the app and 'app' part of the app
#[derive(Clone)]
pub struct CoordTransform {
    f: Rc<dyn Fn((f64, f64)) -> (f64, f64)>
}

impl CoordTransform {
    pub fn new<F: 'static + Fn((f64, f64)) -> (f64, f64)>(f: F) -> Self {
        Self {
            f: Rc::new(f) as Rc<dyn Fn((f64, f64)) -> (f64, f64)>
        }
    }

    pub fn call(&self, x: (f64, f64)) -> (f64, f64) {
        return (self.f)(x);
    }
}
