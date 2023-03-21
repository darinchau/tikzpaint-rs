//! Utility functions related to parser

use super::ast::ASTNode;
use crate::core::*;

pub fn print_fn(name: &str, nodes: &Vec<ASTNode>) -> String {
    format!("Function:{}{}", name, nodes.iter().map(|y| format!("({:?})", y)).collect::<Vec<String>>().join(", "))
}

pub trait ExtraStringMethodsForAST {
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

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn test_mathematical_expr() {

    }
}