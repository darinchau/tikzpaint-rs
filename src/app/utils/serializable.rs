//! A trait which, when defined, lets us serialize components/structs into strings and the other way round

/// The serializable trait allows components/struct to turn from or into string.
/// let s = Self::from_str(self.into_str()); should be identical to Clone
pub trait Serializable where
Self: Sized {
    fn into_str(&self) -> String;
    fn from_str(s: &str) -> Option<Self>;
}

