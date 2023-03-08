//! This file contains the definition for the abstract syntax tree of the text

use crate::figures::*;
use lazy_static::lazy_static;
use regex::Regex;

/// Implementation of AST.
pub struct AST {
    root: ASTNode,
}

#[derive(Debug)]
enum ASTNode {
    Number(f64),
    Identifier(CheapString),
    Expression(Vec<ASTNode>),
    Function(CheapString, Vec<ASTNode>)
}

#[derive(Debug)]
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
        let root = from_str(s, 0)?;
        Ok(Self {
            root
        })
    }
}

lazy_static! {
    static ref IS_NUMBER: Regex = Regex::new(r"^-?\d+\.?\d*$").unwrap();
    static ref IS_IDENT: Regex = Regex::new(r"^[A-Za-z_][A-Za-z_0-9]*$").unwrap();
}

/// Creates an ASTNode from a string
/// Offset is purely for error displaying purpopses
/// This does one of the following few things:
/// - If this is a pure string, then this is an identifier
/// - If this contains at least one left or right bracket, then commence the bracket search
/// etc. etc. Returns the ASTNode, or returns an error if the parser failed to parse the code
fn from_str(st: &str, offset: usize) -> Result<ASTNode, ASTError> {
    let s = st.trim();

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

    // Extract all brackets and commas and perform recursion magic
    if s.contains('(') || s.contains(',') {
        return Ok(splice_string(s, offset)?);
    }

    return Err(ASTError {
        error_type: ASTErrorType::InvalidSyntax,
        position: offset,
        message: Some(format!("Failed to match any known patterns - got ({})", s)),
        source: "AST::from_str()"
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
                    position: i,
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
            root.push(from_str(substr.trim(), offset + idx)?);
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
    let mut parts = s.split('(');

    // Unwrap should be safe here because we checked the top level contains a string.
    let fn_ident = parts
        .next()
        .ok_or(ASTError{
            error_type: ASTErrorType::InvalidSyntax,
            position: offset,
            message: Some(String::from("Invalid function identifier")),
            source: "AST::splice_fn_like_args()"
        })?
        .wrap();

    // Add one for the stripped left bracket (
    let mut cum_str_len = fn_ident.len() + 1;

    let mut subnodes = vec![];

    for part in parts {
        let substr = part.strip_suffix(')')
            .map(|s| s.to_string())
            .ok_or_else(|| ASTError{
                error_type: ASTErrorType::InvalidSyntax,
                position: offset,
                message: Some(String::from("Invalid bracketed expression")),
                source: "AST::splice_fn_like_args()"
            })?;

        subnodes.push(from_str(&substr, cum_str_len)?);

        cum_str_len += part.len() + 1;
    }

    return Ok(ASTNode::Function(fn_ident, subnodes))
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
    fn test_1() {
        let result = splice_string("123, point(4, 5, 6), 78, 9", 0).unwrap();
        println!("{:?}", result);
    }
}
