
#[derive(Debug, PartialEq)]
pub enum ParserErrorType {
    /// The submitted string contains one or more invalid commands
    CommandNotFound,

    /// The user tries to draw stuff that is in the wrong dimension
    DimensionError,

    /// Rust just fails to turn the string into a number for some weird reason
    ASTCompilationError,

    /// Found invalid match syntax
    ASTMatchError,

    // Some shit happened during the evaluation of the function
    FunctionEvaluateError,
}

#[derive(Debug)]
pub struct ParserError {
    pub error_type: ParserErrorType,
    pub msg: String,
    pub src: &'static str,
}