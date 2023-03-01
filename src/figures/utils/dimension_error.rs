use std::fmt::Display;
use std::fmt::Debug;

/// Implements the dimension error struct. If you are seeing this, it is probably because you screwed up the dimensions somewhere...

pub struct DimensionError {
    pub msg: String,
    pub source: &'static str,
}

impl Display for DimensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n -- from {}\n", self.msg, self.source)
    }
}

impl Debug for DimensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}
