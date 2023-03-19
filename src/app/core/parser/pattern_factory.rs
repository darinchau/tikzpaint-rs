//! This module contains all known patterns

use std::rc::Rc;
use std::{cell::RefCell, sync::Mutex};
use super::ast::{ASTNode, AST};
use crate::figures::*;
use crate::core::*;

type PatternConstructor = Box<dyn Fn(Vec<f64>) -> DrawableObject + Send + Sync>;

/// A pattern is something to match our code against
struct Pattern {
    ptr: PatternConstructor,
    ast: AST
}

impl Pattern {
    fn new<F>(pat: &str, f: F) -> Self where
    F: Fn(Vec<f64>) -> DrawableObject + Send + Sync + 'static {
        let ast = AST::new(pat).expect(&format!("Failed to compile predefined patterns! {}", pat));
        Self {
            ast,
            ptr: Box::new(f) as PatternConstructor
        }
    }

    fn call(&self, v: Vec<f64>) -> DrawableObject {
        return (self.ptr)(v);
    }
}

static PATTERNS: Mutex<Vec<Pattern>> = Mutex::new(Vec::new());

macro_rules! pattern {
    ($s:expr, $f:expr) => {
        {
            let cls = |v| ($f)(v).wrap();
            let pat = Pattern::new($s, cls);
            PATTERNS.lock().unwrap().push(pat);
        }
    };
}

#[derive(PartialEq, Clone, Debug)]
pub enum PatternMatchError {
    NoMatch,
    ASTMatchError(String)
}

/// Performs linear search and try to match every pattern possible
pub fn try_patterns(s: &AST) -> Result<DrawableObject, PatternMatchError> {
    for pat in &*PATTERNS.lock().unwrap() {
        println!("{:?}", &pat.ast);
        match s.matches(&pat.ast) {
            Ok(Some(x)) => {
                return Ok(pat.call(x));
            }

            // AST parse error just means the pattern does not match
            Err(e) => {
                return Err(PatternMatchError::ASTMatchError(format!("Error: {:?}", e)));
            }

            Ok(None) => {
                continue;
            }
        }
    }

    return Err(PatternMatchError::NoMatch);
}

/// This function is called on initialization of the parser. Put patterns here.
pub fn init_pattern_factory() {
    pattern!("point({}, {})", |v: Vec<f64>| Point::new(Coordinates::new(v[0], v[1])));
}