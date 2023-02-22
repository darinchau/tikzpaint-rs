//! Projections are traits that takes coordinates and outputs coordinates
use std::rc::Rc;

use crate::figures::{Coordinates, DimensionError};

/// Basic implementation for a projection function trait object that takes Coordinates of INPUT dimensions and turn
/// them into coordinates of OUTPUT dimensions
pub trait Projection where
Self: 'static {
    fn input(&self) -> usize;
    fn output(&self) -> usize;
    /// Performs the projection.
    fn call(&self, v: &Coordinates) -> Result<Coordinates, DimensionError>;
}

struct BoundProjectionDispatcher {
    ptr: Rc<dyn Projection>
}

impl Projection for BoundProjectionDispatcher {
    fn input(&self) -> usize {
        self.ptr.input()
    }

    fn output(&self) -> usize {
        self.ptr.output()
    }

    fn call(&self, v: &Coordinates) -> Result<Coordinates, DimensionError>{
        if v.dims != self.input() {
            return Err(DimensionError{
                msg: format!("Found incorrect input dimensions. Expect {}, found {}", self.input(), v.dims),
                source: "call() from BoundProjectionDispatcher (did you extract a projection from a concatenated projection?)"
            });
        }

        Ok(self.ptr.call(v))
    }
}

/// A struct signifying the identity projection. The call() method will never return an error.
///
/// Example
/// ```
/// use tikzpaint_rs::figures::{Identity, Coordinates, Projection};
/// let x = Coordinates::new(vec![3, 4, 5]);
/// let y = Coordinates::new(vec![3, 4, 5]);
/// let proj = Identity::<3>;
/// assert!(x[0] == proj.call(&y)[0]);
/// assert!(x[1] == proj.call(&y)[1]);
/// assert!(x[2] == proj.call(&y)[2]);
/// ```
pub struct Identity;
impl Projection for Identity {
    fn call(&self, v: &Coordinates) -> Result<Coordinates, DimensionError>{
        return Ok(v.clone());
    }
}

///The concat type allows us to construct new projections from existing projections.
pub struct Concat {
    proj1: Rc<dyn Projection>,
    proj2: Rc<dyn Projection>
}

impl Concat {
    /// This allows us to construct new projections from existing projections.
    /// Concat::from(p1, p2) is a projection object that essentially does
    /// proj2(proj1(v)) when called
    ///
    /// Examples:
    /// ```
    /// use tikzpaint_rs::figures::{Coordinates, Matrix, Concat, Projection};
    /// let x = Coordinates::new([3, 4, 5]);
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
    /// assert!(y3 == Coordinates::new([23, 12]));
    /// ```
    ///
    /// We can also retreive both projections (sorta) as follows
    /// ```
    /// use tikzpaint_rs::figures::{Coordinates, Matrix, Concat, Projection};
    /// let x = Coordinates::new([3, 4, 5]);
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
    /// assert!(y1 == Coordinates::new([3, 4, 10]));
    ///
    /// let this_is_also_proj2 = proj3.second();
    /// let y2 = this_is_also_proj2.call(&x);
    /// assert!(y2 == Coordinates::new([13, 7]));
    /// ```
    pub fn from(proj1: &Rc<dyn Projection>, proj2: &Rc<dyn Projection>) -> Result<Self, DimensionError> {
        if proj1.output() != proj2.output() {
            return Err(DimensionError{
                msg: format!("Expect the inner dimensions of projection 1 ({} -> {}) and projection 2 ({} -> {}) to match",
                    proj1.input(), proj1.output(), proj2.input(), proj2.output()),
                source: "from() from Concat"
            });
        }

        let r1 = Rc::clone(&proj1);
        let r2 = Rc::clone(&proj2);

        Some(Concat {
            proj1: r1,
            proj2: r2
        })
    }

    /// Returns the first projection object in this chained projection, with the same lifetime as this chained projection object
    /// i.e. the intersection of T and S. Check the documentation for Concat::from for usage.
    pub fn first(&self) -> impl Projection {
        BoundProjectionDispatcher {
            ptr: Rc::clone(&self.proj1)
        }
    }

    /// Returns the second projection object in this chained projection, with the same lifetime as this chained projection object
    /// i.e. the intersection of T and S. Check the documentation for Concat::from for usage.
    pub fn second(&self) -> impl Projection {
        BoundProjectionDispatcher {
            ptr: Rc::clone(&self.proj2)
        }
    }
}

impl Projection for Concat {
    fn call(&self, v: &Coordinates) -> Result<Coordinates, DimensionError> {
        let y = self.proj1.call(v)?;
        let z = self.proj2.call(&y)?;
        return Ok(z)
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
            values: vec![vec![0; cols]; rows],
            rows,
            cols
        })
    }

    pub fn new<T>(vals: Vec<Vec<T>>) -> Result<Self, DimensionError> where
    T: Into<f64> + Clone {
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
                x[i][j] = vals[i][j].into();
            }
        }

        Some(Matrix {
            values: x,
            rows,
            cols
        })
    }
}

impl Projection for Matrix {
    fn input(&self) -> usize {
        self.cols
    }

    fn output(&self) -> usize {
        self.rows
    }

    fn call(&self, v: &Coordinates) -> Result<Coordinates, DimensionError> {
        if v.dims != self.cols {
            return Err(DimensionError{
                msg: format!("Expect the dimension of v to equal the number of columns ({}), found {}", self.cols, v.dims),
                source: "call() from Matrix",
            })
        }
        let w = (0..self.rows).iter().map(|i| {
            (0..self.cols).into_iter().map(|j| v[j] * self.values[i][j]).sum()
        }).collect::<Vec<f64>>();

        Ok(Coordinates::new(&w))
    }
}
