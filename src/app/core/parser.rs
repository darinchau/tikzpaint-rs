//! This module provides the parse() method to parse commands
//! All the core implementations is actually in the parser folder. This module mostly only formats and bubbles the error back up only

use crate::figures::*;
use crate::app::*;
use std::rc::Rc;
use std::cell::RefCell;

use regex::Regex;
use lazy_static::lazy_static;
use paste::paste;

mod parser_error;
mod ast;

// Included for reexport
pub use parser_error::ParserError;

/// Enum to hold different result type
enum ResultType {
    /// Corresponds to the arguments in {}
    Number(f64),
    NumberVector(Vec<f64>)
}

/// Regex to replace bracket expressions
lazy_static! {
    static ref BRACKET_REPLACE: Regex = Regex::new(r"\{\*(\d*)\}").unwrap();
}

pub fn parse(s: CheapString) -> Result<FigureObjectComplex, ParserError> {
    todo!()
}