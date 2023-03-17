//! A figure object serves as a canvas to convert drawables into displayables into code and shapes

use std::any::{TypeId, Any};
use std::cell::RefCell;
use crate::figures::*;
use gloo::console::log;

#[derive(PartialEq)]
// Rerender every time we draw/project/do anything basically
pub struct Figure {
    to_draw: Vec<DrawableObject>,
    newly_drawn: RefCell<Vec<DrawableObject>>,
}

impl Figure {
    pub fn new() -> Self {
        Figure {
            to_draw: vec![],
            newly_drawn: RefCell::new(vec![]),
        }
    }

    /// Adds 'obj' to the list of objects to be drawn. Returns an error if the dimension of the obj does not match that of the figure
    pub fn draw<T: Drawable + WrapAsDrawable>(&mut self, obj: T) where {
        let a = obj.wrap();
        self.to_draw.push(a.clone());
        self.newly_drawn.borrow_mut().push(a);
    }

    /// Load method takes a function object and a projection object. The method will feed
    /// every coordinate in every drawable object through the function f that you provide.
    /// The projection will be fed through the project method defined on the function object.
    pub fn render<T, S>(&self, f: T) -> Vec<S> where
        T: Fn(PlottableObject) -> S,
    {
        let x = self.load(f, &self.newly_drawn.borrow());
        *self.newly_drawn.borrow_mut() = vec![];
        return x;
    }

    /// Render method only loads the newly drawns since last render. The method will feed
    /// every coordinate in every drawable object through the function f that you provide.
    /// The projection will be fed through the project method defined on the function object.
    pub fn load_all<T, S>(&self, f: T) -> Vec<S> where
    T: Fn(PlottableObject) -> S
    {
        return self.load(f, &self.to_draw);
    }

    fn load<T, S>(&self, f: T, vecd: &Vec<DrawableObject>) -> Vec<S> where
        T: Fn(PlottableObject) -> S
    {
        let mut v: Vec<S> = Vec::new();
        for x in vecd {
            for obj in x.draw() {
                let ret_s = f(obj);
                v.push(ret_s);
            }
        }

        return v;
    }
}
