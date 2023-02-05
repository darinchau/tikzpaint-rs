//! Projections are traits that takes coordinates and outputs coordinates
use std::rc::Rc;

use crate::figures::{Coordinates};

/// Basic implementation for a projection function trait object that takes Coordinates of INPUT dimensions and turn
/// them into coordinates of OUTPUT dimensions
pub trait Projection<const INPUT: usize, const OUTPUT: usize> {
    fn call(&self, v: &Coordinates<INPUT>) -> Coordinates<OUTPUT>;
}

/// A struct signifying the identity projection.
/// 
/// Example
/// ```
/// use tikzpaint_rs::figures::{Identity, Coordinates, Projection};
/// let x = Coordinates::new(&[3, 4, 5]);
/// let y = Coordinates::new(&[3, 4, 5]);
/// let proj = Identity::<3>;
/// assert!(x[0] == proj.call(&y)[0]);
/// assert!(x[1] == proj.call(&y)[1]);
/// assert!(x[2] == proj.call(&y)[2]);
/// ```
pub struct Identity<const DIMS: usize>;
impl<const DIMS: usize> Projection<DIMS, DIMS> for Identity<DIMS> {
    fn call(&self, v: &Coordinates<DIMS>) -> Coordinates<DIMS> {
        return v.clone();
    }
}

pub struct Concat<'a, const I: usize, const J: usize, const K: usize> {
    proj1: Rc<dyn Projection<I, J> + 'a>,
    proj2: Rc<dyn Projection<J, K> + 'a>
}

impl<'a, const I: usize, const J: usize, const K: usize> Concat<'a, I, J, K> {
    /// We return also the RCs because we want people to be able to retreive
    pub fn from<T, S>(proj1: T, proj2: S) -> (Self, Rc<T>, Rc<S>) where
    T: Projection<I, J> + 'a,
    S: Projection<J, K> + 'a {
        let r1 = Rc::new(proj1);
        let r2 = Rc::new(proj2);
        
        let res = Concat {
            proj1: Rc::clone(&r1) as Rc<dyn Projection<I, J>>,
            proj2: Rc::clone(&r2) as Rc<dyn Projection<J, K>>
        };

        (res, r1, r2)
    }
}

impl<'a, const I: usize, const J: usize, const K: usize> Projection<I, K> for Concat<'a, I, J, K> {
    fn call(&self, v: &Coordinates<I>) -> Coordinates<K> {
        let y = self.proj1.call(v);
        self.proj2.call(&y)
    }
}


pub struct Matrix<const I: usize, const J: usize> {
    values: [[f64; J]; I]
}

impl<const I: usize, const J: usize> Matrix<I, J> {
    /// Returns the zero matrix
    pub fn zero<T>() -> Self where
    T: Into<f64> + Copy {
        Matrix {
            values: [[0.; J]; I]
        }
    }

    pub fn new<T>(vals: [[T; J]; I]) -> Self where 
    T: Into<f64> + Copy {
        let mut x = [[0.; J]; I];
        for i in 0..I {
            for j in 0..J {
                x[i][j] = vals[i][j].into();
            }
        }
        Matrix {
            values: x
        }
    }
}

impl<const I: usize, const J: usize> Projection<J, I> for Matrix<I, J> {
    fn call(&self, v: &Coordinates<J>) -> Coordinates<I> {
        let mut w = [0.; I];
        for i in 0..I {
            w[i] = (0..J).into_iter().map(|j| {v[j] * self.values[i][j]}).sum()
        }
        Coordinates::new(&w)
    }
}
