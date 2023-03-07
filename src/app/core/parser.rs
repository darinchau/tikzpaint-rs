//! This module provides the parse() method to parse commands only

use crate::figures::*;
use crate::app::*;
use std::rc::Rc;
use std::cell::RefCell;

use regex::Regex;
use lazy_static::lazy_static;
use paste::paste;

pub enum ParserError {
    /// Not an error per se, but just a signal that we dont need to render anything
    EmptyObject,

    /// The submitted string contains one or more invalid commands
    CommandNotFound{msg: CheapString},

    /// The user tries to draw stuff that is in the wrong dimension
    DimensionError{err: CheapString, src: &'static str},

    ParseNumberError{err: &'static str}
}

/// This macro returns
macro_rules! pattern {
    ($x:expr, $s:ident) => {
        // The gist of this macro is to "unformat" the string using an regex expression
        // that we will parse and initialize and compile during compile time
        // wrap the whole thing in a giant expression to avoid namespace problems
        {
            fn wrap_pattern(s: CheapString) -> Result<Vec<f64>, ParserError> {
                lazy_static! {
                    static ref PATTERN: (Regex, usize) = {
                        // Perform string processing
                        let mut regx: String = String::from($x)
                            .replace(" ", "");

                        let i = regx.matches("{}").count();

                        regx = regx.replace("{}", r"( *-?\d*\.?\d*)");

                        (Regex::new(&regx).expect("Regex did not compile"), i)
                    };
                }

                let mat = PATTERN.0.captures(&s).unwrap();
                let mut res = vec![0.; PATTERN.1];

                for i in 0..(PATTERN.1) {
                    let a = mat.get(i);
                    if a.is_none() {
                        return Err(ParserError::ParseNumberError{err: "Failed to get matching number pattern"})
                    }

                    let b = a.unwrap().as_str().parse::<f64>();
                    if let Err(e) = b {
                        return Err(ParserError::ParseNumberError{err: "Failed to parse number as integer"})
                    }

                    res[i] = b.unwrap();
                }

                return Ok(res);
            }

            wrap_pattern($s)
        }
    };
}

pub fn parse(s: CheapString) -> Result<FigureObjectComplex, ParserError> {
    let pat1 = pattern!("point({}, {})", s);
    todo!()
}