//! Hashables exist purely to support comparison of figures - this allows us to rerender only if the figure has changed
//! Ideally hashs are fast to compute, because they are recomputed many many times, while serialize are slow, because they
//! are only computed during saves

pub trait Hashable {
    fn hash(&self) -> i64;
}

impl Hashable for f64 {
    fn hash(&self) -> i64 {
        return i64::from_be_bytes((*self * 1597.).round().to_be_bytes());
    }
}