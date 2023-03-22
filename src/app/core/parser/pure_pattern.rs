//! This module defines a basic function. We are currently in sort of a functional programming
//! paradigm so the base type is only functions and numbers.

use std::rc::Rc;
use std::{cell::RefCell, sync::Mutex};
use super::ast::*;
use super::impure_pattern::is_name_of_impure_fn;
use super::variables::*;
use crate::app::core::parser::ast_matcher::copy_args_with_mat;
use crate::core::calc::is_zero;
use crate::figures::*;
use crate::core::*;
use lazy_static::lazy_static;

use std::collections::HashSet;

type FunctionBehaviour = dyn Fn(Vec<VariablePayload>) -> Result<ASTNode, FunctionEvaluateError> + Send + Sync;

/// An implementation of a function that evaluates to something
/// This is a functional language :D which means when we assign x = 5
/// we are really assigning x to a function that takes no arguments and evaluates to 5
pub struct Pattern {
    pattern: AST,
    f: Box<FunctionBehaviour>
}

impl Pattern {
    fn call(&self, args: Vec<VariablePayload>) -> Result<ASTNode, FunctionEvaluateError> {
        (self.f)(args)
    }
}

/// A singleton variable lookup table that helps us evaluate all non-drawing functions
pub struct PatternLookup {
    fns: Mutex<Vec<Pattern>>,
    names: Mutex<HashSet<String>>
}

impl PatternLookup {
    pub fn new() -> Self {
        Self {
            fns: Mutex::new(vec![]),
            names: Mutex::new(HashSet::new())
        }
    }


    pub fn push<F>(&self, pattern: &'static str, behavior: F) where
    F: Fn(Vec<VariablePayload>) -> Result<ASTNode, FunctionEvaluateError> + Send + Sync + 'static {
        let ast = AST::new(pattern).expect(&format!("Failed to compile pure pattern: {}", pattern));

        if let ASTNode::Function(ref name, _) = ast.root {
            self.names.lock().unwrap().insert(name.to_owned());

            let pat = Pattern {
                pattern: ast,
                f: Box::new(behavior) as Box<FunctionBehaviour>
            };

            self.fns.lock().unwrap().push(pat);

            return;
        }

        panic!("Precompiled pure pattern not a function")
    }

    /// This is only for predefined functions
    fn push_raw<F>(&self, pattern: ASTNode, behavior: F) where
    F: Fn(Vec<VariablePayload>) -> Result<ASTNode, FunctionEvaluateError> + Send + Sync + 'static {
        let pat = Pattern {
            pattern: AST{root: pattern},
            f: Box::new(behavior) as Box<FunctionBehaviour>
        };

        self.fns.lock().unwrap().push(pat);
    }

    /// Searches through every possible function out there and evaluates it if we find a match
    /// The ASTNode x is guaranteed to be a function
    pub fn evaluate(&self, x: ASTNode) -> Result<ASTNode, FunctionEvaluateError> {
        for f in self.fns.lock().unwrap().iter() {
            if let Some(vars) = f.pattern.matches(&x).map_err(|x| FunctionEvaluateError{msg: format!("{:?}", x)})? {
                return Ok(f.call(vars)?);
            }
        }

        Err(FunctionEvaluateError{msg: format!("Function does not match any known patterns: {:?}", x)})
    }

    /// Returns true if the fn_name corresponds to a (pure) function. This is useful because we want to defer any impure patterns inside the pure pattern function lookup
    pub fn quick_lookup(&self, fn_name: &str) -> bool {
        return self.names.lock().unwrap().contains(fn_name)
    }
}

#[derive(Debug)]
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

pub fn is_name_of_pure_fn(name: &str) -> bool {
    return FUNCTIONS.quick_lookup(name);
}


pub fn initialize_lookup() {
    // Precompiled pattern - the assignment operator
    FUNCTIONS.push_raw(ASTNode::Function("assign".to_string(), vec![
        ASTNode::Variable(VariableType::AST),
        ASTNode::Variable(VariableType::AST)
    ]), |x| {
        let left = {
            if let Some(VariablePayload::AST(left)) = x.get(0) {
                left
            }
            else {
                unreachable!()
            }
        };

        let right = {
            if let Some(VariablePayload::AST(right)) = x.get(1) {
                right
            }
            else {
                unreachable!()
            }
        };

        let match_result = copy_args_with_mat(right, left)
            .map_err(|x| {
                FunctionEvaluateError {
                    msg: format!("{:?}", x)
                }
            })?;

        if match_result.is_none() {
            return Err(FunctionEvaluateError{
                msg: format!("Assignment pattern does not match up: found {:?} on the left and {:?} on the right", left, right)
            });
        }

        let payloads = match_result.unwrap();

        // This has to change in the future - for now we only support the {x} = 5 syntax
        // Technically for this way of implementing we get pattern matching for free
        for payload in payloads {
            match payload {
                // User cannot create AST variable payloads
                VariablePayload::AST(_) => unreachable!(),
                VariablePayload::Function(x, f) => {
                    todo!()
                }
                _ => todo!()
            }
        }

        todo!()
    });

    FUNCTIONS.push("add({})({})", |v: Vec<VariablePayload>| {
        let v0: f64 = (&v[0]).into();
        let v1: f64 = (&v[1]).into();
        return Ok(ASTNode::Number(v0 + v1))
    });

    FUNCTIONS.push("sub({})({})", |v: Vec<VariablePayload>| {
        let v0: f64 = (&v[0]).into();
        let v1: f64 = (&v[1]).into();
        return Ok(ASTNode::Number(v0 - v1))
    });

    FUNCTIONS.push("mul({})({})", |v: Vec<VariablePayload>| {
        let v0: f64 = (&v[0]).into();
        let v1: f64 = (&v[1]).into();
        return Ok(ASTNode::Number(v0 * v1))
    });

    FUNCTIONS.push("div({})({})", |v: Vec<VariablePayload>| {
        let v0: f64 = (&v[0]).into();
        let v1: f64 = (&v[1]).into();
        if is_zero(v1) {
            return Err(FunctionEvaluateError {
                msg: String::from("Cannot divide by zero")
            })
        }
        return Ok(ASTNode::Number(v0 / v1))
    });
}