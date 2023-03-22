//! This submodule defines the variable types and handles the parsing

use std::fmt::Debug;
use std::any::TypeId;
use std::any::Any;
use crate::core::*;
use crate::core::calc::*;

use super::ast::ASTNode;
use super::pure_pattern::FunctionEvaluateError;
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

pub struct FunctionPayload {
    pub num_layers: usize,
    pub name: ThreadSafeCheapString,
    pub f: Box<dyn Fn(Vec<VariablePayload>) -> ASTNode + Send + Sync + 'static>
}

impl Debug for FunctionPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function:{}({})", self.name, self.num_layers)
    }
}

impl FunctionPayload {
    pub fn pattern(&self) -> ASTNode {
        ASTNode::Function(self.name.to_string(), vec![ASTNode::Variable(VariableType::AST); self.num_layers])
    }

    pub fn call(&self, loads: Vec<VariablePayload>) -> ASTNode {
        (self.f)(loads)
    }
}

pub enum VariablePayload {
    Number(f64),
    NumberTuple(Vec<f64>),
    Function(FunctionPayload),
    AST(ASTNode)
}

impl Debug for VariablePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariablePayload::Number(y) => write!(f, "Var({})", y),
            VariablePayload::NumberTuple(y) => write!(f, "Var({:?})", y),
            VariablePayload::Function(x) => write!(f, "Function({:?})", x),
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

impl VariablePayload {
    pub fn float(&self) -> Result<f64, FunctionEvaluateError> {
        if let VariablePayload::Number(x) = self {
            return Ok(*x);
        }

        Err(FunctionEvaluateError {
            msg: format!("Unknown error: type mismatch for f64")
        })
    }

    pub fn ast<'a>(&'a self) -> Result<&'a ASTNode, FunctionEvaluateError> {
        if let VariablePayload::AST(x) = self {
            return Ok(x);
        }

        Err(FunctionEvaluateError {
            msg: format!("Unknown error: type mismatch for f64")
        })
    }

    pub fn tuple<'a>(&'a self) -> Result<&'a Vec<f64>, FunctionEvaluateError> {
        if let VariablePayload::NumberTuple(x) = self {
            return Ok(x);
        }

        Err(FunctionEvaluateError {
            msg: format!("Unknown error: type mismatch for f64")
        })
    }
}
