use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Div, Index};
use std::f64::EPSILON;

pub struct Coordinates<const DIMS: usize> {
    values: [f64; DIMS],
}

impl<const DIMS: usize> Coordinates<DIMS> {
    /// Creates a new coordinate point from array
    pub fn new<T>(x: [T; DIMS]) -> Coordinates<DIMS> where
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
    /// use tikzpaint_rs::figures::Coordinates;
    /// let coord = Coordinates::new([1, 2, -3]);
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
    /// use tikzpaint_rs::figures::Coordinates;
    /// let coord = Coordinates::new([1, 2, -3]).scale(6);
    /// let coord2 = Coordinates::new([6, 12, -18]);
    /// assert!(coord == coord2);
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
    /// use tikzpaint_rs::figures::Coordinates;
    /// let coord = Coordinates::new([1, 2, -3]);
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
    /// use tikzpaint_rs::figures::Coordinates;
    /// let coord = Coordinates::new([3, 4]).normalize();
    /// let coord2 = Coordinates::new([0.6, 0.8]);
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

impl<const DIMS: usize> Display for Coordinates<DIMS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..DIMS {
            let f = if i < DIMS - 1 {format!("{}, ", self[i])} else {format!("{}", self[i])};
            s.push_str(&f);
        }
        write!(f, "({})", s)
    }
}

impl<const DIMS: usize> PartialEq for Coordinates<DIMS> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..DIMS {
            if (self.values[i] - other.values[i]).abs() >= 1e-8 {
                return false;
            }
        }

        true
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

impl<const DIMS: usize> Clone for Coordinates<DIMS> {
    fn clone(&self) -> Self {
        self.scale(1)
    }
}

impl Coordinates<3> {
    /// The cross product. "a.cross(b)" indicates a x b
    ///
    /// # Examples
    ///
    /// ```
    /// use tikzpaint_rs::figures::Coordinates;
    /// let a = Coordinates::new([3, 2, -4]);
    /// let b = Coordinates::new([1, 2, 0]);
    /// let ab = a.cross(&b);
    /// let ba = b.cross(&a);
    /// assert!(ab == Coordinates::new([8, -4, 4]));
    /// assert!(ba == Coordinates::new([-8, 4, -4]));
    /// ```
    pub fn cross(&self, other: &Coordinates<3>) -> Self {
        let u = self.values;
        let v = other.values;
        return Coordinates {
            values: [
                u[1] * v[2] - u[2] * v[1], 
                u[2] * v[0] - u[0] * v[2], 
                u[0] * v[1] - u[1] * v[0]
            ]
        }
    }
}