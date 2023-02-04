//! Projections are traits that takes coordinates and outputs coordinates
use crate::figures::{Coordinates};

pub trait Projection<const INPUT: usize, const OUTPUT: usize> {
    fn call(&self, v: &Coordinates<INPUT>) -> Coordinates<OUTPUT>;
}

struct Project<T, S, const INPUT: usize, const J: usize, const OUTPUT: usize> where 
T: Projection<INPUT, J>,
S: Projection<J, OUTPUT>
{
    proj1: T,
    proj2: S,
}

/// A struct signifying the identity projection. This may be removed and merged into the matrix struct later.
struct Identity<const DIMS: usize>;
impl<const DIMS: usize> Projection<DIMS, DIMS> for Identity<DIMS> {
    fn call(&self, v: &Coordinates<DIMS>) -> Coordinates<DIMS> {
        return v.clone();
    }
}



impl<T, S, const INPUT: usize, const J: usize, const OUTPUT: usize> Projection<INPUT, OUTPUT> for Project<T, S, INPUT, J, OUTPUT> where 
T: Projection<INPUT, J>,
S: Projection<J, OUTPUT>
{
    fn call(&self, v: &Coordinates<INPUT>) -> Coordinates<OUTPUT> {
        self.proj2.call(&self.proj1.call(v))
    }
}

pub fn concat<const INPUT: usize, const J: usize, const OUTPUT: usize>
    (p1: impl Projection<INPUT, J>, p2: impl Projection<J, OUTPUT>) -> impl Projection<INPUT, OUTPUT> {
    Project {
        proj1: p1,
        proj2: p2
    }
}