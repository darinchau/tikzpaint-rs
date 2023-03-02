//! Implementation of a node. Our convention is to begin the name of every direct implementation of figure object
//! with the prefix FO-

use crate::figures::*;

pub struct FOPoint {
    point: Coordinates,
    content: String,
}

impl FOPoint {
    pub fn new(x: Coordinates) -> Self {
        Self {
            point: x,
            content: String::from("")
        }
    }
}

impl Plottable for FOPoint {
    fn tikzify(&self) -> String {
        format!("\\node[{}] at {} {{}}", self.tikz_options(), self.point)
    }

    fn get_svg(&self) -> SVG {
        let (x, y) = (self.point[0], self.point[1]);
        SVG::new()
            .draw(SVGCircle::new(x, y, 0.1))
    }
}

impl IsFigureObject for FOPoint {
    fn coordinates(&self) -> Vec<Coordinates> {
        vec![self.point.clone()]
    }

    fn len(&self) -> usize {
        1
    }

    fn project(&self, p: Projection) -> FigureObject {
        let new_p = p.project(&self.point).unwrap();
        let new_self = Self {
            point: new_p,
            content: self.content.clone()
        };

        return new_self.wrap();
    }

    fn dims(&self) -> usize {
        return self.point.dims;
    }

    fn name(&self) -> &'static str {
        "point"
    }
}

