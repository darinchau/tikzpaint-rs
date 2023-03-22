//! This module provides the parse() method to parse commands
//! All the core implementations is actually in the parser folder. This module mostly only formats and bubbles the error back up only

use crate::figures::*;
use crate::app::*;
use crate::core::*;

use std::rc::Rc;
use std::cell::RefCell;

use regex::Regex;
use lazy_static::lazy_static;
use paste::paste;

mod ast;
mod impure_pattern;
mod ast_matcher;
mod variables;
mod parser_error;
mod pure_pattern;
mod utils;

use self::impure_pattern::*;
use self::pure_pattern::*;
use self::ast::AST;
pub use self::parser_error::*;


pub fn initialize_parser() {
    initialize_lookup();
    init_pattern_matcher();
}

/// Parses a string into possibly a figure object complex, trying to match every pattern possible
/// If nothing matches, returns a parser error which is like an abstraction of every possible error that could occur
pub fn parse<S: StringLike>(s: S) -> Result<Vec<FigureObjectComplex>, ParserError> {
    // 1. Turn the command into a syntax tree
    let ast = AST::new(&(s.wrap())).map_err( |x| {
        let msg = format!("Parse error: {} - {} (char {})", x.error_type, x.message.unwrap_or_default(), x.position);

        ParserError {
            error_type: ParserErrorType::ASTCompilationError,
            msg,
            src: x.source
        }
    })?;

    // 2. Evaluate all the functions inside expanded
    let expanded = evaluate_all(ast).map_err(|x| {
        ParserError {
            error_type: ParserErrorType::FunctionEvaluateError,
            msg: x.msg,
            src: "parser::parse()"
        }
    })?;

    // 3. Draw everything inside the AST after function evaluation
    let drawables = parse_draw(expanded).map_err(|x| {
        match x {
            PatternMatchError::NoMatch => ParserError {
                error_type: ParserErrorType::CommandNotFound,
                msg: format!("Command not found"),
                src: "parser::parse()"
            },

            PatternMatchError::ASTMatchError(er) => ParserError {
                error_type: ParserErrorType::ASTMatchError,
                msg: format!("Invalid syntax: {}", er),
                src: "parser::parse()"
            }

        }
    })?;

    // 4. Turn all the drawable objects into Figure object complexes
    let focs_to_draw = drawables.into_iter().map(|dr| {
        FigureObjectComplex {
            st: dr.repr().wrap(),
            fo: Rc::new(RefCell::new(dr)),
        }
    }).collect();

    return Ok(focs_to_draw);
}

// Some of these here might not work, because they are features we aim to develop
#[cfg(test)]
mod test {
    use super::*;

    fn compare_tree(result: &str, expected: &str) {
        initialize_parser();
        let ast1 = evaluate_all(AST::new(result).unwrap()).unwrap();
        let ast2 = AST::new(expected).unwrap();

        assert_eq!(ast1, ast2);
    }

    #[test]
    fn test_parse_1() {
        initialize_parser();
        let cmd = "point(3, 5)".wrap();
        let res = parse(cmd).unwrap();
        assert!(res.len() == 1);
        assert!(res[0].fo.borrow().repr() == "point(3, 5)");
    }

    #[test]
    fn test_parse_2() {
        initialize_parser();
        let cmd = "".wrap();
        if let Err(x) = parse(cmd) {
            assert_eq!(x.error_type, ParserErrorType::ASTCompilationError);
        }
        else {
            panic!()
        }
    }

    #[test]
    fn test_parse_3() {
        compare_tree("point(1, add(2, 3))", "point(1, 5)");
    }

    #[test]
    fn test_parse_4() {
        compare_tree("point(1 + 2, 3 + 4 * (5 - 6))", "point(3, -1)");
    }

    #[test]
    fn test_parse_5() {
        let cmd = "{x} = 5, point(3, x)".wrap();
        let res = parse(cmd).unwrap();
    }

    #[test]
    fn test_parse_6() {
        let cmd = "{x} = 5, point(3, x).wrap()".wrap();
        let res = parse(cmd).unwrap();
    }
}