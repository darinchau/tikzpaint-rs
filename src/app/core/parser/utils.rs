//! Utility functions related to parser

use super::ast::ASTNode;
use crate::core::*;

pub fn print_fn(name: &str, nodes: &Vec<ASTNode>) -> String {
    format!("Function:{}{}", name, nodes.iter().map(|y| format!("({:?})", y)).collect::<Vec<String>>().join(", "))
}