//! Drawables are high-level implementations of Figure objects. They contain methods and stuff to implement
//! drawing multiple figure objects in a particular way.
//! If we look at the requirements for a Drawable object, we see we need the draw method, sized, clone, and no lifetime parameters

use crate::figures::*;
use std::any::Any;
use std::rc::Rc;
use std::fmt::Debug;

/// High level implementations of figure objects. They are insulated from migration hell
pub trait Drawable: 'static {
    /// Returns a vector of PlottableObject that we will pass to the figure to draw.
    fn draw(&self) -> Vec<PlottableObject>;

    /// This is useful for debug purposes. It should produce a unique string
    fn repr(&self) -> String;
}

pub trait WrapAsDrawable {
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

impl<T: Sized + Drawable + Any + 'static> WrapAsDrawable for T {}

/// Drawable wrappers are reference counted smart pointers to the object itself.
pub struct DrawableObject {
    obj: Rc<dyn Drawable>
}

impl Drawable for DrawableObject {
    /// Draws this object
    fn draw(&self) -> Vec<PlottableObject> {
        return self.obj.draw();
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