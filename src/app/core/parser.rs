//! This module provides the parse() method to parse commands only

use crate::figures::*;
use crate::app::*;
use std::rc::Rc;
use std::cell::RefCell;

pub enum FactoryParseError {
    EmptyObject,
    CommandNotFound(&'static str),
}

pub fn parse<T: StringLike + 'static>(s: T) -> Result<FigureObjectComplex, FactoryParseError> {
    todo!()
}