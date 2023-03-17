//! A path created with the tikz path as reference. It will be capable of drawing circles and so many more stuff
use crate::renderer::*;
use crate::figures::*;

// Possible things to draw in a tikz path
pub enum PathType {
    /// Straight line segment
    Line{to: Coordinates},

    /// Quadratic Bezier Curve
    Quadratic{control: Coordinates, to: Coordinates},

    /// Cubic Bezier Curve
    Cubic{control_start: Coordinates, control_end: Coordinates, to: Coordinates},

    /// Rectangle
    Rectangle{to: Coordinates},

    // /// This path is made up of two segments: going the x direction first then the y direction
    // LineXY{to: Coordinates},

    // /// This path is made up of two segments: going the y direction first then the x direction
    // LineYX{to: Coordinates}

    // Circle{radius: f64},
}

/// Implementation of a tikz path
pub struct Path {
    start: Coordinates,
    data: PathType
}

impl FigureObject for Path {
    fn draw_on_canvas(&self, c: HtmlCanvas) -> Result<(), DrawError> {
        todo!()
    }

    fn repr(&self) -> String {
        todo!()
    }

    fn tikzify(&self) -> String {
        todo!()
    }
}