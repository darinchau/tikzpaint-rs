//! This module defines the logic where we try to pattern-match the user's command against a list of predefined commands

use super::ast::*;

/// Gets the list of variables if the structure of the two strings matched, otherwise return an error
/// This works for precompiled ASTNodes
pub fn copy_args_with_mat(ast1: &ASTNode, ast2: &ASTNode) -> Result<Vec<f64>, ASTParseError> {
    let mut v = vec![];

    copy_args_recursive(&ast1, &ast2, &mut v)?;

    return Ok(v);
}

fn copy_args_recursive(s: &ASTNode, mat: &ASTNode, result: &mut Vec<f64>) -> Result<(), ASTParseError> {
    match (s, mat) {
        (ASTNode::Number(x), ASTNode::Variable) => {result.push(x.clone())},

        // If we were to write variables and identifiers this is the line we would have to change
        (ASTNode::Identifier(x), ASTNode::Variable) => { todo!() },

        (ASTNode::Expression(_), ASTNode::Variable) => { return Err(ASTParseError::VarCannotMatchExpr) },
        (ASTNode::Function(_, _), ASTNode::Variable) => { return Err(ASTParseError::VarCannotMatchFn) },
        (ASTNode::Variable, _) => { return Err(ASTParseError::VarOnLeftExpr)},

        // If the right hand side is anything but a variable, the types and values have to match up
        (ASTNode::Number(x), ASTNode::Number(y)) => {
            if x != y {
                return Err(ASTParseError::NumberMismatch(*x, *y))
            }
        },

        (ASTNode::Identifier(x), ASTNode::Identifier(y)) => {
            if x != y {
                return Err(ASTParseError::IdentMismatch(x.clone(), y.clone()))
            }
        },

        (ASTNode::Expression(x), ASTNode::Expression(y)) => {
            if x.len() != y.len() {
                return Err(ASTParseError::NumChildrenMismatch(x.len(), y.len()))
            }

            let n = x.len();
            for i in 0..n {
                copy_args_recursive(&x[i], &y[i], result)?;
            }
        }

        (ASTNode::Function(name_x, x), ASTNode::Function(name_y, y)) => {
            if x.len() != y.len() {
                return Err(ASTParseError::FnTypeMismatch(x.len(), y.len()))
            }

            if name_x != name_y {
                return Err(ASTParseError::FnNameMismatch(name_x.clone(), name_y.clone()))
            }

            let n = x.len();
            for i in 0..n {
                copy_args_recursive(&x[i], &y[i], result)?;
            }
        }

        (x, y) => { return Err(ASTParseError::TypeMismatch(format!("{:?}", x), format!("{:?}", y))) }
    }

    Ok(())
}