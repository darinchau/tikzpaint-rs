//! This module defines a basic function. We are currently in sort of a functional programming
//! paradigm so the base type is only functions and numbers.

use std::rc::Rc;
use std::{cell::RefCell, sync::Mutex};
use super::ast::*;
use super::impure_pattern::is_name_of_impure_fn;
use super::variables::*;
use crate::figures::*;
use crate::core::*;
use lazy_static::lazy_static;

type FunctionBehaviour = dyn Fn(Vec<VariablePayload>) -> ASTNode + Send + Sync;

/// An implementation of a function that evaluates to something
/// This is a functional language :D which means when we assign x = 5
/// we are really assigning x to a function that takes no arguments and evaluates to 5
pub struct Pattern {
    pattern: AST,
    f: Box<FunctionBehaviour>
}

impl Pattern {
    fn call(&self, args: Vec<VariablePayload>) -> ASTNode {
        (self.f)(args)
    }
}

/// A singleton variable lookup table that helps us evaluate all non-drawing functions
pub struct PatternLookup {
    fns: Mutex<Vec<Pattern>>
}

impl PatternLookup {
    pub fn new() -> Self {
        Self {
            fns: Mutex::new(vec![])
        }
    }

    pub fn push<F>(&self, pattern: &'static str, behavior: F) where
    F: Fn(Vec<VariablePayload>) -> ASTNode + Send + Sync + 'static {
        let ast = AST::new(pattern).expect("Failed to compile predefined patterns :(");

        // This guarantees we have a function
        match ast.root {
            ASTNode::Function(_, _) => {},
            _ => unreachable!()
        }

        let f = Pattern {
            pattern: ast,
            f: Box::new(behavior)
        };

        self.fns.lock().unwrap().push(f);
    }

    /// Searches through every possible function out there and evaluates it if we find a match
    /// The ASTNode x is guaranteed to be a function
    pub fn evaluate(&self, x: ASTNode) -> Result<ASTNode, FunctionEvaluateError> {
        for f in self.fns.lock().unwrap().iter() {
            if let Some(vars) = f.pattern.matches(&x).map_err(|x| FunctionEvaluateError{msg: format!("{:?}", x)})? {
                return Ok(f.call(vars));
            }
        }

        Err(FunctionEvaluateError{msg: format!("Function does not match any known patterns: {:?}", x)})
    }
}

pub struct FunctionEvaluateError {
    pub msg: String
}

lazy_static! {
    static ref FUNCTIONS: PatternLookup = PatternLookup::new();
}

/// This evaluates a function in the ASTNode recursively
fn eval_recursive(node: ASTNode) -> Result<ASTNode, FunctionEvaluateError> {
    match node {
        ASTNode::Function(name, args) => {
            // First evaluate all the arguments recursively, then evaluate the big one
            let mut evaluated = vec![];
            for arg in args.into_iter() {
                evaluated.push(eval_recursive(arg)?);
            }

            // Check if we need to defer the evaluation of this function
            let need_defer = is_name_of_impure_fn(&name);

            let modified_node = ASTNode::Function(name, evaluated);

            if need_defer {
                return Ok(modified_node);
            }

            Ok( FUNCTIONS.evaluate(modified_node)? )
        },

        ASTNode::Expression(v) => {
            let mut evaluated = vec![];
            for arg in v.into_iter() {
                evaluated.push(eval_recursive(arg)?);
            }
            return Ok(ASTNode::Expression(evaluated));
        },

        _ => Ok(node)
    }
}

/// This evaluates all function in the AST
pub fn evaluate_all(x: AST) -> Result<AST, FunctionEvaluateError> {
    Ok(AST {
        root: eval_recursive(x.root)?
    })
}

pub fn initialize_lookup() {
    FUNCTIONS.push("add({}, {})", |v: Vec<VariablePayload>| {
        let v0: f64 = (&v[0]).into();
        let v1: f64 = (&v[1]).into();
        return ASTNode::Number(v0 + v1)
    });
}