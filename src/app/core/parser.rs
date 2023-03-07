//! This module provides the parse() method to parse commands only

use crate::figures::*;
use crate::app::*;
use std::rc::Rc;
use std::cell::RefCell;

use regex::Regex;
use lazy_static::lazy_static;
use paste::paste;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    /// Not an error per se, but just a signal that we dont need to render anything
    EmptyObject,

    /// The submitted string contains one or more invalid commands
    CommandNotFound{msg: String},

    /// The user tries to draw stuff that is in the wrong dimension
    DimensionError{err: String, src: &'static str},

    ParseNumberError{err: String}
}

#[derive(Debug, PartialEq)]
enum PatternMatchError {
    ParseNumberError{err: String},
    NoMatch,
}

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

/// Preprocess and Compiles regex
fn process_regex(x: &str) -> String {
    // Perform string processing
    // 1. Change brackets to blackslash escaped versions and remove white spaces
    let mut regx: String = String::from(x)
        .replace("(", r"\(")
        .replace(")", r"\)")
        .replace(" ", "");

    println!("{regx}");

    // Replace brackets with the bracketed delimeters
    while let Some(captured) = BRACKET_REPLACE.captures(&regx) {
        println!("{regx}");
        // Number of repetition
        let num = captured[1].parse::<usize>().unwrap();
        let rep = format!(r" *\({}\) *", vec!["{}"; num].join(","));
        println!("{rep}");
        regx = BRACKET_REPLACE.replace(&regx, rep).to_string();
    }

    println!("{regx}");

    let i = regx.matches("{}").count();

    regx = regx.replace("{}", r"( *-?\d*\.?\d* *)");
    regx = format!("^{}$", regx);

    println!("{}", regx);

    return regx;
}

#[cfg(test)]
mod test_compile {
    use super::process_regex;

    #[test]
    fn test_1() {
        process_regex("hiya({*1},{*1})");
    }
}


fn wrap_pattern(s: &str, pattern: Regex) -> Result<Vec<f64>, PatternMatchError> {
    let mut i = 0;
    let capture = pattern.captures(s);

    if capture.is_none() {
        return Err(PatternMatchError::NoMatch);
    }

    let mut res: Vec<f64> = vec![];

    let result = capture.unwrap();

    while let Some(a) = result.get(i+1) {
        let parsed = a.as_str().replace(" ", "");

        match parsed.parse::<f64>() {
            Err(e) => {
                return Err(PatternMatchError::ParseNumberError{err: format!("Failed to parse number as integer, got {}, error: {:?}", parsed, e)})
            }

            Ok(s) => {
                res[i] = s;
            }
        }

        i += 1;
    }

    return Ok(res);
}

/// The gist of this function is to "unformat" the string using an regex expression
/// that we will parse and initialize and compile during compile time
fn pattern(pattern: &'static str, s: &str) -> Result<Vec<f64>, PatternMatchError> {
    let pat = Regex::new(&process_regex(pattern)).unwrap();
    return wrap_pattern(s, pat);
}

pub fn parse(s: CheapString) -> Result<FigureObjectComplex, ParserError> {
    let pat1 = pattern("point({}, {})", &s);
    todo!()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_1() {
        let s = "point(3, 5)";
        let pat1 = pattern("point({}, {})", s).unwrap();

        assert!(pat1[0] == 3.);
        assert!(pat1[1] == 5.);
        assert!(pat1.get(2) == None);

    }

    #[test]
    fn test_point_2() {
        let s = "point(             -4.83   ,          -0.84    )";
        let pat1 = pattern("point({}, {})", s).unwrap();

        assert!(pat1[0] == -4.83);
        assert!(pat1[1] == -0.84);
        assert!(pat1.get(2) == None);

    }

    #[test]
    fn test_point_3() {
        let s = "something(420, 69)";
        let pat1 = pattern("point({}, {})", s);

        assert!(pat1 == Err(PatternMatchError::NoMatch))
    }

    #[test]
    #[should_panic]
    fn test_point_4() {
        let s = "point(42, 0, 69)";
        let pat1 = pattern("point({}, {})", s);

        // assert!(pat1 == Err(PatternMatchError::ParseNumberError { err:  }))
        pat1.unwrap();
    }

    #[test]
    fn test_compile_1() {
        let r = process_regex("hiya({*1},{*1})");
        let reg = Regex::new(&r).unwrap();

        assert!(reg.is_match("hiya((1), (2))"));
        assert!(reg.is_match("hiya((   1 ), (     2))"));
        assert!(reg.is_match("hiya(    (1  )   , (2 ))"));
        assert!(reg.is_match("hiya(  (1 ), (    2)    )"));
    }
}