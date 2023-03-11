//! This module provides the parse() method to parse commands
//! All the core implementations is actually in the parser folder. This module mostly only formats and bubbles the error back up only

use crate::figures::*;
use crate::app::*;
use std::rc::Rc;
use std::cell::RefCell;

use regex::Regex;
use lazy_static::lazy_static;
use paste::paste;

mod ast;
mod pattern_factory;
mod ast_matcher;

use self::pattern_factory::*;
use self::ast::AST;

#[derive(Debug, PartialEq)]
pub enum ParserErrorType {
    /// Not an error per se, but just a signal that we dont need to render anything
    EmptyObject,

    /// The submitted string contains one or more invalid commands
    CommandNotFound,

    /// The user tries to draw stuff that is in the wrong dimension
    DimensionError,

    /// Rust just fails to turn the string into a number for some weird reason
    ASTCompilationError,
}

#[derive(Debug)]
pub struct ParserError {
    pub error_type: ParserErrorType,
    pub msg: String,
    pub src: &'static str,
}

pub fn initialize_parser() {
    init_pattern_factory();
}

/// Parses a string into possibly a figure object complex, trying to match every pattern possible
/// If nothing matches, returns a parser error which is like an abstraction of every possible error that could occur
pub fn parse(s: CheapString) -> Result<FigureObjectComplex, ParserError> {
    let ast = AST::new(&s).map_err( |x| {
        let msg = format!("Parse error: {} - {} (char {})", x.error_type, x.message.unwrap_or_default(), x.position);

        ParserError {
            error_type: ParserErrorType::ASTCompilationError,
            msg,
            src: x.source
        }
    })?;

    let r = try_patterns(&ast).map_err(|x| {
        match x {
            PatternMatchError::NoMatch => ParserError {
                error_type: ParserErrorType::CommandNotFound,
                msg: format!("Command not found"),
                src: "parser::parse()"
            },

            PatternMatchError::ASTMatchError(er) => {
                ParserError {
                    error_type: ParserErrorType::CommandNotFound,
                    msg: format!("Command not found"),
                    src: "parser::parse()"
                }
            }
        }
    })?;

    let foc = FigureObjectComplex {
        st: s,
        fo: Rc::new(RefCell::new(r)),
    };

    return Ok(foc);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_1() {
        let cmd = "point(3, 5)".wrap();
        let res = parse(cmd).unwrap();
    }
}