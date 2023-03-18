//! The implementation of a straight line segment on the canvas

use crate::figures::*;
use crate::core::*;

#[derive(Clone, Copy)]
pub struct FOLine {
    a: Coordinates,
    b: Coordinates
}

impl FOLine {
    pub fn new(from: Coordinates, to: Coordinates) -> Self {
        Self {
            a: from,
            b: to
        }
    }
}

impl FigureObject for FOLine {
    fn tikzify(&self) -> (String, Option<String>) {
        let a = format!("\\draw {} -- {};", self.a, self.b);
        (a, None)
    }

    fn draw_on_canvas(&self, c: HtmlCanvas) -> Result<(), DrawError> {
        c.draw_line(self.a, self.b)
    }

    fn repr(&self) -> String {
        format!("line{}{}", self.a, self.b)
    }
}
