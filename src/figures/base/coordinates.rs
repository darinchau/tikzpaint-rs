use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::ops::{Add, Sub, Mul, Div, Index};
use std::f64::EPSILON;
use std::rc::Rc;

use crate::figures::DimensionError;

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
        for (i, v) in self.values.iter().enumerate() {
            s.push_str(&v.to_string());
            if i != self.dims - 1 {
                s.push_str(", ");
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize1() {
        let v = Coordinates::new(vec![0, 0, 0]);
        let x = v.normalize();
        let new_v = Coordinates::new(vec![0, 0, 0]);
        assert_eq!(new_v, x);
    }


    #[test]
    fn test_normalize2() {
        let v = Coordinates::new(vec![2147483647, 0, 0]);
        let x = v.normalize();
        let new_v = Coordinates::new(vec![1, 0, 0]);
        assert_eq!(new_v, x);
    }


    #[test]
    fn test_normalize3() {
        let v = Coordinates::new(vec![-47]);
        let x = v.normalize();
        let new_v = Coordinates::new(vec![-1.0]);
        assert_eq!(new_v, x);
    }


    #[test]
    fn test_normalize4() {
        let v = Coordinates::new(vec![13, -45, 2, -42, 26, 30, 46, 41]);
        let x = v.normalize();
        let new_v = Coordinates::new(vec![0.13455077114631694, -0.4657526693526356, 0.020700118637894917, -0.43470249139579326, 0.2691015422926339, 0.31050177956842373, 0.4761027286715831, 0.4243524320768458]);
        assert_eq!(new_v, x);
    }


    #[test]
    fn test_normalize5() {
        let v = Coordinates::new(vec![16, -9, -36, -17, -17, 10]);
        let x = v.normalize();
        let new_v = Coordinates::new(vec![0.33282811850722743, -0.18721581666031542, -0.7488632666412617, -0.35362987591392914, -0.35362987591392914, 0.20801757406701712]);
        assert_eq!(new_v, x);
    }


    #[test]
    fn test_normalize6() {
        let v = Coordinates::new(vec![31, -33, -26]);
        let x = v.normalize();
        let new_v = Coordinates::new(vec![0.5937433630937711, -0.6320493865191756, -0.4979783045302596]);
        assert_eq!(new_v, x);
    }


    #[test]
    fn test_normalize7() {
        let v = Coordinates::new(vec![-29]);
        let x = v.normalize();
        let new_v = Coordinates::new(vec![-1.0]);
        assert_eq!(new_v, x);
    }


    #[test]
    fn test_normalize8() {
        let v = Coordinates::new(vec![-3, 20, -17, -8, -42, -7, 43, -39, 36]);
        let x = v.normalize();
        let new_v = Coordinates::new(vec![-0.03525510244359311, 0.23503401629062073, -0.19977891384702762, -0.0940136065162483, -0.49357143421030353, -0.08226190570171725, 0.5053231350248346, -0.45831633176671044, 0.4230612293231173]);
        assert_eq!(new_v, x);
    }


    #[test]
    fn test_normalize9() {
        let v = Coordinates::new(vec![9, -2, -48]);
        let x = v.normalize();
        let new_v = Coordinates::new(vec![0.18413418951636573, -0.040918708781414605, -0.9820490107539506]);
        assert_eq!(new_v, x);
    }


    #[test]
    fn test_normalize10() {
        let v = Coordinates::new(vec![-19, 41, 25, 40, 26, -3, 27]);
        let x = v.normalize();
        let new_v = Coordinates::new(vec![-0.2520816345981441, 0.5439656325538899, 0.3316863613133475, 0.530698178101356, 0.3449538157658814, -0.0398023633576017, 0.35822127021841527]);
        assert_eq!(new_v, x);
    }

    #[test]
    fn test_add_sub1() {
        let v = (Coordinates::new(vec![40]) + Coordinates::new(vec![-20])).unwrap();
        let x = Coordinates::new(vec![20]);
        assert_eq!(v, x);

        let v = (Coordinates::new(vec![40]) - Coordinates::new(vec![-20])).unwrap();
        let x = Coordinates::new(vec![60]);
        assert_eq!(v, x);
    }


    #[test]
    fn test_add_sub2() {
        let v = (Coordinates::new(vec![10]) + Coordinates::new(vec![-42])).unwrap();
        let x = Coordinates::new(vec![-32]);
        assert_eq!(v, x);

        let v = (Coordinates::new(vec![10]) - Coordinates::new(vec![-42])).unwrap();
        let x = Coordinates::new(vec![52]);
        assert_eq!(v, x);
    }


    #[test]
    fn test_add_sub3() {
        let v = (Coordinates::new(vec![35, 9]) + Coordinates::new(vec![7, 16])).unwrap();
        let x = Coordinates::new(vec![42, 25]);
        assert_eq!(v, x);

        let v = (Coordinates::new(vec![35, 9]) - Coordinates::new(vec![7, 16])).unwrap();
        let x = Coordinates::new(vec![28, -7]);
        assert_eq!(v, x);
    }


    #[test]
    fn test_add_sub4() {
        let v = (Coordinates::new(vec![-26, 19]) + Coordinates::new(vec![41, 38])).unwrap();
        let x = Coordinates::new(vec![15, 57]);
        assert_eq!(v, x);

        let v = (Coordinates::new(vec![-26, 19]) - Coordinates::new(vec![41, 38])).unwrap();
        let x = Coordinates::new(vec![-67, -19]);
        assert_eq!(v, x);
    }


    #[test]
    fn test_add_sub5() {
        let v = (Coordinates::new(vec![16, 26]) + Coordinates::new(vec![-2, 26])).unwrap();
        let x = Coordinates::new(vec![14, 52]);
        assert_eq!(v, x);

        let v = (Coordinates::new(vec![16, 26]) - Coordinates::new(vec![-2, 26])).unwrap();
        let x = Coordinates::new(vec![18, 0]);
        assert_eq!(v, x);
    }


    #[test]
    fn test_add_sub6() {
        let v = (Coordinates::new(vec![-48, -35, 9]) + Coordinates::new(vec![-30, -43, 21])).unwrap();
        let x = Coordinates::new(vec![-78, -78, 30]);
        assert_eq!(v, x);

        let v = (Coordinates::new(vec![-48, -35, 9]) - Coordinates::new(vec![-30, -43, 21])).unwrap();
        let x = Coordinates::new(vec![-18, 8, -12]);
        assert_eq!(v, x);
    }


    #[test]
    fn test_add_sub7() {
        let v = (Coordinates::new(vec![-42, -37, -50]) + Coordinates::new(vec![-47, -18, 7])).unwrap();
        let x = Coordinates::new(vec![-89, -55, -43]);
        assert_eq!(v, x);

        let v = (Coordinates::new(vec![-42, -37, -50]) - Coordinates::new(vec![-47, -18, 7])).unwrap();
        let x = Coordinates::new(vec![5, -19, -57]);
        assert_eq!(v, x);
    }


    #[test]
    fn test_add_sub8() {
        let v = (Coordinates::new(vec![-3, -22, -45, -40]) + Coordinates::new(vec![42, 41, -46, 0])).unwrap();
        let x = Coordinates::new(vec![39, 19, -91, -40]);
        assert_eq!(v, x);

        let v = (Coordinates::new(vec![-3, -22, -45, -40]) - Coordinates::new(vec![42, 41, -46, 0])).unwrap();
        let x = Coordinates::new(vec![-45, -63, 1, -40]);
        assert_eq!(v, x);
    }


    #[test]
    fn test_add_sub9() {
        let v = (Coordinates::new(vec![16, -8, -8, 26, 47, 44]) + Coordinates::new(vec![-10, -18, -1, -40, 32, 17])).unwrap();
        let x = Coordinates::new(vec![6, -26, -9, -14, 79, 61]);
        assert_eq!(v, x);

        let v = (Coordinates::new(vec![16, -8, -8, 26, 47, 44]) - Coordinates::new(vec![-10, -18, -1, -40, 32, 17])).unwrap();
        let x = Coordinates::new(vec![26, 10, -7, 66, 15, 27]);
        assert_eq!(v, x);
    }


    #[test]
    fn test_add_sub10() {
        let v = (Coordinates::new(vec![-28, 11, 26, 48, 38, -27, 46, 29, -24]) + Coordinates::new(vec![-26, 19, -25, 26, 29, 35, 19, -26, 35])).unwrap();
        let x = Coordinates::new(vec![-54, 30, 1, 74, 67, 8, 65, 3, 11]);
        assert_eq!(v, x);

        let v = (Coordinates::new(vec![-28, 11, 26, 48, 38, -27, 46, 29, -24]) - Coordinates::new(vec![-26, 19, -25, 26, 29, 35, 19, -26, 35])).unwrap();
        let x = Coordinates::new(vec![-2, -8, 51, 22, 9, -62, 27, 55, -59]);
        assert_eq!(v, x);
    }

    #[test]
    #[should_panic]
    fn test_add_fail0() {
        let v = (Coordinates::new(vec![-36]) + Coordinates::new(vec![-26, -45])).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_sub_fail0() {
        let v = (Coordinates::new(vec![-36]) - Coordinates::new(vec![-26, -45])).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_add_fail1() {
        let v = (Coordinates::new(vec![29, -44]) + Coordinates::new(vec![36])).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_sub_fail1() {
        let v = (Coordinates::new(vec![29, -44]) - Coordinates::new(vec![36])).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_add_fail2() {
        let v = (Coordinates::new(vec![25, 2, 2, 48, -32]) + Coordinates::new(vec![46, 19, -47, -24, 43, -37])).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_sub_fail2() {
        let v = (Coordinates::new(vec![25, 2, 2, 48, -32]) - Coordinates::new(vec![46, 19, -47, -24, 43, -37])).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_add_fail3() {
        let v = (Coordinates::new(vec![-12, 43, -14, 29, 33, 4, -47]) + Coordinates::new(vec![15, -41, -38])).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_sub_fail3() {
        let v = (Coordinates::new(vec![-12, 43, -14, 29, 33, 4, -47]) - Coordinates::new(vec![15, -41, -38])).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_add_fail4() {
        let v = (Coordinates::new(vec![30, -26, -23, -10, -37, -16]) + Coordinates::new(vec![30, -36, 7, -35, 4, 46, 25, 7, 3])).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_sub_fail4() {
        let v = (Coordinates::new(vec![30, -26, -23, -10, -37, -16]) - Coordinates::new(vec![30, -36, 7, -35, 4, 46, 25, 7, 3])).unwrap();
    }


    #[test]
    #[should_panic]
    fn test_add_fail5() {
        let v = (Coordinates::new(vec![-39, -39]) + Coordinates::new(vec![-40, -49, -39])).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_sub_fail5() {
        let v = (Coordinates::new(vec![-39, -39]) - Coordinates::new(vec![-40, -49, -39])).unwrap();
    }


    #[test]
    #[should_panic]
    fn test_add_fail6() {
        let v = (Coordinates::new(vec![26, 49, 23]) + Coordinates::new(vec![-7, 40])).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_sub_fail6() {
        let v = (Coordinates::new(vec![26, 49, 23]) - Coordinates::new(vec![-7, 40])).unwrap();
    }

    #[test]
    fn test_mul1() {
        let v = Coordinates::new(vec![7]).scale(-2.74);
        let res = Coordinates::new(vec![-19.18]);
        assert_eq!(v, res);

        let v = Coordinates::new(vec![7]) * -2.74;
        let res = Coordinates::new(vec![-19.18]);
        assert_eq!(v, res);
    }


    #[test]
    fn test_mul2() {
        let v = Coordinates::new(vec![40]).scale(-1.67);
        let res = Coordinates::new(vec![-66.8]);
        assert_eq!(v, res);

        let v = Coordinates::new(vec![40]) * -1.67;
        let res = Coordinates::new(vec![-66.8]);
        assert_eq!(v, res);
    }


    #[test]
    fn test_mul3() {
        let v = Coordinates::new(vec![41, -21]).scale(3.58);
        let res = Coordinates::new(vec![146.78, -75.18]);
        assert_eq!(v, res);

        let v = Coordinates::new(vec![41, -21]) * 3.58;
        let res = Coordinates::new(vec![146.78, -75.18]);
        assert_eq!(v, res);
    }


    #[test]
    fn test_mul4() {
        let v = Coordinates::new(vec![49, 5]).scale(-0.65);
        let res = Coordinates::new(vec![-31.85, -3.25]);
        assert_eq!(v, res);

        let v = Coordinates::new(vec![49, 5]) * -0.65;
        let res = Coordinates::new(vec![-31.85, -3.25]);
        assert_eq!(v, res);
    }


    #[test]
    fn test_mul5() {
        let v = Coordinates::new(vec![-46, 2]).scale(-2.92);
        let res = Coordinates::new(vec![134.32, -5.84]);
        assert_eq!(v, res);

        let v = Coordinates::new(vec![-46, 2]) * -2.92;
        let res = Coordinates::new(vec![134.32, -5.84]);
        assert_eq!(v, res);
    }


    #[test]
    fn test_mul6() {
        let v = Coordinates::new(vec![47, -43, -42]).scale(-2.26);
        let res = Coordinates::new(vec![-106.22, 97.18, 94.92]);
        assert_eq!(v, res);

        let v = Coordinates::new(vec![47, -43, -42]) * -2.26;
        let res = Coordinates::new(vec![-106.22, 97.18, 94.92]);
        assert_eq!(v, res);
    }


    #[test]
    fn test_mul7() {
        let v = Coordinates::new(vec![5, 19, -15]).scale(4.09);
        let res = Coordinates::new(vec![20.45, 77.71, -61.35]);
        assert_eq!(v, res);

        let v = Coordinates::new(vec![5, 19, -15]) * 4.09;
        let res = Coordinates::new(vec![20.45, 77.71, -61.35]);
        assert_eq!(v, res);
    }


    #[test]
    fn test_mul8() {
        let v = Coordinates::new(vec![-15, 6, 28, -7]).scale(-1.01);
        let res = Coordinates::new(vec![15.15, -6.06, -28.28, 7.07]);
        assert_eq!(v, res);

        let v = Coordinates::new(vec![-15, 6, 28, -7]) * -1.01;
        let res = Coordinates::new(vec![15.15, -6.06, -28.28, 7.07]);
        assert_eq!(v, res);
    }


    #[test]
    fn test_mul9() {
        let v = Coordinates::new(vec![48, 32, -4, 30, -46, -34]).scale(-3.51);
        let res = Coordinates::new(vec![-168.48, -112.32, 14.04, -105.3, 161.46, 119.34]);
        assert_eq!(v, res);

        let v = Coordinates::new(vec![48, 32, -4, 30, -46, -34]) * -3.51;
        let res = Coordinates::new(vec![-168.48, -112.32, 14.04, -105.3, 161.46, 119.34]);
        assert_eq!(v, res);
    }


    #[test]
    fn test_mul10() {
        let v = Coordinates::new(vec![24, -9, -44, 33, -9, -9, 47, 36, -43]).scale(-4.67);
        let res = Coordinates::new(vec![-112.08, 42.03, 205.48, -154.11, 42.03, 42.03, -219.49, -168.12, 200.81]);
        assert_eq!(v, res);

        let v = Coordinates::new(vec![24, -9, -44, 33, -9, -9, 47, 36, -43]) * -4.67;
        let res = Coordinates::new(vec![-112.08, 42.03, 205.48, -154.11, 42.03, 42.03, -219.49, -168.12, 200.81]);
        assert_eq!(v, res);
    }
}