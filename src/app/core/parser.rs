//! This module provides the parse() method to parse commands only

use crate::figures::*;
use crate::app::*;
use std::rc::Rc;
use std::cell::RefCell;

pub enum ParserError {
    /// Not an error per se, but just a signal that we dont need to render anything
    EmptyObject,

    /// The submitted string contains one or more invalid commands
    CommandNotFound{msg: CheapString},

    /// The user tries to draw stuff that is in the wrong dimension
    DimensionError{err: CheapString, src: &'static str}
}

pub fn parse(s: CheapString) -> Result<FigureObjectComplex, ParserError> {
    todo!()
}