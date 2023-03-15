//! A figure object serves as a canvas to convert drawables into displayables into code and shapes

use std::any::{TypeId, Any};
use std::cell::RefCell;
use crate::figures::*;
use gloo::console::log;

#[derive(PartialEq)]
// Rerender every time we draw/project/do anything basically
pub struct Figure {
    dims: usize,
    to_draw: Vec<DrawableObject>,
    newly_drawn: RefCell<Vec<DrawableObject>>,
}

impl Figure {
    pub fn new(dims: usize) -> Self {
        Figure {
            dims,
            to_draw: vec![],
            newly_drawn: RefCell::new(vec![]),
        }
    }

    /// Adds 'obj' to the list of objects to be drawn. Returns an error if the dimension of the obj does not match that of the figure
    pub fn draw<T: Drawable + WrappableAsDrawable>(&mut self, obj: T) -> Result<(), DimensionError> where {
        if obj.dims() != self.dims {
            return Err(DimensionError {
                msg: format!("Expect dimensions of the object ({}) to be the same as the dimensions of the figure ({})", obj.dims(), self.dims),
                source: "draw() from figure"
            });
        }
        let a = obj.wrap();
        self.to_draw.push(a.clone());
        self.newly_drawn.borrow_mut().push(a);
        return Ok(());
    }

    /// Load method takes a function object and a projection object. The method will feed
    /// every coordinate in every drawable object through the function f that you provide.
    /// The projection will be fed through the project method defined on the function object.
    pub fn render<T, S, P>(&self, f: T, proj: P) -> Result<Vec<S>, DimensionError> where
        T: Fn(PlottableObject) -> S,
        P: IsProjection + Any + 'static
    {
        let x = self.load(f, proj, &self.newly_drawn.borrow());
        *self.newly_drawn.borrow_mut() = vec![];
        return x;
    }

    /// Render method only loads the newly drawns since last render. The method will feed
    /// every coordinate in every drawable object through the function f that you provide.
    /// The projection will be fed through the project method defined on the function object.
    pub fn load_all<T, S, P>(&self, f: T, proj: P) -> Result<Vec<S>, DimensionError> where
    T: Fn(PlottableObject) -> S,
    P: IsProjection + Any + 'static
    {
        return self.load(f, proj, &self.to_draw);
    }

    fn load<T, S, P>(&self, f: T, proj: P, vecd: &Vec<DrawableObject>) -> Result<Vec<S>, DimensionError> where
        T: Fn(PlottableObject) -> S,
        P: IsProjection + Any + 'static
    {
        let fig_proj = proj.wrap();
        if fig_proj.output() != 2 {
            return Err(DimensionError{
                msg: format!("The output dimension of the projection ({}) should be 2", fig_proj.dims()),
                source: "load() from Figure"
            })
        }

        let mut v: Vec<S> = Vec::new();
        for x in vecd {
            for obj in x.draw() {
                let new_p = fig_proj.clone();

                // This projects everything to 2 dimensions. Early return error if some figure object is of wrong dimension
                let new_obj = (&obj).project(new_p)?;
                let plottable = new_obj.plot()?;
                let ret_s = f(plottable);
                v.push(ret_s);
            }
        }

        return Ok(v);
    }
}
