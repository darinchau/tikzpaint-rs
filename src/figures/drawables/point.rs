//! This is a direct implementation of a point out of the FOPoint

use crate::figures::*;

/// The implementation of a node with no contents.
///
/// Example
/// ```
/// use tikzpaint_rs::figures::*;
/// let p1 = Point::new(Coordinates::new(vec![2, 3]));
/// let p2 = Point::new(Coordinates::new(vec![4, 5]));
/// let mut fig = Figure::new(2);
/// fig.draw(p1);
/// fig.draw(p2);
/// let result = fig.output_tikz(Identity{dims: 2}).unwrap();
/// let expect = "\\begin{tikzpicture}\n\t\\node[] at (2, 3) {}\n\t\\node[] at (4, 5) {}\n\\end{tikzpicture}";
/// assert_eq!(result, expect);
/// ```

pub struct Point {
    p: FigureObject,
}

impl Point {
    pub fn new(x: Coordinates) -> Self {
        Point {
            p: FOPoint::new(x).wrap(),
        }
    }
}

impl Drawable for Point {
    fn draw(&self) -> Vec<FigureObject> {
        return vec![self.p.clone()];
    }

    fn dims(&self) -> usize {
        return self.p.dims();
    }
}

impl Serializable for Point {
    fn from_str(s: &str) -> Option<Self> {
        if !s.starts_with("point") {
            return None;
        }

        if let Some(x) = Coordinates::from_str(&s[5..]) {
            return Some(Point::new(x));
        }

        return None;
    }

    fn into_str(&self) -> String {
        format!("point{}", self.p.coordinates()[0].into_str())
    }
}
