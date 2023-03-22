//! This submodule defines the variable types and handles the parsing

use std::fmt::Debug;
use std::any::TypeId;
use std::any::Any;
use crate::core::*;
use crate::core::calc::*;

use super::ast::ASTNode;
use super::utils::print_fn;

use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum VariableType {
    Number,
    NumberTuple,
    Variable(ThreadSafeCheapString),
    AST
}

impl Debug for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::Number => write!(f, "Number"),
            VariableType::NumberTuple => write!(f, "NumberTuple"),
            VariableType::Variable(name) => write!(f, "Variable{}", name),
            VariableType::AST => write!(f, "AST")
        }
    }
}

pub enum VariablePayload {
    Number(f64),
    NumberTuple(Vec<f64>),
    Function(ThreadSafeCheapString, Box<dyn Fn(Vec<ASTNode>) -> ASTNode + Send + Sync + 'static>),
    AST(ASTNode)
}

impl Debug for VariablePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariablePayload::Number(y) => write!(f, "Var({})", y),
            VariablePayload::NumberTuple(y) => write!(f, "Var({:?})", y),
            VariablePayload::Function(x, nodes) => write!(f, "Function({})", x),
            VariablePayload::AST(node) => write!(f, "{:?}", node),
        }
    }
}

impl PartialEq<f64> for VariablePayload {
    fn eq(&self, other: &f64) -> bool {
        if let VariablePayload::Number(x) = self {
            return eq(x, other);
        }
        return false;
    }
}

/// Note: This is very convenient but also dangerous. Use only when you are 3000% sure the types match
impl From<&VariablePayload> for f64 {
    fn from(value: &VariablePayload) -> Self {
        match value {
            VariablePayload::Number(x) => *x,
            _ => panic!("Types does not match! Told you not to use implicit conversion :D")
        }
    }
}

/// Note: This is very convenient but also dangerous. Use only when you are 3000% sure the types match
impl From<&VariablePayload> for Vec<f64> {
    fn from(value: &VariablePayload) -> Self {
        match value {
            VariablePayload::NumberTuple(x) => x.clone(),
            _ => panic!("Types does not match! Told you not to use implicit conversion :D")
        }
    }
}