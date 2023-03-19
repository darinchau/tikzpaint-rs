use std::fmt::{Display, Debug};
use std::ops::{Add, Sub, Mul, Div, Index};
use std::rc::Rc;

use crate::core::calc::*;

#[derive(Clone, Copy)]
pub struct Coordinates {
    values: (f64, f64),
}

impl Coordinates {
    /// Creates a new coordinate point from array
    pub fn new<T, U>(x: T, y: U) -> Coordinates where
        T: Into<f64>,
        U: Into<f64>
    {
        Coordinates {
            values: (x.into(), y.into()),
        }
    }

    /// Gets the i-th element. Returns None if the index is out of range
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::figures::Coordinates;
    /// let coord = Coordinates::new(1, 2);
    /// assert_eq!(1., coord.get(0).unwrap());
    /// assert_eq!(2., coord.get(1).unwrap());
    /// assert_eq!(None, coord.get(3));
    /// ```
    pub fn get(&self, index: usize) -> Option<f64> {
        if index == 0 {
            return Some(self.values.0);
        }

        if index == 1 {
            return Some(self.values.1);
        }

        return None;
    }

    /// Scales a coordinate point. This does not consume ownership, unlike operator*
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::figures::Coordinates;
    /// let coord = Coordinates::new(1, -2).scale(6);
    /// let coord2 = Coordinates::new(6, -12);
    /// assert!(coord == coord2);
    /// ```
    pub fn scale<T>(&self, other: T) -> Self where
        T: Into<f64> + Clone
    {
        let x = other.into();
        Coordinates {
            values: (self.values.0 * x, self.values.1 *x),
        }
    }

    /// Returns the magnitude of the coordinates (i.e. distance from origin) under the standard Euclidean L2 norm
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::figures::Coordinates;
    /// let coord = Coordinates::new(vec![1, 13]);
    /// let mag = (14 as f64).sqrt();
    /// assert!(coord.magnitude() - mag <= 1e-8);
    /// ```
    pub fn magnitude(&self) -> f64
    {
        (self.values.0 * self.values.0 + self.values.1 * self.values.1).sqrt()
    }


    /// Normalize the coordinates for self. Returns the zero vector if the input is a zero vector
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
        if is_zero(mag) {
            return Coordinates{
                values: (0., 0.)
            };
        }
        self * (1./mag)
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.values.0, self.values.1)
    }
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.values.0, self.values.1)
    }
}

impl PartialEq for Coordinates {
    /// Returns true if the dimensions are the same and every entry is the same (within certain threshold)
    fn eq(&self, other: &Self) -> bool {
        eq(&self.values.0, &other.values.0) && eq(&self.values.1, &other.values.1)
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            values: (self.values.0 + other.values.0, self.values.1 + other.values.1)
        }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            values: (self.values.0 - other.values.0, self.values.1 - other.values.1)
        }
    }
}

impl Mul<f64> for Coordinates {
    type Output = Self;

    /// Represents scalar multiplication
    fn mul(self, other: f64) -> Self {
        Self {
            values: (self.values.0 * other, self.values.1 * other)
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct DivisionByZeroError;

impl Div<f64> for Coordinates {
    type Output = Result<Self, DivisionByZeroError>;

    /// Represents scalar division. Returns an error if division by 0.
    fn div(self, other: f64) -> Self::Output {
        if is_zero(other) {
            return Err(DivisionByZeroError);
        }

        Ok(self * (1./other))
    }
}

impl Index<usize> for Coordinates {
    type Output = f64;
    /// operator[] panics if the index is out of range.
    fn index(&self, index: usize) -> &f64 {
        if index == 0 {
            return &self.values.0;
        }
        if index == 1 {
            return &self.values.1;
        }
        panic!("Invalid index into coordinates")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let p1 = Coordinates::new(1, 2);
        let p2 = Coordinates::new(3, 4);
        assert_eq!(p1 + p2, Coordinates::new(4, 6));
    }

    #[test]
    fn test_sub() {
        let p1 = Coordinates::new(1, 2);
        let p2 = Coordinates::new(3, 4);
        assert_eq!(p1 - p2, Coordinates::new(-2, -2));
    }

    #[test]
    fn test_mul() {
        let p1 = Coordinates::new(1, 2);
        assert_eq!(p1 * 2.0, Coordinates::new(2, 4));
    }

    #[test]
    fn test_div() {
        let p1 = Coordinates::new(1, 2);
        assert_eq!(p1 / 2.0, Ok(Coordinates::new(0.5, 1.0)));
        assert!((p1 / 0.0).is_err());
    }

    #[test]
    fn test_normalize() {
        let p1 = Coordinates::new(3, 4);
        assert_eq!(p1.normalize(), Coordinates::new(0.6, 0.8));

        let p2 = Coordinates::new(0, 0);
        assert_eq!(p2.normalize(), Coordinates::new(0, 0));
    }
}
