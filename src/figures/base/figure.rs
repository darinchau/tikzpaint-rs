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
                let new_obj = (&obj).project_to_plot(new_p)?;
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
    /// use tikzpaint_rs::figures::WrappableAsProjection;
    ///
    /// let fig = Figure::new(2);
    /// let st = fig.output_tikz(Identity{dims: 2}).unwrap();
    /// assert_eq!(st, "\\begin{tikzpicture}\n\\end{tikzpicture}")
    /// ```
    pub fn output_tikz<P: IsProjection + Any + 'static>(&self, projection: P) -> Result<String, DimensionError> {
        // If p is not a projection make it a projection, otherwise leave it as is.
        let proj = projection.wrap();

        if self.dims != proj.input() {
            return Err(DimensionError{
                msg: format!("The input dimension of the projection ({}) should be the same as the figure dimension ({})", proj.input(), self.dims),
                source: "output_tikz() from Figure"
            })
        }
        if proj.output() != 2 {
            return Err(DimensionError{
                msg: format!("The output dimension of the projection ({}) should be 2", proj.dims()),
                source: "output_tikz() from Figure"
            })
        }

        let mut st = String::from("\\begin{tikzpicture}\n");
        for s in self.load_all(|x| {
            return x.tikzify();
        }, proj)? {
            let res = s.output();
            st.push_str("\t");
            st.push_str(&res);
            st.push_str("\n");
        }
        st.push_str("\\end{tikzpicture}");
        return Ok(st);
    }
}
