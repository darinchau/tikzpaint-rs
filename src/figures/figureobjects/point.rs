//! Implementation of a single point. Our convention is to begin the name of every direct implementation of figure object
//! with the prefix FO-

use crate::figures::*;
use crate::renderer::*;

#[derive(Clone)]
pub struct FOPoint {
    point: Coordinates,
}

impl FOPoint {
    pub fn new(x: Coordinates) -> Self {
        Self {
            point: x,
        }
    }
}

impl Plottable for FOPoint {
    fn tikzify(&self) -> String {
        let (x, y) = (self.point[0], self.point[1]);
        todo!()
    }

    fn draw_on_canvas(&self, c: CanvasStateHandle) -> Result<(), DrawError> {
        return c.draw_circle((self.point[0], self.point[1]), 2.);
    }
}

impl IsFigureObject for FOPoint {
    fn coordinates(&self) -> Vec<Coordinates> {
        vec![self.point.clone()]
    }

    fn len(&self) -> usize {
        1
    }

    fn project(&self, p: Projection) -> Self {
        let new_p = p.project(&self.point).unwrap();
        let new_self = Self {
            point: new_p,
        };

        return new_self;
    }

    fn dims(&self) -> usize {
        return self.point.dims;
    }

    fn name(&self) -> &'static str {
        "point"
    }
}

