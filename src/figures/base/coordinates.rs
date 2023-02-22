use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, Sub, Mul, Div, Index};
use std::f64::EPSILON;

use crate::app::Serializable;
use crate::figures::{Hashable, DimensionError};

#[derive(Clone)]
pub struct Coordinates {
    values: Vec<f64>,
    pub dims: usize
}

impl Coordinates {
    /// Creates a new coordinate point from array
    pub fn new<T>(&x: Vec<T>) -> Coordinates where
        T: Into<f64> + Clone
    {
        let n = x.len();
        let res = x.iter().map(|val| {
            val.into();
        }).collect::<Vec<f64>>();
        Coordinates {
            values: res,
            dims: x.len()
        }
    }

    /// Gets the i-th element. Returns None if the index is out of range
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::figures::Coordinates;
    /// let coord = Coordinates::new(vec![1, 2, -3]);
    /// assert_eq!(1., coord.get(0).unwrap());
    /// assert_eq!(2., coord.get(1).unwrap());
    /// assert_eq!(-3., coord.get(2).unwrap());
    /// assert_eq!(None, coord.get(3));
    /// ```
    pub fn get(&self, index: usize) -> Option<f64> {
        if index >= self.dims {
            return None
        }

        Some(self.values[index])
    }

    /// Scales a coordinate point. This does not consume ownership, unlike operator*
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::figures::Coordinates;
    /// let coord = Coordinates::new(vec![1, 2, -3]).scale(6);
    /// let coord2 = Coordinates::new(vec![6, 12, -18]);
    /// assert!(coord == coord2);
    /// ```
    pub fn scale<T>(&self, &other: T) -> Self where
        T: Into<f64> + Clone
    {
        let res = self.values.iter().map(|x: f64| {
            x * other
        }).collect();

        Coordinates {
            values: res,
            dims: self.dims
        }
    }

    /// Returns the magnitude of the coordinates (i.e. distance from origin) under the standard Euclidean L2 norm
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::figures::Coordinates;
    /// let coord = Coordinates::new(vec![1, 2, -3]);
    /// let mag = (14 as f64).sqrt();
    /// assert!(coord.magnitude() - mag <= 1e-8);
    /// ```
    pub fn magnitude(&self) -> f64
    {
        self.values
            .iter()
            .map(|x| { x * x })
            .sum::<f64>()
            .sqrt()
    }


    /// Normalize the coordinates for self
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::figures::Coordinates;
    /// let coord = Coordinates::new(vec![3, 4]).normalize();
    /// let coord2 = Coordinates::new(vec![0.6, 0.8]);
    /// assert!(coord == coord2);
    /// ```
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        if mag <= 1e-8 {
            return self;
        }
        self * (1./mag)
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.dims {
            let f = if i < self.dims - 1 {format!("{}, ", self[i])} else {format!("{}", self[i])};
            s.push_str(&f);
        }
        write!(f, "({})", s)
    }
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> Result<bool, DimensionError> {
        if self.dims != other.dims {
            return Err(DimensionError{
                msg: format!("Cannot compare coordinate points of unequal dimensions ({} != {})", self.dims, other.dims),
                source: "PartialEq for Coordinates"
            })
        }

        for i in 0..self.dims {
            if (self.values[i] - other.values[i]).abs() >= 1e-8 {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

impl Add for Coordinates {
    type Output = Result<Self, DimensionError>;

    fn add(self, other: &Self) -> Self::Output {
        if self.dims != other.dims {
            return Err(DimensionError{
                msg: format!("Cannot add coordinate points of unequal dimensions ({} != {})", self.dims, other.dims),
                source: "Add for Coordinates"
            })
        }

        let res = (0..self.dims).iter().map(|i| {
            self.values[i] + other.values[i]
        }).collect();

        return Some(Coordinates { values: res, dims: self.dims });
    }
}

impl Sub for Coordinates {
    type Output = Result<Self, DimensionError>;

    fn sub(self, other: &Self) -> Self::Output {
        if self.dims != other.dims {
            return Err(DimensionError{
                msg: format!("Cannot subtract coordinate points of unequal dimensions ({} != {})", self.dims, other.dims),
                source: "Add for Coordinates"
            })
        }

        let res = (0..self.dims).iter().map(|i| {
            self.values[i] - other.values[i]
        }).collect();

        return Some(Coordinates { values: res, dims: self.dims });
    }
}

impl Mul<f64> for Coordinates {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        let res = (0..self.dims).iter().map(|i| {
            self.values[i] * other
        }).collect();

        return Some(Coordinates { values: res, dims: self.dims });
    }
}

impl Div<f64> for Coordinates {
    type Output = Result<Self, &'static str>;

    fn div(self, other: f64) -> Result<Self, &'static str> {
        if other < EPSILON {
            return Err("Division by 0");
        }

        Ok(self * (1./other))
    }
}

impl Index<usize> for Coordinates {
    type Output = f64;
    /// operator[] panics if the index is out of range.
    fn index(&self, other: usize) -> &f64 {
        if other >= self.dims {
            panic!("Index out of range");
        }

        &self.values[other]
    }
}

impl Coordinates {
    /// The cross product. "a.cross(b)" indicates a x b
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::figures::Coordinates;
    /// let a = Coordinates::new(vec![3, 2, -4]);
    /// let b = Coordinates::new(vec![1, 2, 0]);
    /// let ab = a.cross(&b);
    /// let ba = b.cross(&a);
    /// assert!(ab == Coordinates::new(vec![8, -4, 4]));
    /// assert!(ba == Coordinates::new(vec![-8, 4, -4]));
    /// ```
    pub fn cross(&self, other: &Coordinates) -> Result<Self, DimensionError> {
        if self.dims != 3 {
            return Err(DimensionError{
                msg: format!("Self dimension is not 3 (found {})", self.dims),
                source: "cross() for Coordinates"
            });
        }

        if other.dims != 3 {
            return Err(DimensionError{
                msg: format!("RHS dimension is not 3 (found {})", other.dims),
                source: "cross() for Coordinates"
            });
        }

        let u = self.values;
        let v = other.values;
        return Ok(Coordinates {
            values: vec![
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0]
            ],
            dims: 3
        });
    }
}

impl Serializable for Coordinates {
    fn into_str(&self) -> String {
        let mut s = format!("cd{},", self.dims);
        for v in self.values {
            s.push_str(&v.into_str());
            s.push_str(",");
        }

        s
    }

    fn from_str(s: &str) -> Option<Self> {
        if !s.starts_with("cd") {
            return None;
        }

        let mut split = (&s[2..]).split(",");

        let num_dims = split
            .next()?
            .parse::<usize>()
            .ok()?;

        let mut v = vec![0; num_dims];

        for i in (0..num_dims) {
            v.push(f64::from_str(split.next()?)?);
        }

        return Some(Self {
            values: v,
            dims: num_dims
        })
    }
}

impl Hashable for Coordinates {
    fn hash(&self) -> i64 {
        return self.values.iter().enumerate().map(|(i, x)| {
            i as i64 | x.hash()
        }).sum();
    }
}