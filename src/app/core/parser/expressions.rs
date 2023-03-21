//! This modules provides the functionify_math() method to turn mathematical expressions into functions

use super::parser_error::ParserError;
use super::utils::*;
use super::ast::IS_NUMBER;

fn functionify_math<'a>(s: &str) -> &'a str {
    // We build a syntax tree in two passes as follows:
    // - If we find + or -, turn it into add and sub
    // - If we find * or /, turn it into mul and div
    // - If we find anything else, leave them intact

    // Example: 1 + 2 * (3 + x) is processed as following:
    //  Recursive: 3 + x -> add(3, x)
    //  Multiplication: 1 + mul(2, add(3, x))
    //  Addition: add(1, mul(2, add(3, x)))
}

#[cfg(test)]
mod test {
    use super::functionify_math;

    #[test]
    fn test_math_1() {
        let test = "1 + 2";
        let expected = "add(1, 2)";
        assert_eq!(functionify_math(test), expected);
    }

    #[test]
    fn test_math_2() {
        let test = "1 + 2 * (3 + x)";
        let expected = "add(1, mul(2, add(3, x)))";
        assert_eq!(functionify_math(test), expected);
    }
}