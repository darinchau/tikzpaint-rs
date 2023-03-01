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

impl FOPoint {
    fn tikz_options(&self) -> String {
        return String::new();
    }
}

impl Plottable for FOPoint {
    fn tikzify(&self) -> String {
        format!("\\node[{}] at {} {{}}", self.tikz_options(), self.point)
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

