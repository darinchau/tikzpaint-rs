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
mod pattern_factory;

// Included for reexport
pub use parser_error::ParserError;

use self::pattern_factory::*;

pub fn initialize_parser() {
    init_pattern_factory();
}

pub fn parse(s: CheapString) -> Result<FigureObjectComplex, ParserError> {
    // for pattern in PATTERNS.iter() {
    //     if
    // }
    todo!()
}