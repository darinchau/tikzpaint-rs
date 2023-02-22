//! A figure object serves as a canvas to convert drawables into displayables into code and shapes

use crate::figures::{Drawable, IsProjection, Plottable, DimensionError, DrawableWrapper};
use std::rc::Rc;

pub struct Figure {
    dims: usize,
    to_draw: Vec<DrawableWrapper>,
    hash: i64
}

impl PartialEq for Figure {
    fn eq(&self, other: &Self) -> bool {
        return self.hash == other.hash;
    }
}

impl Figure {
    pub fn new(dims: usize) -> Self {
        Figure {
            dims,
            to_draw: vec![],
            hash: 0
        }
    }

    /// Adds 'obj' to the list of objects to be drawn. We use an RC because we don't want
    /// to take ownership of your lovely drawable object, but we also need the drawable object
    /// to live long enough and the easiest way is to take ownership of the object via an Rc
    pub fn draw(&mut self, obj: DrawableWrapper) where {
        self.to_draw.push(obj);
    }

    /// Load method takes a function object and a projection object. The method will feed
    /// every coordinate in every drawable object through the function f that you provide.
    /// The projection will be fed through the project method defined on the function object.
    pub fn load<T, S, P>(&self, f: T, proj: &P) -> Result<Vec<S>, DimensionError> where
        T: Fn(Box<dyn Plottable>) -> S,
        P: IsProjection
    {
        let project = Box::new(proj as &dyn IsProjection);
        let mut v: Vec<S> = Vec::new();
        for x in &self.to_draw {
            for obj in x.draw() {
                let new_obj = (*obj).project_to_plot(&project)?;
                let ret_s = f(new_obj);
                v.push(ret_s);
            }
        }

        return Ok(v);
    }

    /// Output Tikz is a wrapper around the load method, also the core of Tikz code
    /// generation. The function object we pass is x -> x.tikzify(), and we collect
    /// everything into one string.
    ///
    /// Example:
    /// ```
    /// use tikzpaint_rs::figures::{Figure, Identity};
    ///
    /// let fig = Figure::new(2);
    /// let st = fig.output_tikz(&Identity{dims: 2}).unwrap();
    /// assert_eq!(st, "\\begin{tikzpicture}\n\\end{tikzpicture}")
    /// ```
    pub fn output_tikz<P>(&self, proj: &P) -> Result<String, DimensionError> where
    P: IsProjection {
        if self.dims != proj.input() {
            return Err(DimensionError{
                msg: format!("The input dimension of the projection ({}) should be the same as the figure dimension ({})", proj.input(), self.dims),
                source: "output_tikz() from Figure"
            })
        }
        if proj.output() != 2 {
            return Err(DimensionError{
                msg: format!("The output dimension of the projection ({}) should be 2", proj.output()),
                source: "output_tikz() from Figure"
            })
        }

        let mut st = String::from("\\begin{tikzpicture}\n");
        for s in self.load(|x| {
            return x.tikzify();
        }, proj)? {
            st.push_str("\t");
            st.push_str(&s);
            st.push_str("\n");
        }
        st.push_str("\\end{tikzpicture}");
        return Ok(st);
    }
}
