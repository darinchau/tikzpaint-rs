//! Implementation of a single point. Our convention is to begin the name of every direct implementation of figure object
//! with the prefix FO-

use crate::figures::*;
use crate::core::*;

#[derive(Clone)]
pub struct FOPoint {
    point: Coordinates,
}

impl FOPoint {
    pub fn point(&self) -> Coordinates {
        return self.point.clone();
    }
}

impl FOPoint {
    pub fn new(x: Coordinates) -> Self {
        Self {
            point: x,
        }
    }
}

impl FigureObject for FOPoint {
    fn tikzify(&self) -> (String, Option<String>) {
        return (
            format!("\\filldraw[color=black] {} circle (0.8pt);", self.point),
            None
        )
    }

    fn draw_on_canvas(&self, c: HtmlCanvas) -> Result<(), DrawError> {
        return c.draw_circle(self.point, 2.);
    }

    fn repr(&self) -> String {
        format!("point{}", self.point)
    }
}
