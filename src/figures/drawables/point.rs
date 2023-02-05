//! This is a direct implementation of a point out of the FOPoint

use crate::figures::{Coordinates, Drawable, FigureObject, FOPoint};

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
    fn draw(&self) -> Vec<&dyn FigureObject<DIMS>> {
        return vec![&self.p]
    }
}