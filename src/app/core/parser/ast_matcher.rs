//! This module defines the logic where we try to pattern-match the user's command against a list of predefined commands

use super::ast::*;

/// If AST::matches() returns this, it screams stop immediately
#[derive(Debug, PartialEq)]
pub enum ASTParseError {
    VarOnLeftExpr,
}

/// Gets the list of variables if the structure of the two strings matched,
/// If it doesn't, then return none
/// otherwise return an error
/// This works for precompiled ASTNodes
pub fn copy_args_with_mat(ast1: &ASTNode, ast2: &ASTNode) -> Result<Option<Vec<f64>>, ASTParseError> {
    let mut v = vec![];

    let x = copy_args_recursive(&ast1, &ast2, &mut v)?;

    if !x {
        return Ok(None);
    }

    return Ok(Some(v));
}

/// Returns true if the AST matches, and
fn copy_args_recursive(s: &ASTNode, mat: &ASTNode, result: &mut Vec<f64>) -> Result<bool, ASTParseError> {
    // Honestly it kinda doesn't matter which type of mismatch we get. Hence we simplified the implementation of ast parse errors
    return match (s, mat) {
        (ASTNode::Number(x), ASTNode::Variable) => {
            result.push(x.clone());
            Ok(true)
        },

        // If we were to write variables and identifiers this is the line we would have to change
        (ASTNode::Identifier(x), ASTNode::Variable) => { todo!() },

        (ASTNode::Variable, _) => { return Err(ASTParseError::VarOnLeftExpr) },

        // If the right hand side is anything but a variable, the types and values have to match up
        (ASTNode::Number(x), ASTNode::Number(y)) => return Ok(x == y),

        (ASTNode::Identifier(x), ASTNode::Identifier(y)) => return Ok(x == y),

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
