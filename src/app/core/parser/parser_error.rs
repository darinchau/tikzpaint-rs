#[derive(Debug, PartialEq)]
pub enum ParserError {
    /// Not an error per se, but just a signal that we dont need to render anything
    EmptyObject,

    /// The submitted string contains one or more invalid commands
    CommandNotFound{msg: String},

    /// The user tries to draw stuff that is in the wrong dimension
    DimensionError{err: String, src: &'static str},

    ParseNumberError{err: String}
}
