//! This file contains the definition for the abstract syntax tree of the text
//! Construct an AST by AST::new() and try to match an AST by AST::matches()

use crate::figures::*;
use crate::core::calc::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt::{Debug, Display}, rc::Rc};
use super::ast_matcher::{copy_args_with_mat, ASTParseError};
use paste::paste;
use super::utils::print_fn;

use super::variables::*;

/// Implementation of AST.
pub struct AST {
    pub root: ASTNode,
}

#[derive(PartialEq, Clone)]
pub enum ASTNode {
    Number(f64),
    Expression(Vec<ASTNode>),
    Function(String, Vec<ASTNode>),

    /// A variable purely exists for formatting expressions using AST
    Variable(VariableType),
}

impl Debug for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Number(x) => write!(f, "Number({})", x),
            ASTNode::Expression(x) => write!(f, "Expression({})", x.iter().map(|y| format!("{:?}", y)).collect::<Vec<String>>().join(", ")),
            ASTNode::Function(name, x) => write!(f, "{}", print_fn(name, x)),
            ASTNode::Variable(x) => write!(f, "Variable({:?})", x),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ASTErrorType {
    /// We found the unclosed left bracket
    UnclosedBrackets,

    /// We found unopened right brackets
    ExtraBrackets,

    /// We found mismatching bracket types, for example ( closed by }
    BracketsMismatch,

    /// The usize denotes the start of the number
    ParseNumberFail,

    /// Basically this means unknown error
    InvalidSyntax,

    /// Invalid syntax inside a variable
    InvalidVariableSyntax
}

impl Display for ASTErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s =  match self {
            Self::UnclosedBrackets => String::from("Found unclosed left bracket"),
            Self::ExtraBrackets => String::from("Found extra right bracket"),
            Self::ParseNumberFail => String::from("Failed to parse number"),
            Self::InvalidSyntax => String::from("Invalid syntax -"),
            Self::InvalidVariableSyntax => String::from("Invalid variable syntax -"),
            Self::BracketsMismatch => String::from("Mismatching brackets")
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub struct ASTError {
    pub error_type: ASTErrorType,

    // Position which the error occur
    pub position: usize,

    // Optional message
    pub message: Option<String>,

    pub source: &'static str,
}

impl AST {
    /// Creates an ASTNode from a string
    /// Offset is purely for error displaying purpopses
    /// This does one of the following few things:
    /// - If this is a pure string, then this is an identifier
    /// - If this contains at least one left or right bracket, then commence the bracket search
    /// etc. etc. Returns the ASTNode, or returns an error if the parser failed to parse the code

    pub fn new(s: &str) -> Result<AST, ASTError> {
        let root = ASTNode::from_str(s)?;
        Ok(Self {
            root
        })
    }
}

impl Debug for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.root)
    }
}

lazy_static! {
    static ref IS_NUMBER: Regex = Regex::new(r"^-?\d+\.?\d*$").unwrap();
    static ref IS_POSITIVE_INT: Regex = Regex::new(r"^\d+$").unwrap();
    static ref IS_IDENT: Regex = Regex::new(r"^[A-Za-z_][A-Za-z_0-9]*$").unwrap();
    static ref IS_BRACKETED: Regex = Regex::new(r"^\(.*\)$").unwrap();
}

impl ASTNode {
    /// This function exists for testing only
    fn from_str(st: &str) -> Result<ASTNode, ASTError> {
        return ASTNode::from_str_recursive(st, 0);
    }

    fn from_str_recursive(st: &str, offset: usize) -> Result<ASTNode, ASTError> {
        let s = st.trim();

        // - Is it a variable?
        if s.check_brackets("{}") {
            return handle_variable(s, offset);
        }

        // - Is it a number?
        if IS_NUMBER.is_match(s) {
            let num = parse_number(s, offset)?;
            return Ok(ASTNode::Number(num));
        }

        // - Is it a pure string (i.e. valid variable name in rust)
        if IS_IDENT.is_match(s) {
            let ident = parse_ident(s, offset)?;
            return Ok(ASTNode::Function(ident, vec![]));
        }

        // This is to handle a special case:
        // If the expression is surrounded in brackets, then remove the brackets and wrap that in an expression
        // But if what is inside is already an expression, then no need to wrap them in extra brackets
        if s.check_brackets("()") {
            let node = ASTNode::from_str_recursive(&s[1..s.len()-1], offset)?;
            return Ok(node);
        }

        // Extract all brackets and commas and perform recursion magic
        if s.contains_one_of("(){},+-*/") {
            return Ok(splice_complex_expr(s, offset)?);
        }

        return Err(ASTError {
            error_type: ASTErrorType::InvalidSyntax,
            position: offset,
            message: Some(format!("Failed to match any known patterns - got ({})", s)),
            source: "AST::from_str()"
        });
    }
}

trait ExtraStringMethodsForAST {
    /// Returns true if s contains one of the characters in chars
    fn contains_one_of(&self, chars: &'static str) -> bool;

    /// This removes the surrounding curly brackets of a string. This panics if the string is not surrounded by curly brackets.
    fn remove_curly_brackets(&self) -> Self;

    /// Returns true if there is a "top-level" bracket surrounding an expression
    /// (1, 2, 3) is true
    /// (1)(2) is false
    /// 1, 2, 3 is false
    /// ((1), 2) is true
    fn check_brackets(&self, delimeters: &str) -> bool;
}

impl ExtraStringMethodsForAST for &str {
    fn contains_one_of(&self, chars: &'static str) -> bool {
        for x in chars.chars() {
            if self.contains(x) {
                return true;
            }
        }

        return false;
    }

    #[inline(always)]
    fn remove_curly_brackets(&self) -> Self {
        let len = self.len();
        if len >= 2 && &self[0..1] == "{" && &self[len-1..] == "}" {
            &self[1..len-1]
        } else {
            panic!("The string should be surrounded by curly brackets")
        }
    }

    /// Returns true if there is a "top-level" bracket surrounding an expression
    /// (1, 2, 3) is true
    /// (1)(2) is false
    /// 1, 2, 3 is false
    /// ((1), 2) is true
    fn check_brackets(&self, delimeters: &str) -> bool {
        let (left, right) = {
            let mut c = delimeters.chars();
            (c.next().unwrap(), c.next().unwrap())
        };

        if !self.starts_with(left) {
            return false;
        }

        let mut brackets_count = 0;
        for (i, c) in self.chars().enumerate() {
            if c == left {
                brackets_count += 1;
            }
            else if c == right {
                brackets_count -= 1;
                if brackets_count == 0 {
                    return i == self.len() - 1;
                }
            }
        }

        return brackets_count == 0;
    }
}

fn handle_variable(st: &str, offset: usize) -> Result<ASTNode, ASTError> {
    let contents = st.remove_curly_brackets();

    // If contents is white space, then it should match a single number
    if contents.trim().is_empty() {
        return Ok(ASTNode::Variable(VariableType::Number));
    }

    // If contents contains a number n, then it should match an expression that contains n numbers
    if IS_POSITIVE_INT.is_match(contents) {
        let n = contents.parse::<usize>().map_err(|x| {
            ASTError {
                error_type: ASTErrorType::ParseNumberFail,
                position: offset + 1,
                message: Some(format!("Failed to parse the number ({}) as a positive integer", st)),
                source: "AST::handle_variable()"
            }
        })?;

        if n == 0 {
            return Err(ASTError {
                error_type: ASTErrorType::InvalidSyntax,
                position: offset + 1,
                message: Some(format!("Cannot match 0 variables")),
                source: "AST::handle_variable()"
            })
        }

        // This is a hacky way of producing the desired syntax tree - doing the dirty work and expanding during compile time
        return Ok(ASTNode::Expression(vec![ASTNode::Variable(VariableType::Number); n]));
    }

    // If contents contains a single *, then let it match an arbitrary number of variables
    if contents == "*" {
        return Ok(ASTNode::Variable(VariableType::NumberTuple));
    }

    // if contents contain a valid variable name, this means some point in the near future we will try to assign
    // the variable with a number/expression/whatever.
    // Store the variable name first
    if IS_IDENT.is_match(contents) {
        return Ok(ASTNode::Variable(VariableType::Function(contents.to_string(), vec![])));
    }

    return Err(ASTError {
        error_type: ASTErrorType::InvalidVariableSyntax,
        position: offset + 1,
        message: Some(format!("Unknown variable syntax: ({})", contents)),
        source: "AST::handle_variable()"
    });
}

/// It is known that s is a number. Make that into an AST node
fn parse_number(s: &str, offset: usize) -> Result<f64, ASTError> {
    s.parse::<f64>().map_err(|x| ASTError {
        error_type: ASTErrorType::ParseNumberFail,
        position: offset,
        message: Some(format!("Got {}", s)),
        source: "AST::parse_number()"
    })
}

/// It is known that s is a valid variable name. Make that into an AST node
fn parse_ident(s: &str, offset: usize) -> Result<String, ASTError> {
    Ok(s.to_string())
}

#[derive(Clone, Copy, PartialEq)]
enum BracketTypes {
    Round,
    Curly
}

impl Display for BracketTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BracketTypes::Curly => write!(f, "{{"),
            BracketTypes::Round => write!(f, "(")
        }
    }
}

impl Debug for BracketTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BracketTypes::Curly => write!(f, "}}"),
            BracketTypes::Round => write!(f, ")")
        }
    }
}

fn handle_brackets_close(brackets: &mut Vec<(BracketTypes, usize)>, expected: BracketTypes, pos: usize) -> Result<(), ASTError> {
    if let Some((ty, _)) = brackets.pop() {
        if ty != expected {
            return Err(ASTError {
                error_type: ASTErrorType::BracketsMismatch,
                position: pos,
                message: Some(format!("Found mismatching brackets {} closed by {:?}", expected, ty)),
                source: "AST::splice_at_top_level_delim()"
            });
        }

        return Ok(());
    }

    return Err(ASTError {
        error_type: ASTErrorType::ExtraBrackets,
        position: pos,
        message: Some(format!("Found extra right bracket {:?} without corresponding open brackets", expected)),
        source: "AST::splice_at_top_level_delim()"
    });
}

/// Splice at all top level delimeter. Returns an error if the bracket types does not match or we found dangling brackets
fn splice_at_top_level_delim<'a>(s: &'a str, offset: usize, delim: char) -> Result<Vec<(&'a str, usize)>, ASTError> {
    let mut substrs = vec![];

    // The position of the first character of this substring
    let mut first_substr_pos = 0;

    let mut brackets = vec![];

    for (i, c) in s.chars().enumerate() {
        // Use the stack implementation to check
        if c == '(' {
            brackets.push((BracketTypes::Round, i));
        }
        if c == ')' {
            handle_brackets_close(&mut brackets, BracketTypes::Round, offset + i)?;
        }

        // Handle curly brackets
        if c == '{' {
            brackets.push((BracketTypes::Curly, i));
        }
        if c == '}' {
            handle_brackets_close(&mut brackets, BracketTypes::Curly, offset + i)?;
        }

        else if c == delim {
            // Splice if it is a top level bracket
            if brackets.len() == 0 {
                let substr = &s[first_substr_pos..i];
                substrs.push((substr, first_substr_pos + offset));
                first_substr_pos = i + 1;

            }
        }
    }

    if brackets.len() > 0 {
        let (ty, pos) = brackets[0];
        return Err(ASTError {
            error_type: ASTErrorType::UnclosedBrackets,
            position: offset + pos,
            message: Some(format!("Found unclosed bracket {}", ty)),
            source: "AST::splice_at_top_level_delim()"
        });
    }

    // That means we found at least one top level commas
    if first_substr_pos > 0 {
        substrs.push((&s[first_substr_pos..], first_substr_pos));
    }

    return Ok(substrs);
}

/// This means we encountered a complex expression. We want to splice the string at top-level bracket commas
fn splice_complex_expr(s: &str, offset: usize) -> Result<ASTNode, ASTError> {
    // Use a two-pass strategy
    // First pass is for commas
    let substrs = splice_at_top_level_delim(s, offset, ',')?;

    let mut root: Vec<ASTNode> = vec![];

    // That means we found at least one top level commas
    if substrs.len() > 1 {
        for (substr, idx) in substrs.into_iter() {
            root.push(ASTNode::from_str_recursive(substr, offset + idx)?);
        }

        return Ok(ASTNode::Expression(root));
    }

    // Second pass
    return Ok(splice_fn_like_args(s, offset)?);
}

/// Handles the function call syntax - funct(call1)(call2)...(calln)
fn splice_fn_like_args(s: &str, offset: usize) -> Result<ASTNode, ASTError> {
    // Second pass - if we reach here this means there is no top level commas
    // So an expression must be of the form identifier(node)(node)...(node)
    let mut left_bracket_pos = None;
    let mut brackets_count = 0;
    let mut parts = vec![];

    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            if left_bracket_pos.is_none() {
                left_bracket_pos = Some(i);
            }
            brackets_count += 1;
        }
        else if c == ')' {
            // - if the bracket is not closed, bubble up an error
            brackets_count -= 1;
            if brackets_count < 0 {
                return Err(ASTError {
                    error_type: ASTErrorType::ExtraBrackets,
                    position: offset + i,
                    message: None,
                    source: "AST::splice_fn_like_args()"
                });
            }
            if brackets_count == 0 {
                if let Some(lbp) = left_bracket_pos {
                    parts.push(&s[lbp+1..i]);
                    left_bracket_pos = None;
                }
                else {
                    return Err(ASTError {
                        error_type: ASTErrorType::InvalidSyntax,
                        position: offset + i,
                        message: Some(String::from("Unknown error encountered")),
                        source: "AST::splice_fn_like_args()"
                    });
                }

            }
        }
    }

    let fn_ident = (&s[0..s.find('(').unwrap()]).trim().to_string();

    // Add one for the stripped left bracket (
    let mut cum_str_len = fn_ident.len() + 1;

    let mut subnodes = vec![];

    for part in parts {
        subnodes.push(ASTNode::from_str_recursive(part, cum_str_len)?);

        cum_str_len += part.len() + 2;
    }

    return Ok(ASTNode::Function(fn_ident, subnodes))
}

impl AST {
    /// Returns a vector containing the matching pattern if the pattern matches, otherwise return None
    /// Raises an error if there is anything wrong with the syntax
    /// 'self' should be the one with variables
    pub fn matches(&self, s: &ASTNode) -> Result<Option<Vec<VariablePayload>>, ASTParseError> {
        return copy_args_with_mat(&s, &self.root);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_regex_1() {
        assert_eq!(IS_NUMBER.is_match("123"), true);
        assert_eq!(IS_NUMBER.is_match("-2.9"), true);
        assert_eq!(IS_NUMBER.is_match("3.1415926535"), true);
        assert_eq!(IS_NUMBER.is_match("420.6969.420"), false);
        assert_eq!(IS_NUMBER.is_match("123.456 78"), false);
        assert_eq!(IS_NUMBER.is_match("1 2 3 4 5"), false);
        assert_eq!(IS_NUMBER.is_match("1a"), false);
        assert_eq!(IS_NUMBER.is_match("point(4, 5, 6)"), false);
        assert_eq!(IS_NUMBER.is_match("2 + 3i"), false);
    }

    #[test]
    fn test_regex_2() {
        assert_eq!(IS_IDENT.is_match("hello"), true);
        assert_eq!(IS_IDENT.is_match("world"), true);
        assert_eq!(IS_IDENT.is_match("_foo"), true);
        assert_eq!(IS_IDENT.is_match("bar123"), true);
        assert_eq!(IS_IDENT.is_match("i_am_a_valid_variable_name"), true);
        assert_eq!(IS_IDENT.is_match(""), false);
        assert_eq!(IS_IDENT.is_match("123"), false);
        assert_eq!(IS_IDENT.is_match("foo bar"), false);
        assert_eq!(IS_IDENT.is_match("heya="), false);
        assert_eq!(IS_IDENT.is_match("my-variable"), false);
        assert_eq!(IS_IDENT.is_match("1i"), false);
    }

    #[test]
    fn test_check_bracket() {
        let delims = "()";
        assert_eq!("(s)".check_brackets(delims), true);
        assert_eq!("(x, y)".check_brackets(delims), true);
        assert_eq!("3x + y".check_brackets(delims), false);
        assert_eq!("(x), (y)".check_brackets(delims), false);
        assert_eq!("((x), (y))".check_brackets(delims), true);
        assert_eq!("(((()()())())())".check_brackets(delims), true);
        assert_eq!("(".check_brackets(delims), false);
    }

    fn compare_spliced(result: Vec<(&'static str, usize)>, expected: Vec<&'static str>) -> bool {
        for ((s0, _), t0) in result.into_iter().zip(expected.into_iter()) {
            if s0.trim() != t0.trim() {
                return false;
            }
        }

        return true;
    }

    #[test]
    fn test_splice_1() {
        let st = "1, 2, (3, 4), 5, (6, (7, 8))";
        let expected = vec!["1", "2", "(3, 4)", "5", "(6, (7, 8))"];
        let result = splice_at_top_level_delim(st,  0, ',').unwrap();
        assert!(compare_spliced(result, expected));
    }

    #[test]
    fn test_splice_2() {
        let st = "1, {2, 3}, (4, {5, 6})";
        let expected = vec!["1", "{2, 3}", "(4, {5, 6})"];
        let result = splice_at_top_level_delim(st,  0, ',').unwrap();
        assert!(compare_spliced(result, expected));
    }

    #[test]
    fn test_compile_ast1() {
        let result = ASTNode::from_str("123, point(4, 5, 6), 78, 9").unwrap();
        let expected = ASTNode::Expression(vec![
            ASTNode::Number(123.),
            ASTNode::Function("point".to_string(), vec![
                ASTNode::Expression(vec![
                    ASTNode::Number(4.),
                    ASTNode::Number(5.),
                    ASTNode::Number(6.)
                ])
            ]),
            ASTNode::Number(78.),
            ASTNode::Number(9.)
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_ast2() {
        let result = ASTNode::from_str("F(f)(x)").unwrap();
        let expected = ASTNode::Function("F".to_string(), vec![
            ASTNode::Function("f".to_string(), vec![]),
            ASTNode::Function("x".to_string(), vec![])
        ]);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_compile_ast3() {
        let result = ASTNode::from_str(" F  ( f )   ( x    )").unwrap();
        let expected = ASTNode::Function("F".to_string(), vec![
            ASTNode::Function("f".to_string(), vec![]),
            ASTNode::Function("x".to_string(), vec![])
        ]);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_compile_ast4() {
        let result = ASTNode::from_str(",");
        let er = result.err().unwrap();
        assert_eq!(er.error_type, ASTErrorType::InvalidSyntax)
    }

    #[test]
    fn test_compile_ast5() {
        let result = ASTNode::from_str("forgot)to_close_right_bracket");
        let er = result.err().unwrap();
        assert_eq!(er.error_type, ASTErrorType::ExtraBrackets);
        assert_eq!(er.position, 6);
    }

    #[test]
    fn test_compile_ast6() {
        let result = ASTNode::from_str("ex,left(bracket()");
        let er = result.err().unwrap();
        assert_eq!(er.error_type, ASTErrorType::UnclosedBrackets);
        // Position of first left bracket
        assert_eq!(er.position, 7);
    }

    #[test]
    fn test_compile_ast7() {
        let s = "point(3, 5)";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Function("point".to_string(), vec![
            ASTNode::Expression(vec![
                ASTNode::Number(3.),
                ASTNode::Number(5.)
            ])
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_ast8() {
        let s = "point({}, {})";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Function("point".to_string(), vec![
            ASTNode::Expression(vec![
                ASTNode::Variable(VariableType::Number),
                ASTNode::Variable(VariableType::Number)
            ])
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_ast9() {
        let s = "f((x))";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Function("f".to_string(), vec![
            ASTNode::Function("x".to_string(), vec![])
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_ast10() {
        let s = "(x), (x, y)";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Expression(vec![
            ASTNode::Function("x".to_string(), vec![]),
            ASTNode::Expression(vec![
                ASTNode::Function("x".to_string(), vec![]),
                ASTNode::Function("y".to_string(), vec![])
            ])
        ]);
        assert_eq!(result, expected);
    }


    #[test]
    fn test_compile_ast11() {
        let s = "point({}, hiya(4, ({}, 6))), f(({}, 8), {})";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Expression(vec![
            ASTNode::Function("point".to_string(), vec![
                ASTNode::Expression(vec![
                    ASTNode::Variable(VariableType::Number),
                    ASTNode::Function("hiya".to_string(), vec![
                        ASTNode::Expression(vec![
                            ASTNode::Number(4.),
                            ASTNode::Expression(vec![
                                ASTNode::Variable(VariableType::Number),
                                ASTNode::Number(6.)
                            ])
                        ])
                    ])
                ])
            ]),
            ASTNode::Function("f".to_string(), vec![
                ASTNode::Expression(vec![
                    ASTNode::Expression(vec![
                        ASTNode::Variable(VariableType::Number),
                        ASTNode::Number(8.)
                    ]),
                    ASTNode::Variable(VariableType::Number)
                ])
            ])
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_ast12() {
        let s = "{}, {*}, {3}";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Expression(vec![
            ASTNode::Variable(VariableType::Number),
            ASTNode::Variable(VariableType::NumberTuple),
            ASTNode::Expression(vec![
                ASTNode::Variable(VariableType::Number),
                ASTNode::Variable(VariableType::Number),
                ASTNode::Variable(VariableType::Number)
            ])
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_ast13() {
        let s = "fn({*})";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Function(String::from("fn"), vec![
            ASTNode::Variable(VariableType::NumberTuple)
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_ast14() {
        let s = "fn({x})";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Function(String::from("fn"), vec![
            ASTNode::Variable(VariableType::Function("x".to_string(), vec![]))
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_1() {
        let s1 = "point(3, 5)";
        let s2 = "point({}, {})";
        let ast1 = ASTNode::from_str(s1).unwrap();
        let ast2 = ASTNode::from_str(s2).unwrap();
        let result = copy_args_with_mat(&ast1, &ast2).unwrap().unwrap();
        assert_eq!(result[0], 3.);
        assert_eq!(result[1], 5.);
    }

    #[test]
    fn test_parse_2() {
        let s1 = "point(3, hiya(4, (5, 6))), f((7, 8), 9)";
        let s2 = "point({}, hiya(4, ({}, 6))), f(({}, 8), {})";
        let ast1 = ASTNode::from_str(s1).unwrap();
        let ast2 = ASTNode::from_str(s2).unwrap();
        let result = copy_args_with_mat(&ast1, &ast2).unwrap().unwrap();
        assert_eq!(result[0], 3.);
        assert_eq!(result[1], 5.);
        assert_eq!(result[2], 7.);
        assert_eq!(result[3], 9.);
    }

    #[test]
    fn test_parse_4() {
        let s1 = "point(0, 1, 2, 3, 4, 5, -6.)";
        let s2 = "point({7})";
        let ast1 = ASTNode::from_str(s1).unwrap();
        let ast2 = ASTNode::from_str(s2).unwrap();
        let result = copy_args_with_mat(&ast1, &ast2).unwrap().unwrap();
        assert_eq!(result[0], 0.);
        assert_eq!(result[1], 1.);
        assert_eq!(result[2], 2.);
        assert_eq!(result[3], 3.);
        assert_eq!(result[4], 4.);
        assert_eq!(result[5], 5.);
        assert_eq!(result[6], -6.);
    }
}
