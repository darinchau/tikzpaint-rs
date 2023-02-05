//! A figure object serves as a canvas to convert drawables into displayables into code and shapes

use crate::figures::{Drawable, Projection, Plot};

pub struct Figure<'a, const DIMS: usize> {
    to_draw: Vec<&'a dyn Drawable<DIMS>>,
}

impl<'a, const DIMS: usize> Figure<'a, DIMS> {
    pub fn new() -> Self {
        Figure {
            to_draw: vec![]
        }
    }

    /// Adds 'obj' to the list of objects to be drawn. We use an RC because we don't want
    /// to take ownership of your lovely drawable object, but we also need the drawable object
    /// to live long enough and the easiest way is to take ownership of the object via an Rc
    pub fn draw(&mut self, obj: &'a dyn Drawable<DIMS>) where {
        self.to_draw.push(obj);
    } 

    /// Load method takes a function object and a projection object. The method will feed
    /// every coordinate in every drawable object through the function f that you provide.
    /// The projection will be fed through the project method defined on the function object.
    pub fn load<T, S, P>(&self, f: T, proj: &P) -> Vec<S> where
    T: Fn(Box<dyn Plot>) -> S,
    P: Projection<DIMS, 2>
    {
        let project = Box::new(proj as &dyn Projection<DIMS, 2>);
        let mut v: Vec<S> = Vec::new();
        for x in &self.to_draw {
            for obj in x.draw() {
                let new_obj = obj.project(&project);
                let ret_s = f(new_obj);
                v.push(ret_s);
            }
        }

        return v;
    }

    /// Output Tikz is a wrapper around the load method, also the core of Tikz code
    /// generation. The function object we pass is x -> x.tikzify(), and we collect
    /// everything into one string.
    /// 
    /// Example:
    /// ```
    /// use tikzpaint_rs::figures::{Figure, Identity};
    /// 
    /// let fig = Figure::<2>::new();
    /// let st = fig.output_tikz(&Identity);
    /// assert_eq!(st, "\\begin{tikzpicture}\n\\end{tikzpicture}")
    /// ```
    pub fn output_tikz<P>(&self, proj: &P) -> String where
    P: Projection<DIMS, 2> {
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