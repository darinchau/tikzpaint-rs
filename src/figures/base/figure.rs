//! A figure object serves as a canvas to convert drawables into displayables into code and shapes

use crate::figures::{FigureObject, Drawable, Projection};

pub struct Figure<const DIMS: usize> {
    to_draw: Vec<Box<dyn Drawable<DIMS>>>,
}

impl<'a, const DIMS: usize> Figure<DIMS> {
    pub fn new() -> Self {
        Figure {
            to_draw: vec![]
        }
    }

    /// Adds 'draw' to the list of objects to be drawn. The object must have 
    pub fn draw(&mut self, draw: Box<dyn Drawable<DIMS>>) {
        self.to_draw.push(draw);
    } 

    pub fn load<T, S, P>(&self, f: T, p: P) -> Vec<S> where
    T: Fn(&dyn FigureObject<2>) -> S,
    P: Projection<DIMS, 2> {
        let mut v: Vec<S> = Vec::new();
        for x in &self.to_draw {
            for obj in x.draw() {
                let new_x = p.call(obj);
                let val = f(new_x);
                v.push(val);
            }
        }

        return v;
    }
}

impl Figure<2> {
    pub fn output_tikz(&self) -> String {
        let mut st = String::from("\\begin{tikzpicture}\n");
        for s in self.load(|x| {
            
        }) {

        }
        return st;
    }
}