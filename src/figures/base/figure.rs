//! A figure object serves as a canvas to convert drawables into displayables into code and shapes

use crate::figures::{FigureObject, Drawable};

pub struct Figure<const DIMS: usize> {
    to_draw: Vec<Box<dyn Drawable<DIMS>>>,
}

impl<'a, const DIMS: usize> Figure<DIMS> {
    fn new() -> Self {
        Figure {
            to_draw: vec![]
        }
    }

    /// Adds 'draw' to the list of objects to be drawn. The object must have 
    fn draw(&mut self, draw: Box<dyn Drawable<DIMS>>) {
        self.to_draw.push(draw);
    } 

    fn load<T, S>(&self, f: T) -> Vec<S> where
    T: Fn(&dyn FigureObject<DIMS>) -> S {
        let mut v: Vec<S> = Vec::new();
        for x in &self.to_draw {
            for obj in x.draw() {
                let val = f(obj);
                v.push(val);
            }
        }

        return v;
    }
}