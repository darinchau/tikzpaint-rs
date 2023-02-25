//! This is a direct implementation of a point out of the FOPoint

use crate::figures::{Coordinates, Drawable, Hashable, Serializable};

/// The implementation of a node with no contents.
///
/// Example
/// ```
/// use tikzpaint_rs::figures::{Point, Coordinates, Figure, Identity};
/// let p1 = Point::new(Coordinates::new([2, 3]));
/// let p2 = Point::new(Coordinates::new([4, 5]));
/// let mut fig = Figure::<2>::new();
/// fig.draw(&p1);
/// fig.draw(&p2);
///
/// let result = fig.output_tikz(&Identity);
/// let expect = "\\begin{tikzpicture}\n\t\\node[] at (2, 3) {}\n\t\\node[] at (4, 5) {}\n\\end{tikzpicture}";
/// assert_eq!(result, expect);
/// ```
#[derive(Clone)]
pub struct Point<const DIMS: usize> {
    p: FOPoint<DIMS>,
}

impl<const DIMS: usize> Point<DIMS> {
    pub fn new(x: Coordinates<DIMS>) -> Self {
        Point {
            p: FOPoint::new(x)
        }
    }
}

impl<const DIMS: usize> Drawable<DIMS> for Point<DIMS> {
    fn draw(&self) -> Vec<&dyn FO<DIMS>> {
        return vec![&self.p]
    }
}

impl<const DIMS: usize> Serializable for Point<DIMS>{
    fn from_str(s: &str) -> Option<Self> {
        if !s.starts_with("pt") {
            return None;
        }

        if let Some(x) = FOPoint::<DIMS>::from_str(&s[2..]) {
            return Some(Self {
                p: x,
            });
        }

        return None;
    }

    fn into_str(&self) -> String {
        format!("pt{}", self.p.into_str())
    }
}

impl<const DIMS: usize> Hashable for Point<DIMS>{
    fn hash(&self) -> i64 {
        return 7 * self.p.hash();
    }
}

impl<const DIMS: usize> DrawableObject<DIMS> for Point<DIMS>{}
