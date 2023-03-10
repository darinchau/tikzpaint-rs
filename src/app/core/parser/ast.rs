//! This file contains the definition for the abstract syntax tree of the text

use crate::figures::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Debug;

/// Implementation of AST.
pub struct AST {
    root: ASTNode,
}

#[derive(PartialEq)]
pub enum ASTNode {
    Number(f64),
    Identifier(CheapString),
    Expression(Vec<ASTNode>),
    Function(CheapString, Vec<ASTNode>),

    /// A variable purely exists for unformatting expressions using AST
    Variable
}

impl Debug for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Number(x) => write!(f, "Number({})", x),
            ASTNode::Expression(x) => write!(f, "Expression({})", x.iter().map(|y| format!("{:?}", y)).collect::<Vec<String>>().join(", ")),
            ASTNode::Function(name, x) => write!(f, "Function:{}({})", name, x.iter().map(|y| format!("{:?}", y)).collect::<Vec<String>>().join(", ")),
            ASTNode::Identifier(x) => write!(f, "Identifier({})", x),
            ASTNode::Variable => write!(f, "Variable")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ASTErrorType {
    /// The usize denotes the position which we found the unclosed left bracket
    BracketNotClosed,

    /// The usize denotes the position which we found the right bracket
    ExtraRightBracket,

    /// The usize denotes the start of the number
    ParseNumberFail,

    /// Basically this means unknown error
    InvalidSyntax,
}

#[derive(Debug)]
pub struct ASTError {
    error_type: ASTErrorType,

    // Position which the error occur
    position: usize,

    // Optional message
    message: Option<String>,

    source: &'static str,
}

impl AST {
    pub fn new(s: &str) -> Result<AST, ASTError> {
        let root = ASTNode::from_str(s)?;
        Ok(Self {
            root
        })
    }
}

lazy_static! {
    static ref IS_NUMBER: Regex = Regex::new(r"^-?\d+\.?\d*$").unwrap();
    static ref IS_IDENT: Regex = Regex::new(r"^[A-Za-z_][A-Za-z_0-9]*$").unwrap();
    static ref IS_BRACKETED: Regex = Regex::new(r"^\(.*\)$").unwrap();
}

impl ASTNode {
    /// Creates an ASTNode from a string
    /// Offset is purely for error displaying purpopses
    /// This does one of the following few things:
    /// - If this is a pure string, then this is an identifier
    /// - If this contains at least one left or right bracket, then commence the bracket search
    /// etc. etc. Returns the ASTNode, or returns an error if the parser failed to parse the code
    fn from_str(st: &str) -> Result<ASTNode, ASTError> {
        return ASTNode::from_str_recursive(st, 0);
    }

    fn from_str_recursive(st: &str, offset: usize) -> Result<ASTNode, ASTError> {
        let s = st.trim();

        // - Is it a variable?
        if s == "{}" {
            return Ok(ASTNode::Variable)
        }

        // - Is it a number?
        if IS_NUMBER.is_match(s) {
            let num = parse_number(s, offset)?;
            return Ok(ASTNode::Number(num));
        }

        // - Is it a pure string (i.e. valid variable name in rust)
        if IS_IDENT.is_match(s) {
            let ident = parse_ident(s, offset)?.wrap();
            return Ok(ASTNode::Identifier(ident));
        }

        // This is to handle a special case:
        // If the expression is surrounded in brackets, then remove the brackets and wrap that in an expression
        // But if what is inside is already an expression, then no need to wrap them in extra brackets
        if check_brackets(s) {
            let node = ASTNode::from_str_recursive(&s[1..s.len()-1], offset)?;
            return Ok(node);
        }

        // Extract all brackets and commas and perform recursion magic
        if s.contains('(') || s.contains(',') {
            return Ok(splice_string(s, offset)?);
        }

        // This means s has an unclosed right bracket
        if s.contains(')') {
            return Err(ASTError {
                error_type: ASTErrorType::ExtraRightBracket,
                position: offset + s.find(')').unwrap(),
                message: None,
                source: "AST::from_str()"
            });
        }

        return Err(ASTError {
            error_type: ASTErrorType::InvalidSyntax,
            position: offset,
            message: Some(format!("Failed to match any known patterns - got ({})", s)),
            source: "AST::from_str()"
        });
    }
}

fn check_brackets(s: &str) -> bool {
    if !s.starts_with('(') {
        return false;
    }

    let mut brackets_count = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            brackets_count += 1;
        }
        else if c == ')' {
            brackets_count -= 1;
            if brackets_count == 0 {
                return i == s.len() - 1;
            }
        }
    }

    return brackets_count == 0;
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

/// This means we encountered a complex expression. We want to splice the string at top-level bracket commas
fn splice_string(s: &str, offset: usize) -> Result<ASTNode, ASTError> {
    // Use a two-pass strategy
    // First pass is for commas
    let mut left_bracket_pos = None;
    let mut brackets_count = 0;

    let mut root: Vec<ASTNode> = vec![];

    let mut substrs = vec![];

    // The position of the first character of this substring
    let mut first_substr_pos = 0;

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
                    error_type: ASTErrorType::ExtraRightBracket,
                    position: offset + i,
                    message: None,
                    source: "AST::splice_string()"
                });
            }
        }
        else if c == ',' {
            // Splice if it is a top level bracket
            if brackets_count == 0 {
                let substr = &s[first_substr_pos..i];
                substrs.push((substr, first_substr_pos + offset));
                first_substr_pos = i + 1;

            }
        }
    }

    // That means we found at least one top level commas
    if first_substr_pos > 0 {
        substrs.push((&s[first_substr_pos..], first_substr_pos));

        for (substr, idx) in substrs.into_iter() {
            root.push(ASTNode::from_str_recursive(substr.trim(), offset + idx)?);
        }

        return Ok(ASTNode::Expression(root));
    }

    // Catch a left bracket thing if we found
    if let Some(i) = left_bracket_pos {
        if brackets_count > 0 {
            return Err(ASTError {
                error_type: ASTErrorType::BracketNotClosed,
                position: offset + i,
                message: None,
                source: "AST::splice_str()"
            });
        }
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
                    error_type: ASTErrorType::ExtraRightBracket,
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

    let fn_ident = (&s[0..s.find('(').unwrap()]).trim().wrap();

    // Add one for the stripped left bracket (
    let mut cum_str_len = fn_ident.len() + 1;

    let mut subnodes = vec![];

    for part in parts {
        subnodes.push(ASTNode::from_str_recursive(part, cum_str_len)?);

        cum_str_len += part.len() + 2;
    }

    return Ok(ASTNode::Function(fn_ident, subnodes))
}

#[derive(Debug, PartialEq)]
pub enum ASTParseError {
    VarCannotMatchExpr,
    VarCannotMatchFn,
    VarOnLeftExpr,
    NumberMismatch(f64, f64),
    IdentMismatch(CheapString, CheapString),
    NumChildrenMismatch(usize, usize),
    FnTypeMismatch(usize, usize),
    FnNameMismatch(CheapString, CheapString),
    TypeMismatch(CheapString, CheapString)
}

impl AST {
    /// Returns a vector containing the matching pattern if the pattern matches, otherwise return None
    /// Raises an error if there is anything wrong with the syntax
    fn matches(&self, s: &AST) -> Result<Vec<f64>, ASTError> {
        return copy_args_with_mat(&self.root, &s.root);
    }
}

/// Gets the list of variables if the structure of the two strings matched, otherwise return an error
/// This works for precompiled ASTNodes
fn copy_args_with_mat(ast1: &ASTNode, ast2: &ASTNode) -> Result<Vec<f64>, ASTError> {
    let mut v = vec![];

    copy_args_recursive(&ast1, &ast2, &mut v).map_err(|x| {
        match x {
            _ => todo!()
        }
    });

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

        (x, y) => { return Err(ASTParseError::TypeMismatch(format!("{:?}", x).wrap(), format!("{:?}", y).wrap())) }
    }

    Ok(())
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
    }

    #[test]
    fn test_check_bracket() {
        assert_eq!(check_brackets("(s)"), true);
        assert_eq!(check_brackets("(x, y)"), true);
        assert_eq!(check_brackets("3x + y"), false);
        assert_eq!(check_brackets("(x), (y)"), false);
        assert_eq!(check_brackets("((x), (y))"), true);
        assert_eq!(check_brackets("(((()()())())())"), true);
        assert_eq!(check_brackets("("), false);
    }

    #[test]
    fn test_compile_ast1() {
        let result = ASTNode::from_str("123, point(4, 5, 6), 78, 9").unwrap();
        let expected = ASTNode::Expression(vec![
            ASTNode::Number(123.),
            ASTNode::Function("point".wrap(), vec![
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
        let expected = ASTNode::Function("F".wrap(), vec![
            ASTNode::Identifier("f".wrap()),
            ASTNode::Identifier("x".wrap())
        ]);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_compile_ast3() {
        let result = ASTNode::from_str(" F  ( f )   ( x    )").unwrap();
        let expected = ASTNode::Function("F".wrap(), vec![
            ASTNode::Identifier("f".wrap()),
            ASTNode::Identifier("x".wrap())
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
        assert_eq!(er.error_type, ASTErrorType::ExtraRightBracket);
        assert_eq!(er.position, 6);
    }

    #[test]
    fn test_compile_ast6() {
        let result = ASTNode::from_str("ex,left(bracket()");
        let er = result.err().unwrap();
        assert_eq!(er.error_type, ASTErrorType::BracketNotClosed);
        // Position of first left bracket
        assert_eq!(er.position, 7);
    }

    #[test]
    fn test_compile_ast7() {
        let s = "point(3, 5)";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Function("point".wrap(), vec![
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
        let expected = ASTNode::Function("point".wrap(), vec![
            ASTNode::Expression(vec![
                ASTNode::Variable,
                ASTNode::Variable
            ])
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_ast9() {
        let s = "f((x))";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Function("f".wrap(), vec![
            ASTNode::Identifier("x".wrap())
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_ast10() {
        let s = "(x), (x, y)";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Expression(vec![
            ASTNode::Identifier("x".wrap()),
            ASTNode::Expression(vec![
                ASTNode::Identifier("x".wrap()),
                ASTNode::Identifier("y".wrap())
            ])
        ]);
        assert_eq!(result, expected);
    }


    #[test]
    fn test_compile_ast11() {
        let s = "point({}, hiya(4, ({}, 6))), f(({}, 8), {})";
        let result = ASTNode::from_str(s).unwrap();
        let expected = ASTNode::Expression(vec![
            ASTNode::Function("point".wrap(), vec![
                ASTNode::Expression(vec![
                    ASTNode::Variable,
                    ASTNode::Function("hiya".wrap(), vec![
                        ASTNode::Expression(vec![
                            ASTNode::Number(4.),
                            ASTNode::Expression(vec![
                                ASTNode::Variable,
                                ASTNode::Number(6.)
                            ])
                        ])
                    ])
                ])
            ]),
            ASTNode::Function("f".wrap(), vec![
                ASTNode::Expression(vec![
                    ASTNode::Expression(vec![
                        ASTNode::Variable,
                        ASTNode::Number(8.)
                    ]),
                    ASTNode::Variable
                ])
            ])
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_1() {
        let s1 = "point(3, 5)";
        let s2 = "point({}, {})";
        let ast1 = ASTNode::from_str(s1).unwrap();
        let ast2 = ASTNode::from_str(s2).unwrap();
        let result = copy_args_with_mat(&ast1, &ast2).unwrap();
        assert_eq!(result[0], 3.);
        assert_eq!(result[1], 5.);
    }

    #[test]
    fn test_parse_2() {
        let s1 = "point(3, hiya(4, (5, 6))), f((7, 8), 9)";
        let s2 = "point({}, hiya(4, ({}, 6))), f(({}, 8), {})";
        let ast1 = ASTNode::from_str(s1).unwrap();
        let ast2 = ASTNode::from_str(s2).unwrap();
        let result = copy_args_with_mat(&ast1, &ast2).unwrap();
        assert_eq!(result[0], 3.);
        assert_eq!(result[1], 5.);
        assert_eq!(result[2], 7.);
        assert_eq!(result[3], 9.);
    }
}
