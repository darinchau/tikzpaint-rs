//! Projections are traits that takes coordinates and outputs coordinates
use std::rc::Rc;

use crate::figures::{Coordinates};

/// Basic implementation for a projection function trait object that takes Coordinates of INPUT dimensions and turn
/// them into coordinates of OUTPUT dimensions
pub trait Projection<const INPUT: usize, const OUTPUT: usize> {
    fn call(&self, v: &Coordinates<INPUT>) -> Coordinates<OUTPUT>;
}

struct BoundProjectionDispatcher<'a, const INPUT: usize, const OUTPUT: usize> {
    ptr: Rc<dyn Projection<INPUT, OUTPUT> + 'a>
}

impl<'a, const INPUT: usize, const OUTPUT: usize> Projection<INPUT, OUTPUT> for BoundProjectionDispatcher<'a, INPUT, OUTPUT> {
    fn call(&self, v: &Coordinates<INPUT>) -> Coordinates<OUTPUT> {
        self.ptr.call(v)
    }
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
    /// This allows us to construct new projections from existing projections.
    /// Concat::from(p1, p2) is a projection object that essentially does
    /// proj2(proj1(v)) when called
    /// 
    /// Examples:
    /// ```
    /// use tikzpaint_rs::figures::{Coordinates, Matrix, Concat, Projection};
    /// let x = Coordinates::new(&[3, 4, 5]);
    /// let proj1 = Matrix::new([
    ///     [1, 0, 0],
    ///     [0, 1, 0],
    ///     [0, 0, 2]
    /// ]);
    /// let proj2 = Matrix::new([
    ///     [1, 0, 2],
    ///     [2, -1, 1]
    /// ]);
    /// let proj3 = Concat::from(proj1, proj2);
    /// let y3 = proj3.call(&x);
    /// assert!(y3 == Coordinates::new(&[23, 12]));
    /// ```
    /// 
    /// We can also retreive both projections (sorta) as follows
    /// ```
    /// use tikzpaint_rs::figures::{Coordinates, Matrix, Concat, Projection};
    /// let x = Coordinates::new(&[3, 4, 5]);
    /// let proj1 = Matrix::new([
    ///     [1, 0, 0],
    ///     [0, 1, 0],
    ///     [0, 0, 2]
    /// ]);
    /// let proj2 = Matrix::new([
    ///     [1, 0, 2],
    ///     [2, -1, 1]
    /// ]);
    /// let proj3 = Concat::from(proj1, proj2);
    /// let this_is_also_proj1 = proj3.first();
    /// let y1 = this_is_also_proj1.call(&x);
    /// assert!(y1 == Coordinates::new(&[3, 4, 10]));
    /// 
    /// let this_is_also_proj2 = proj3.second();
    /// let y2 = this_is_also_proj2.call(&x);
    /// assert!(y2 == Coordinates::new(&[13, 7]));
    /// ```
    pub fn from<T, S>(proj1: T, proj2: S) -> Self where
    T: Projection<I, J> + 'a,
    S: Projection<J, K> + 'a {
        let r1 = Rc::new(proj1);
        let r2 = Rc::new(proj2);
        
        let res = Concat {
            proj1: Rc::clone(&r1) as Rc<dyn Projection<I, J>>,
            proj2: Rc::clone(&r2) as Rc<dyn Projection<J, K>>
        };

        res
    }

    /// Returns the first projection object in this chained projection, with the same lifetime as this chained projection object
    /// i.e. the intersection of T and S. Check the documentation for Concat::from for usage.
    pub fn first(&self) -> impl Projection<I, J> + 'a {
        BoundProjectionDispatcher {
            ptr: Rc::clone(&self.proj1)
        }
    }

    /// Returns the second projection object in this chained projection, with the same lifetime as this chained projection object
    /// i.e. the intersection of T and S. Check the documentation for Concat::from for usage.
    pub fn second(&self) -> impl Projection<J, K> + 'a {
        BoundProjectionDispatcher {
            ptr: Rc::clone(&self.proj2)
        }
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
