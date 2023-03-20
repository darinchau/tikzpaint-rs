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
    // Honestly it kinda doesn't matter which type of mismatch we get. Hence we simplified the implementation of ast parse errors
    match (s, mat) {
        (ASTNode::Number(x), ASTNode::Variable(t)) => {
            // A number can only match a variable that is supposed to store a number
            if *t != VariableType::Number {
                return Ok(false);
            }
            result.push(VariablePayload::Number(*x));
            Ok(true)
        },

        (ASTNode::Expression(x), ASTNode::Variable(t)) => {
            // Number cannot match an entire expression
            match *t {
                VariableType::Number => { Ok(false) }
                VariableType::NumberTuple => { try_match_number_tuple_expr(x, result) }
                VariableType::Function(_, _) => { Ok(false) }
            }
        },

        (ASTNode::Function(x, v), ASTNode::Variable(t)) => {
            match *t {
                VariableType::Number => { todo!() }
                VariableType::NumberTuple => { Ok(false) }
                VariableType::Function(ref name, ref node) => {
                    if x == name && v.len() == node.len() {
                        todo!()
                    }

                    Ok(false)
                }
            }
        },

        (ASTNode::Variable(_), _) => { return Err(ASTParseError::VarOnLeftExpr) },

        // If the right hand side is anything but a variable, the types and values have to match up
        (ASTNode::Number(x), ASTNode::Number(y)) => return Ok(eq(x, y)),

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

            return Ok(true);
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

            return Ok(true);
        }

        // The remaining cases means the node types does not match up
        _ => return Ok(false)
    }
}

fn try_match_number_tuple_expr(x: &Vec<ASTNode>, result: &Vec<VariablePayload>) -> Result<bool, ASTParseError> {
    todo!()
}
