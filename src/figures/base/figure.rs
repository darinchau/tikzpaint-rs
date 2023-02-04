//! A figure object serves as a canvas to convert drawables into displayables into code and shapes

use crate::figures::{Drawable, Projection, Plot};

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

    pub fn load<T, S>(&self, f: T, proj: &Box<dyn Projection<DIMS, 2>>) -> Vec<S> where
    T: Fn(Box<dyn Plot>) -> S,
    {
        let mut v: Vec<S> = Vec::new();
        for x in &self.to_draw {
            for obj in x.draw() {
                let new_obj = obj.project(proj);
                let ret_s = f(new_obj);
                v.push(ret_s);
            }
        }

        return v;
    }

    pub fn output_tikz(&self, proj: &Box<dyn Projection<DIMS, 2>>) -> String {
        let mut st = String::from("\\begin{tikzpicture}\n");
        for s in self.load(|x| {
            return x.tikzify();
        }, proj) {
            st.push_str("\t");
            st.push_str(&s);
            st.push_str("\n");
        }
        st.push_str("\\end{tikzpicture}");
        return st;
    }
}