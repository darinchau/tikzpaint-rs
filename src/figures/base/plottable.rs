//! Figure objects are the first layer of objects between tikz code and our code. They are responsible for handling projections.
//! Plottables must translate directly into tikz code and svg code and whatever
//! Drawable objects are high level abstractions of figure objects.

use crate::figures::*;
use crate::renderer::*;
use std::rc::Rc;
use std::any::Any;
use std::fmt::Debug;