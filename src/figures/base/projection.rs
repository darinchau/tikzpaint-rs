//! Projections are traits that takes coordinates and outputs coordinates
use std::rc::Rc;

use crate::figures::{Coordinates, DimensionError};

/// Basic implementation for a projection function trait object that takes Coordinates of INPUT dimensions and turn
/// them into coordinates of OUTPUT dimensions. Implicitly we also have to satisfy Sized.
pub trait IsProjection where
Self: 'static {
    /// Returns the input dimension of this projection
    fn input(&self) -> usize;
    /// Returns the output dimension of this projection
    fn output(&self) -> usize;
    /// Performs the projection. We guarantee the coordinates passed into this trait has correct number of dimensions.
    fn call(&self, v: &Coordinates) -> Coordinates;
}

pub trait WrappableAsProjection {
    fn wrap(self) -> Projection where
        Self: Sized + IsProjection {
        Projection { obj: Rc::new(self) }
    }
}

impl<T: IsProjection + Sized> WrappableAsProjection for T {}

/// You probably got a generic projection because you called the wrap() function on an object with IsProjection.
/// Internally this is just an Rc wrapping a dynamic trait object. Hence cloning is very cheap.
pub struct Projection {
    obj: Rc<dyn IsProjection>
}

impl Clone for Projection {
    fn clone(&self) -> Self {
        let obj = Rc::clone(&self.obj);
        return Projection {
            obj
        }
    }
}

impl Projection {
    pub fn input(&self) -> usize {
        self.obj.input()
    }

    pub fn output(&self) -> usize {
        self.obj.output()
    }

    pub fn call(&self, v: &Coordinates) -> Result<Coordinates, DimensionError>{
        if v.dims != self.input() {
            return Err(DimensionError{
                msg: format!("Found incorrect input dimensions. Expect {}, found {}", self.input(), v.dims),
                source: "call() from Projection"
            });
        }

        Ok(self.obj.call(v))
    }

    /// Used for outputing error message
    pub fn dims(&self) -> String {
        format!("({} -> {})", self.input(), self.output())
    }
}

/// A struct signifying the identity projection.
///
/// Example
/// ```
/// use tikzpaint_rs::figures::{Identity, Coordinates, Projection, IsProjection};
/// use tikzpaint_rs::figures::WrappableAsProjection;
/// let x = Coordinates::new(vec![3, 4, 5]);
/// let y = Coordinates::new(vec![3, 4, 5]);
/// let proj = Identity{ dims: 3 }.wrap();
/// let projected = proj.call(&y).unwrap();
/// assert!(x[0] == projected[0]);
/// assert!(x[1] == projected[1]);
/// assert!(x[2] == projected[2]);
/// ```
pub struct Identity {
    pub dims: usize,
}

impl IsProjection for Identity {
    fn call(&self, v: &Coordinates) -> Coordinates {
        v.clone()
    }

    fn input(&self) -> usize {
        self.dims
    }

    fn output(&self) -> usize {
        self.dims
    }
}

///The concat type allows us to construct new projections from existing projections.
pub struct Concat {
    proj1: Projection,
    proj2: Projection
}

impl Concat {
    /// This allows us to construct new projections from existing projections.
    /// Concat::from(p1, p2) is a projection object that essentially does
    /// proj2(proj1(v)) when called
    ///
    /// Examples:
    /// ```
    /// use tikzpaint_rs::figures::{Coordinates, Matrix, Concat, Projection, IsProjection};
    ///
    /// let x = Coordinates::new(vec![3, 4, 5]);
    /// let proj1 = Matrix::array([
    ///     [1, 0, 0],
    ///     [0, 1, 0],
    ///     [0, 0, 2]
    /// ]);
    /// let proj2 = Matrix::array([
    ///     [1, 0, 2],
    ///     [2, -1, 1]
    /// ]);
    /// let proj3 = Concat::from(proj1, proj2).unwrap();
    /// let y3 = proj3.call(&x);
    /// assert!(y3 == Coordinates::new(vec![23, 12]));
    /// ```
    pub fn from<P1: IsProjection, P2: IsProjection>(proj1: P1, proj2: P2) -> Result<Self, DimensionError> {
        let r1 = proj1.wrap();
        let r2 = proj2.wrap();
        return Self::new(r1, r2);
    }

    pub fn new(proj1: Projection, proj2: Projection) -> Result<Self, DimensionError> {
        if proj1.output() != proj2.input() {
            return Err(DimensionError{
                msg: format!("Expect the inner dimensions of projection 1 {} and projection 2 {} to match", proj1.dims(), proj2.dims()),
                source: "from() from Concat"
            });
        }

        Ok(Concat {
            proj1,
            proj2
        })
    }

    /// Returns the first projection object in this chained projection, with the same lifetime as this chained projection object
    /// i.e. the intersection of T and S. Check the documentation for Concat::from for usage.
    /// We can also retreive both projections (sorta) as follows
    /// ```
    /// use tikzpaint_rs::figures::{Coordinates, Matrix, Concat, Projection, IsProjection};
    ///
    /// let x = Coordinates::new(vec![3, 4, 5]);
    /// let proj1 = Matrix::array([
    ///     [1, 0, 0],
    ///     [0, 1, 0],
    ///     [0, 0, 2]
    /// ]);
    /// let proj2 = Matrix::array([
    ///     [1, 0, 2],
    ///     [2, -1, 1]
    /// ]);
    /// let proj3 = Concat::from(proj1, proj2).unwrap();
    /// let this_is_also_proj1 = proj3.first();
    /// let y1 = this_is_also_proj1.call(&x).unwrap();
    /// assert!(y1 == Coordinates::new(vec![3, 4, 10]));
    /// ```
    pub fn first(&self) -> Projection {
        self.proj1.clone()
    }

    /// Returns the second projection object in this chained projection, with the same lifetime as this chained projection object
    /// i.e. the intersection of T and S. Check the documentation for Concat::from for usage.
    /// We can also retreive both projections (sorta) as follows
    /// ```
    /// use tikzpaint_rs::figures::{Coordinates, Matrix, Concat, Projection, IsProjection};
    ///
    /// let x = Coordinates::new(vec![3, 4, 5]);
    /// let proj1 = Matrix::array([
    ///     [1, 0, 0],
    ///     [0, 1, 0],
    ///     [0, 0, 2]
    /// ]);
    /// let proj2 = Matrix::array([
    ///     [1, 0, 2],
    ///     [2, -1, 1]
    /// ]);
    /// let proj3 = Concat::from(proj1, proj2).unwrap();
    /// let this_is_also_proj2 = proj3.second();
    /// let y2 = this_is_also_proj2.call(&x).unwrap();
    /// assert!(y2 == Coordinates::new(vec![13, 7]));
    /// ```
    pub fn second(&self) -> Projection {
        self.proj2.clone()
    }
}

impl IsProjection for Concat {
    fn call(&self, v: &Coordinates) -> Coordinates {
        let y = self.proj1.call(v).unwrap();
        let z = self.proj2.call(&y).unwrap();
        return z
    }

    fn input(&self) -> usize {
        self.proj1.input()
    }

    fn output(&self) -> usize {
        self.proj2.output()
    }
}


pub struct Matrix {
    values: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    /// Returns the zero matrix. This only returns an error if either rows or cols is 0
    pub fn zero<T>(rows: usize, cols: usize) -> Result<Self, DimensionError> where
    T: Into<f64> + Clone {
        if rows == 0 {
            return Err(DimensionError{
                msg: format!("Number of rows cannot be 0"),
                source: "zero() from Matrix",
            })
        }

        if cols == 0 {
            return Err(DimensionError{
                msg: format!("Number of cols cannot be 0"),
                source: "zero() from Matrix",
            })
        }

        Ok(Matrix {
            values: vec![vec![0_f64; cols]; rows],
            rows,
            cols
        })
    }

    pub fn array<T: Into<f64> + Clone, const I: usize, const J: usize>(x: [[T; J]; I]) -> Self {
        let y = x.into_iter().map(|r| {
            r.into_iter().map(|t| t.clone().into()).collect::<Vec<f64>>()
        }).collect();
        Matrix {
            values: y,
            rows: I,
            cols: J
        }
    }

    pub fn new<T>(vals: Vec<Vec<T>>) -> Result<Self, DimensionError> where
        T: Into<f64> + Clone
    {
        let rows = vals.len();
        if rows == 0 {
            return Err(DimensionError{
                msg: format!("Number of rows cannot be 0"),
                source: "new() from Matrix",
            })
        }
        let cols = vals[0].len();
        if cols == 0 {
            return Err(DimensionError{
                msg: format!("Number of columns cannot be 0"),
                source: "new() from Matrix",
            })
        }

        let mut x = vec![vec![0_f64; cols]; rows];
        for i in 0..rows {
            if vals[i].len() != cols {
                return Err(DimensionError{
                    msg: format!("Expect row {} to have {} entries, found {}", i, cols, vals[i].len()),
                    source: "new() from Matrix",
                })
            }

            for j in 0..cols {
                x[i][j] = vals[i][j].clone().into();
            }
        }

        Ok(Matrix {
            values: x,
            rows,
            cols
        })
    }
}

impl IsProjection for Matrix {
    fn input(&self) -> usize {
        self.cols
    }

    fn output(&self) -> usize {
        self.rows
    }

    fn call(&self, v: &Coordinates) -> Coordinates {
        let w = (0..self.rows).into_iter().map(|i| {
            (0..self.cols).into_iter().map(|j| v[j] * self.values[i][j]).sum()
        }).collect::<Vec<f64>>();

        Coordinates::new(w)
    }
}
