use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::ops::{Add, Sub, Mul, Div, Index};
use std::f64::EPSILON;
use std::rc::Rc;

use crate::figures::DimensionError;
use crate::figures::Serializable;

pub struct Coordinates {
    values: Rc<Vec<f64>>,
    pub dims: usize
}

impl Clone for Coordinates {
    fn clone(&self) -> Self {
        Coordinates {
            values: Rc::clone(&self.values),
            dims: self.dims
        }
    }
}

impl Coordinates {
    /// Creates a new coordinate point from array
    pub fn new<T>(x: Vec<T>) -> Coordinates where
        T: Into<f64> + Clone
    {
        let n = x.len();
        let res = x.iter().map(|val| {
            (*val).clone().into()
        }).collect::<Vec<f64>>();
        Coordinates {
            values: Rc::new(res),
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
    pub fn scale<T>(&self, other: T) -> Self where
        T: Into<f64> + Clone
    {
        let res: Vec<f64> = self.values.iter().map(|x: &f64| {
            x * other.clone().into()
        }).collect();

        Coordinates {
            values: Rc::new(res),
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

impl Debug for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl PartialEq for Coordinates {
    /// Returns true if the dimensions are the same and every entry is the same (within certain threshold)
    fn eq(&self, other: &Self) -> bool {
        if self.dims != other.dims {
            return false
        }

        for i in 0..self.dims {
            if (self.values[i] - other.values[i]).abs() >= 1e-8 {
                return false;
            }
        }

        true
    }
}

impl Add for Coordinates {
    type Output = Result<Self, DimensionError>;

    fn add(self, other: Self) -> Self::Output {
        if self.dims != other.dims {
            return Err(DimensionError{
                msg: format!("Cannot add coordinate points of unequal dimensions ({} != {})", self.dims, other.dims),
                source: "Add for Coordinates"
            })
        }

        let res = (0..self.dims).into_iter().map(|i| {
            self.values[i] + other.values[i]
        }).collect();

        return Ok(Coordinates {
            values: Rc::new(res),
            dims: self.dims }
        );
    }
}

impl Sub for Coordinates {
    type Output = Result<Self, DimensionError>;

    fn sub(self, other: Self) -> Self::Output {
        if self.dims != other.dims {
            return Err(DimensionError{
                msg: format!("Cannot subtract coordinate points of unequal dimensions ({} != {})", self.dims, other.dims),
                source: "Add for Coordinates"
            })
        }

        let res = (0..self.dims).into_iter().map(|i| {
            self.values[i] - other.values[i]
        }).collect();

        return Ok(Coordinates {
            values: Rc::new(res),
            dims: self.dims
        });
    }
}

impl Mul<f64> for Coordinates {
    type Output = Self;

    /// Represents scalar multiplication
    fn mul(self, other: f64) -> Self {
        let res = (0..self.dims).into_iter().map(|i| {
            self.values[i] * other
        }).collect();

        return Coordinates {
            values: Rc::new(res),
            dims: self.dims
        };
    }
}

pub struct DivisionByZeroError;

impl Div<f64> for Coordinates {
    type Output = Result<Self, DivisionByZeroError>;

    /// Represents scalar division. Returns an error if division by 0.
    fn div(self, other: f64) -> Self::Output {
        if other < EPSILON {
            return Err(DivisionByZeroError);
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
    /// let ab = a.cross(&b).unwrap();
    /// let ba = b.cross(&a).unwrap();
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

        let u = &self.values;
        let v = &other.values;
        return Ok(Coordinates {
            values: Rc::new(vec![
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0]
            ]),
            dims: 3
        });
    }
}

impl Serializable for Coordinates {
    fn into_str(&self) -> String {
        let mut s = format!("cd{},", self.dims);
        for v in &*self.values {
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

        let mut v = vec![0_f64; num_dims];

        for i in (0..num_dims) {
            v.push(f64::from_str(split.next()?)?);
        }

        return Some(Self {
            values: Rc::new(v),
            dims: num_dims
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_norm {
        {$($before:expr, $after:expr, $id:ident),*} => {
            $ (
                #[test]
                fn $id() {
                    let v = Coordinates::new($before);
                    let x = v.normalize();
                    let new_v = Coordinates::new($after);
                    assert_eq!(new_v, x);
                }
            )*
        }
    }

    test_norm! {
        vec![0, 0, 0], vec![0, 0, 0], test_normalize1,
        vec![2147483647, 0, 0], vec![1, 0, 0], test_normalize2,
        vec![-47], vec![-1.0], test_normalize3,
        vec![13, -45, 2, -42, 26, 30, 46, 41], vec![0.13455077114631694, -0.4657526693526356, 0.020700118637894917, -0.43470249139579326, 0.2691015422926339, 0.31050177956842373, 0.4761027286715831, 0.4243524320768458], test_normalize4,
        vec![16, -9, -36, -17, -17, 10], vec![0.33282811850722743, -0.18721581666031542, -0.7488632666412617, -0.35362987591392914, -0.35362987591392914, 0.20801757406701712], test_normalize5,
        vec![31, -33, -26], vec![0.5937433630937711, -0.6320493865191756, -0.4979783045302596], test_normalize6,
        vec![-29], vec![-1.0], test_normalize7,
        vec![-3, 20, -17, -8, -42, -7, 43, -39, 36], vec![-0.03525510244359311, 0.23503401629062073, -0.19977891384702762, -0.0940136065162483, -0.49357143421030353, -0.08226190570171725, 0.5053231350248346, -0.45831633176671044, 0.4230612293231173], test_normalize8,
        vec![9, -2, -48], vec![0.18413418951636573, -0.040918708781414605, -0.9820490107539506], test_normalize9,
        vec![-19, 41, 25, 40, 26, -3, 27], vec![-0.2520816345981441, 0.5439656325538899, 0.3316863613133475, 0.530698178101356, 0.3449538157658814, -0.0398023633576017, 0.35822127021841527], test_normalize10
    }


    macro_rules! test_add_sub {
        {$($expr1:expr, $expr2:expr, $added:expr, $subbed:expr, $id:ident),*} => {
            $ (
                #[test]
                fn $id() {
                    let v = (Coordinates::new($expr1) + Coordinates::new($expr2)).unwrap();
                    let x = Coordinates::new($added);
                    assert_eq!(v, x);

                    let v = (Coordinates::new($expr1) - Coordinates::new($expr2)).unwrap();
                    let x = Coordinates::new($subbed);
                    assert_eq!(v, x);
                }
            )*
        }
    }

    test_add_sub! {
        vec![40], vec![-20], vec![20], vec![60], test_add_sub1,
        vec![10], vec![-42], vec![-32], vec![52], test_add_sub2,
        vec![35, 9], vec![7, 16], vec![42, 25], vec![28, -7], test_add_sub3,
        vec![-26, 19], vec![41, 38], vec![15, 57], vec![-67, -19], test_add_sub4,
        vec![16, 26], vec![-2, 26], vec![14, 52], vec![18, 0], test_add_sub5,
        vec![-48, -35, 9], vec![-30, -43, 21], vec![-78, -78, 30], vec![-18, 8, -12], test_add_sub6,
        vec![-42, -37, -50], vec![-47, -18, 7], vec![-89, -55, -43], vec![5, -19, -57], test_add_sub7,
        vec![-3, -22, -45, -40], vec![42, 41, -46, 0], vec![39, 19, -91, -40], vec![-45, -63, 1, -40], test_add_sub8,
        vec![16, -8, -8, 26, 47, 44], vec![-10, -18, -1, -40, 32, 17], vec![6, -26, -9, -14, 79, 61], vec![26, 10, -7, 66, 15, 27], test_add_sub9,
        vec![-28, 11, 26, 48, 38, -27, 46, 29, -24], vec![-26, 19, -25, 26, 29, 35, 19, -26, 35], vec![-54, 30, 1, 74, 67, 8, 65, 3, 11], vec![-2, -8, 51, 22, 9, -62, 27, 55, -59], test_add_sub10
    }

    macro_rules! test_add_sub_fail {
        {$($expr1:expr, $expr2:expr, $id_add:ident, $id_sub:ident),*} => {
            $ (
                #[test]
                #[should_panic]
                fn $id_add() {
                    let v = (Coordinates::new($expr1) + Coordinates::new($expr2)).unwrap();
                }

                #[test]
                #[should_panic]
                fn $id_sub() {
                    let v = (Coordinates::new($expr1) - Coordinates::new($expr2)).unwrap();
                }
            )*
        }
    }

    test_add_sub_fail! {
        vec![-36], vec![-26, -45], test_add_fail0, test_sub_fail0,
        vec![29, -44], vec![36], test_add_fail1, test_sub_fail1,
        vec![25, 2, 2, 48, -32], vec![46, 19, -47, -24, 43, -37], test_add_fail2, test_sub_fail2,
        vec![-12, 43, -14, 29, 33, 4, -47], vec![15, -41, -38], test_add_fail3, test_sub_fail3,
        vec![30, -26, -23, -10, -37, -16], vec![30, -36, 7, -35, 4, 46, 25, 7, 3], test_add_fail4, test_sub_fail4,
        vec![-39, -39], vec![-40, -49, -39], test_add_fail5, test_sub_fail5,
        vec![26, 49, 23], vec![-7, 40], test_add_fail6, test_sub_fail6
    }

    macro_rules! test_mul_scale {
        {$($expr1:expr, $fac:expr, $res:expr, $id_mul:ident, $id_scale:ident),*} => {
            $ (
                #[test]
                fn $id_scale() {
                    let v = Coordinates::new($expr1).scale($fac);
                    let res = Coordinates::new($res);
                    assert_eq!(v, res);
                }

                #[test]
                fn $id_mul() {
                    let v = Coordinates::new($expr1) * $fac;
                    let res = Coordinates::new($res);
                    assert_eq!(v, res);
                }
            )*
        }
    }

    test_mul_scale! {
        vec![7], -2.74, vec![-19.18], test_scale1, test_mul1,
        vec![40], -1.67, vec![-66.8], test_scale2, test_mul2,
        vec![41, -21], 3.58, vec![146.78, -75.18], test_scale3, test_mul3,
        vec![49, 5], -0.65, vec![-31.85, -3.25], test_scale4, test_mul4,
        vec![-46, 2], -2.92, vec![134.32, -5.84], test_scale5, test_mul5,
        vec![47, -43, -42], -2.26, vec![-106.22, 97.18, 94.92], test_scale6, test_mul6,
        vec![5, 19, -15], 4.09, vec![20.45, 77.71, -61.35], test_scale7, test_mul7,
        vec![-15, 6, 28, -7], -1.01, vec![15.15, -6.06, -28.28, 7.07], test_scale8, test_mul8,
        vec![48, 32, -4, 30, -46, -34], -3.51, vec![-168.48, -112.32, 14.04, -105.3, 161.46, 119.34], test_scale9, test_mul9,
        vec![24, -9, -44, 33, -9, -9, 47, 36, -43], -4.67, vec![-112.08, 42.03, 205.48, -154.11, 42.03, 42.03, -219.49, -168.12, 200.81], test_scale10, test_mul10
    }
}