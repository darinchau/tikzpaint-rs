// use std::fmt::{Display, Error};
use std::ops::{Add, Sub, Mul, Div, Index};
use std::f64::EPSILON;

pub struct Coordinates<const DIMS: usize> {
    values: [f64; DIMS],
}

impl<const DIMS: usize> Coordinates<DIMS> {
    /// Creates a new coordinate point from array
    pub fn new<T>(x: &[T; DIMS]) -> Coordinates<DIMS> where
        T: Into<f64> + Copy 
    {
        let mut res: [f64; DIMS] = [0.; DIMS];
        for i in 0..DIMS {
            res[i] = x[i].into();
        }
        Coordinates {
            values: res,
        }
    }

    /// Gets the i-th element. Returns None if the index is out of range
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::Coordinates;
    /// let coord = Coordinates::new(&[1, 2, -3]);
    /// assert_eq!(1., coord.get(0).unwrap());
    /// assert_eq!(2., coord.get(1).unwrap());
    /// assert_eq!(-3., coord.get(2).unwrap());
    /// assert_eq!(None, coord.get(3));
    /// ```
    pub fn get(&self, index: usize) -> Option<f64> {
        if index >= DIMS {
            return None
        }

        Some(self.values[index])
    }

    /// Scales a coordinate point. This does not consume ownership, unlike operator*
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::Coordinates;
    /// let coord = Coordinates::new(&[1, 2, -3]);
    /// let coord2 = coord.scale(6);
    /// assert_eq!(6., coord2[0]);
    /// assert_eq!(12., coord2[1]);
    /// assert_eq!(-18., coord2[2]);
    /// ```
    pub fn scale<T>(&self, other: T) -> Self where
        T: Into<f64> + Copy  
    {
        let mut res = [0.; DIMS];
        for i in 0..DIMS {
            res[i] = self.values[i] * other.into();
        }
        Coordinates {
            values: res
        }
    }

    /// Returns the magnitude of the coordinates (i.e. distance from origin) under the standard Euclidean L2 norm
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::Coordinates;
    /// let coord = Coordinates::new(&[1, 2, -3]);
    /// let mag = (14 as f64).sqrt();
    /// assert!(coord.magnitude() - mag <= 1e-8);
    /// ```
    pub fn magnitude(&self) -> f64
    {
        self.values
            .into_iter()
            .map(|x| { x * x })
            .sum::<f64>()
            .sqrt()
    }


    /// Normalize the coordinates for self
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::Coordinates;
    /// let coord = Coordinates::new(&[3, 4]).normalize();
    /// assert!(coord[0] - 0.6 <= 1e-8);
    /// assert!(coord[1] - 0.8 <= 1e-8);
    /// ```
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        if mag <= 1e-8 {
            return self;
        }
        self * (1./mag)
    }
}

impl<const DIMS: usize> Add for Coordinates<DIMS> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        for i in 0..DIMS {
            self.values[i] += other.values[i];
        }
        self
    }
}

impl<const DIMS: usize> Sub for Coordinates<DIMS> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        for i in 0..DIMS {
            self.values[i] -= other.values[i];
        }
        self
    }
}

impl<const DIMS: usize> Mul<f64> for Coordinates<DIMS> {
    type Output = Self;

    fn mul(mut self, other: f64) -> Self {
        for i in 0..DIMS {
            self.values[i] *= other;
        }
        self
    }
}

impl<const DIMS: usize> Div<f64> for Coordinates<DIMS> {
    type Output = Result<Self, &'static str>;

    fn div(self, other: f64) -> Result<Self, &'static str> {
        if other < EPSILON {
            return Err("Division by 0");
        }

        Ok(self * (1./other))
    }
}

impl<const DIMS: usize> Index<usize> for Coordinates<DIMS> {
    type Output = f64;
    fn index(&self, other: usize) -> &f64 {
        if other >= DIMS {
            panic!("Index out of range");
        }

        &self.values[other]
    }
}