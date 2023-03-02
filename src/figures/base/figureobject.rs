//! This file holds the base implementation of a displayable
//! If you want to define your own layer zero struct, you must implement two traits:
//! 1. FigureObject - an object that holds coordinates in DIMS dimension
//! 2. Plot - The figure will transform DIM coordinates into 2 dimensions - implement Plot to turn it into everything else to plot it on screen
use crate::figures::*;
use std::rc::Rc;
use std::any::Any;

/// A plottable object is the last step before output. This is like the final state of the object to say we are about to plot stuff
/// At this point we guarantee that we only have x coordinates and y coordinates
pub trait Plottable {
    /// Define the construction of Tikz code from an object
    fn tikzify(&self) -> String;

    /// Defines the construction of SVG from an object
    fn get_svg(&self) -> SVG;
}

#[derive(Clone)]
/// Plottable object is like a universal wrapper around a Plottable
pub struct PlottableObject {
    ptr: Rc<dyn Plottable>,
    coords: Vec<(f64, f64)>
}

impl PlottableObject {
    pub fn tikzify(&self) -> String {
        return self.ptr.tikzify();
    }

    pub fn get_svg(&self) -> SVG {
        return self.ptr.get_svg();
    }

    /// Gets and returns the coordinates as an array
    /// This is like extra promise that we only have x and y coordinates
    /// For the default implementation, this function will panic if the resulting coordinates are not all in 2 dimensions
    pub fn coordinates(&self) -> &Vec<(f64, f64)> {
        return &self.coords;
    }
}


// ===============================================================================================================================
// ===============================================================================================================================
// ===============================================================================================================================

/// A figure object is the base object (Layer 0 interface) between Tikz/SVG code and our code.
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
    fn project(&self, p: Projection) -> FigureObject;
}

/// A figure object is the base object (Layer 0 interface) between Tikz/SVG code and our code.
pub struct FigureObject {
    ptr: Rc<dyn IsFigureObject>,
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
    fn tikzify(&self) -> String {
        self.ptr.tikzify()
    }

    fn get_svg(&self) -> SVG {
        todo!()
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

        Ok(self.ptr.project(p))
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
    fn draw(&self) -> Vec<FigureObject> {
        return self.obj.draw();
    }

    fn dims(&self) -> usize {
        return self.obj.dims();
    }
}

impl Clone for DrawableObject {
    fn clone(&self) -> Self {
        DrawableObject {
            obj: Rc::clone(&self.obj)
        }
    }
}
