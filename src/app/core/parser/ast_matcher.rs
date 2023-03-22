//! This module defines the logic where we try to pattern-match the user's command against a list of predefined commands

use crate::core::calc::eq;

use super::ast::*;
use super::variables::*;

/// If AST::matches() returns this, it screams stop immediately
#[derive(Debug, PartialEq)]
pub enum ASTParseError {
    VarOnLeftExpr,
}

/// Gets the list of variables if the structure of the two strings matched,
/// If it doesn't, then return none
/// otherwise return an error
/// This works for precompiled ASTNodes
pub fn copy_args_with_mat(ast1: &ASTNode, ast2: &ASTNode) -> Result<Option<Vec<VariablePayload>>, ASTParseError> {
    let mut v = vec![];

    let x = copy_args_recursive(&ast1, &ast2, &mut v)?;

    if !x {
        return Ok(None);
    }

    return Ok(Some(v));
}

/// Returns true if the AST matches, pushing the results in order into the result vector whenever necessary
fn copy_args_recursive(s: &ASTNode, mat: &ASTNode, result: &mut Vec<VariablePayload>) -> Result<bool, ASTParseError> {
    match (s, mat) {
        // If right hand side is expecting an AST, then match everything - we guarantee left hand side cannot expect
        // an AST because there is no way to get an AST variable except for precompiled patterns
        (_, ASTNode::Variable(VariableType::AST)) => {
            result.push(VariablePayload::AST(s.to_owned()));
            Ok(true)
        }

        (ASTNode::Variable(VariableType::AST), _) => {
            unreachable!()
        }

        // For variable on the right hand side, try to convert the AST nodes into variable payloads whenever possible
        // A number can be matched into a number
        (ASTNode::Number(x), ASTNode::Variable(VariableType::Number)) => {
            result.push(VariablePayload::Number(*x));
            Ok(true)
        },

        // A number can be matched into a variable - this is the assignment operation
        // So we bind the variable to a function that takes 0 arguments and gives said number
        (ASTNode::Number(x), ASTNode::Variable(VariableType::Variable(name))) => {
            let number = *x;
            result.push(VariablePayload::Function(name.clone(), Box::new(move |a| {
                ASTNode::Number(number)
            })));

            Ok(true)
        }

        // An expression can only match into a number tuple
        (ASTNode::Expression(x), ASTNode::Variable(VariableType::NumberTuple)) => {
            let mut v = vec![];
            for node in x {
                if let ASTNode::Number(a) = node {
                    v.push(*a);
                }
                else {
                    return Ok(false);
                }
            }

            result.push(VariablePayload::NumberTuple(v));
            Ok(true)
        },

        // A variable cannot appear on the left hand side
        (ASTNode::Variable(_), _) => {
            Err(ASTParseError::VarOnLeftExpr)
        },

        // If the right hand side is anything but a variable, the types and values have to match up
        (ASTNode::Number(x), ASTNode::Number(y)) => {
            Ok(eq(x, y))
        },

        (ASTNode::Expression(x), ASTNode::Expression(y)) => {
            if x.len() != y.len() {
                return Ok(false);
            }

            let n = x.len();
            for i in 0..n {
                if !copy_args_recursive(&x[i], &y[i], result)? {
                    return Ok(false);
                }
            }

            Ok(true)
        }

        (ASTNode::Function(name_x, x), ASTNode::Function(name_y, y)) => {
            // Functions does not match if their number of brackets in their expression is different
            if x.len() != y.len() {
                return Ok(false);
            }

            // Functions does not match if their name is different
            if name_x != name_y {
                return Ok(false);
            }

            let n = x.len();
            for i in 0..n {
                if !copy_args_recursive(&x[i], &y[i], result)? {
                    return Ok(false);
                }
            }

            Ok(true)
        }

        // The remaining cases means the node types does not match up
        _ => Ok(false)
    }
}
