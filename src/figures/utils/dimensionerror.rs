use std::fmt::Display;

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
