//! This file holds the base implementation of a displayable
//! If you want to define your own layer zero struct, you must implement two traits:
//! 1. FigureObject - an object that holds coordinates in DIMS dimension
//! 2. Plot - The figure will transform DIM coordinates into 2 dimensions - implement Plot to turn it into everything else to plot it on screen
use crate::figures::*;
use crate::renderer::*;
use std::rc::Rc;
use std::any::Any;
use std::fmt::Debug;

/// A plottable object is the last step before output. This is like the final state of the object to say we are about to plot stuff
/// At this point we guarantee that we only have x coordinates and y coordinates
pub trait Plottable {
    /// Define the construction of Tikz code from an object
    fn tikzify(&self) -> TikzFigure;

    /// Define the logic for which we draw the object on an Html Canvas
    fn draw_on_canvas(&self, c: CanvasStateHandle) -> Result<(), DrawError>;
}

#[derive(Clone)]
/// Plottable object is like a universal wrapper around a Plottable
pub struct PlottableObject {
    ptr: Rc<dyn Plottable>,
    coords: Vec<(f64, f64)>
}

impl PlottableObject {
    /// Gets and returns the coordinates as an array
    /// This is like extra promise that we only have x and y coordinates
    /// For the default implementation, this function will panic if the resulting coordinates are not all in 2 dimensions
    pub fn coordinates(&self) -> &Vec<(f64, f64)> {
        return &self.coords;
    }
}

impl Plottable for PlottableObject {
    fn tikzify(&self) -> TikzFigure {
        return self.ptr.tikzify();
    }

    fn draw_on_canvas(&self, c: CanvasStateHandle) -> Result<(), DrawError> {
        return self.ptr.draw_on_canvas(c);
    }
}


// ===============================================================================================================================
// ===============================================================================================================================
// ===============================================================================================================================

/// A figure object is the first layer that unifies the different methods of drawing
pub trait IsFigureObject: Plottable {
    /// Returns a name of this figure object. This is useful for error checking
    fn name(&self) -> &'static str;

    /// Returns a list of the coordinates of the figure object
    fn coordinates(&self) -> Vec<Coordinates>;

    /// Returns the ambient dimensions this object lives in.
    fn dims(&self) -> usize;

    /// Returns the number of coordinates this thing is composed of
    fn len(&self) -> usize {
        return self.coordinates().len();
    }

    /// Project every coordinate in self according to the projection p
    /// We guarantee the projection object passed to you has dimensions (self.dims() -> _)
    fn project(&self, p: Projection) -> Self where Self: Sized;
}

/// First layer of wrapper around a FO because idk why Rust doesn't allow self sized on trait objects
/// This implementation allows us to only call project on FO; other methods will defer to the implementation in IsFigureObject
trait FO where Self: IsFigureObject {
    fn project_and_wrap(&self, p: Projection) -> FigureObject;
}

impl<T: IsFigureObject + Sized + 'static> FO for T {
    fn project_and_wrap(&self, p: Projection) -> FigureObject {
        return self.project(p).wrap();
    }
}


/// A figure object is the base object (Layer 1 interface) between Tikz/SVG code and our code.
/// We have an additional layer of rust bindings to SVGs and Tikz because they are hard af to draw and manipulate
/// But Figure objects are the first layer that creates objects and is able to translate into both SVG and Tikz
pub struct FigureObject {
    ptr: Rc<dyn FO>,
    pub name: &'static str,
}

pub trait WrappableAsFigureObject {
    fn wrap(self) -> FigureObject where Self: IsFigureObject + Sized + 'static {
        let name = self.name();
        FigureObject {
            ptr: Rc::new(self),
            name
        }
    }
}

impl<T: IsFigureObject + Sized + 'static> WrappableAsFigureObject for T {}

impl Plottable for FigureObject {
    fn tikzify(&self) -> TikzFigure {
        self.ptr.tikzify()
    }

    fn draw_on_canvas(&self, c: CanvasStateHandle)  -> Result<(), DrawError> {
        self.ptr.draw_on_canvas(c)
    }
}

impl FigureObject {
    /// Returns a list of the coordinates of the figure object
    pub fn coordinates(&self) -> Vec<Coordinates> {
        return self.ptr.coordinates();
    }

    /// Returns the ambient dimensions this object lives in.
    pub fn dims(&self) -> usize {
        return self.ptr.dims();
    }

    /// Returns the number of coordinates this thing is composed of
    pub fn len(&self) -> usize {
        return self.ptr.coordinates().len();
    }

    /// Project every coordinate in self according to the projection p
    /// We guarantee the projection object passed to you has dimensions (self.dims() -> _)
    pub fn project(&self, p: Projection) -> Result<FigureObject, DimensionError> {
        if p.input() != self.ptr.dims() {
            return Err(DimensionError {
                msg: format!("Expect the output dimension of the projection {} to be same as the dimension of the {} ({})", p.dims(), self.name, self.dims()),
                source: "project() from FigureObject"
            });
        }

        Ok(self.ptr.project_and_wrap(p))
    }

    /// Project this figure object to a 2-dimensional Plottable object
    pub fn project_to_plot(&self, p: Projection) -> Result<PlottableObject, DimensionError> {
        if p.input() != self.dims() {
            return Err(DimensionError {
                msg: format!("Expect the output dimension of the projection {} to be same as the dimension of the {} ({})", p.dims(), self.name, self.dims()),
                source: "project_to_plot() from FigureObject"
            });
        }

        if p.output() != 2 {
            return Err(DimensionError {
                msg: format!("Expect the output dimension of the projection {} to be 2", p.dims()),
                source: "project_to_plot() from FigureObject"
            });
        }

        let res = self.project(p)?;
        let ptr = Rc::new(res) as Rc<dyn Plottable>;

        let coords = self.coordinates().iter().map(|x| {
            if x.dims != 2 {
                panic!("Expected two dimensional points in Plottable::coords(), found {}", x.dims);
            }

            (x[0], x[1])
        }).collect();

        Ok(PlottableObject { ptr, coords })
    }
}

impl Clone for FigureObject {
    fn clone(&self) -> Self {
        FigureObject {
            ptr: Rc::clone(&self.ptr),
            name: self.name
        }
    }
}

// ==================================================================================================
// ============================== Implement wrap pattern for Drawables ==============================
// ==================================================================================================

/// Drawables are high-level implementations of Figure objects. They contain methods and stuff to implement
/// drawing multiple figure objects in a particular way.
/// If we look at the requirements for a Drawable object, we see we need the draw method, sized, clone, and no lifetime parameters
pub trait Drawable: 'static {
    /// Returns a vector of FigureObject that we will pass to the figure to draw.
    fn draw(&self) -> Vec<FigureObject>;

    /// Returns the dimension that this drawable object lives in
    fn dims(&self) -> usize;

    /// This is useful for debug purposes. It should produce a unique string
    fn repr(&self) -> String;
}

pub trait WrappableAsDrawable {
    /// Consumes ownership of self and returns a drawable object wrapper (a reference counted pointer to the object)
    fn wrap(self) -> DrawableObject where Self: Sized + Drawable + Any + 'static {
        if let Some(s) = (&self as &dyn Any).downcast_ref::<DrawableObject>() {
            s.clone()
        }
        else {
            DrawableObject { obj: Rc::new(self) }
        }
    }
}

impl<T: Drawable + Sized> WrappableAsDrawable for T {}

/// Drawable wrappers are reference counted smart pointers to the object itself.
pub struct DrawableObject {
    obj: Rc<dyn Drawable>
}

impl Drawable for DrawableObject {
    /// Draws this object
    fn draw(&self) -> Vec<FigureObject> {
        return self.obj.draw();
    }

    /// Return the number of dimensions this object lives in
    fn dims(&self) -> usize {
        return self.obj.dims();
    }

    /// Returns a string that uniquely represents this object. This is useful for debug only.
    fn repr(&self) -> String {
        return self.obj.repr();
    }
}

impl Clone for DrawableObject {
    fn clone(&self) -> Self {
        DrawableObject {
            obj: Rc::clone(&self.obj)
        }
    }
}

impl PartialEq for DrawableObject {
    fn eq(&self, other: &Self) -> bool {
        return self.repr() == other.repr();
    }
}

impl Debug for DrawableObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}