//! This module contains all functions that draw stuff.
//! A function that draws something is like a print statement - outputs something, impure, and expands to nothing
//! The implementation is a mirror of pure patterns

use std::collections::HashSet;
use std::rc::Rc;
use std::{cell::RefCell, sync::Mutex};
use super::ast::*;
use super::variables::*;
use crate::figures::*;
use crate::core::*;
use lazy_static::lazy_static;

type ImpureFunctionBehavior = Box<dyn Fn(Vec<VariablePayload>) -> DrawableObject + Send + Sync>;

/// A pattern is something to match our code against
pub struct ImpurePattern {
    ptr: ImpureFunctionBehavior,
    pattern: AST
}

impl ImpurePattern {
    fn call(&self, v: Vec<VariablePayload>) -> DrawableObject {
        return (self.ptr)(v);
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum PatternMatchError {
    NoMatch,
    ASTMatchError(String)
}

/// A singleton function parser that helps us match and evaluate functions
pub struct ImpurePatternLookup {
    fns: Mutex<Vec<ImpurePattern>>,
    names: Mutex<HashSet<String>>
}

impl ImpurePatternLookup {
    pub fn new() -> Self {
        Self {
            fns: Mutex::new(vec![]),
            names: Mutex::new(HashSet::new())
        }
    }

    pub fn push<F, S>(&self, pattern: &'static str, behavior: F) where
    F: Fn(Vec<VariablePayload>) -> S + Send + Sync + 'static,
    S: Drawable + WrapAsDrawable {
        let ast = AST::new(pattern).expect(&format!("Failed to compile predefined impure pattern: {}", pattern));

        if let ASTNode::Function(ref name, _) = ast.root {
            self.names.lock().unwrap().insert(name.to_owned());

            let pat = ImpurePattern {
                pattern: ast,
                ptr: Box::new(move |x| (behavior)(x).wrap()) as ImpureFunctionBehavior
            };

            self.fns.lock().unwrap().push(pat);

            return;
        }

        panic!("Precompiled impure pattern not a function")
    }

    /// Searches through every possible patterns out there and evaluates it if we find a match
    /// The ASTNode x is guaranteed to be a function
    pub fn evaluate(&self, x: ASTNode) -> Result<DrawableObject, PatternMatchError> {
        for f in self.fns.lock().unwrap().iter() {
            if let Some(vars) = f.pattern.matches(&x).map_err(|x| PatternMatchError::ASTMatchError(format!("{:?}", x)))? {
                return Ok(f.call(vars));
            }
        }

        Err(PatternMatchError::NoMatch)
    }

    /// Returns true if the fn_name corresponds to a (impure) function. This is useful because we want to defer any impure patterns inside the pure pattern function lookup
    pub fn quick_lookup(&self, fn_name: &str) -> bool {
        return self.names.lock().unwrap().contains(fn_name)
    }
}

lazy_static! {
    static ref PATTERNS: ImpurePatternLookup = ImpurePatternLookup::new();
}


/// Evaluates all the impure patterns in the vector recursively. Since only Impure patterns should be left at this point
/// If we receive NoMatch, it should be fatal too
fn eval_recursive(node: ASTNode, v: &mut Vec<DrawableObject>) -> Result<(), PatternMatchError> {
    match node {
        ASTNode::Function(_, _) => {
            // An impure function must only take numbers and what nots as input. So we won't evaluate recursively
            v.push(PATTERNS.evaluate(node)?);
            Ok(())
        },

        ASTNode::Expression(args) => {
            for arg in args.into_iter() {
                eval_recursive(arg, v)?;
            }

            Ok(())
        },

        _ => Ok(())
    }
}

/// Collects everything that should be drawn. The drawable objects are collected post-order
/// Since this should be the last step, we consume the AST
pub fn parse_draw(s: AST) -> Result<Vec<DrawableObject>, PatternMatchError> {
    let mut v = vec![];
    eval_recursive(s.root, &mut v)?;
    return Ok(v);
}

pub fn is_name_of_impure_fn(name: &str) -> bool {
    return PATTERNS.quick_lookup(name);
}

/// This function is called on initialization of the parser. Put patterns here.
pub fn init_pattern_matcher() {
    PATTERNS.push("point({}, {})", |v: Vec<VariablePayload>| {
        Point::new(Coordinates::new(&v[0], &v[1]))
    });
}