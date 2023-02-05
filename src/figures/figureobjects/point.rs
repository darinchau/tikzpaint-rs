//! Implementation of a node. Our convention is to begin the name of every direct implementation of figure object
//! with the prefix FO-

use crate::figures::{PlotOptions, Coordinates, FigureObject, Plot, Projection};

pub struct FOPoint<const DIMS: usize> {
    point: Coordinates<DIMS>,
    option: PlotOptions,
    content: String,
}

impl<const DIMS: usize> FOPoint<DIMS> {
    pub fn new(c: Coordinates<DIMS>) -> FOPoint<DIMS> {
        FOPoint {
            point: c,
            option: PlotOptions::new(),
            content: String::new(),
        }
    }

    pub fn push_content(&mut self, content: &str) {
        self.content.push_str(content);
    }
}

impl<const DIMS: usize> FigureObject<DIMS> for FOPoint<DIMS> {
    fn coordinates(&self) -> Vec<Coordinates<DIMS>> {
        vec![self.point.clone()]
    }

    fn options(&self) -> &PlotOptions {
        &self.option
    }

    fn len(&self) -> usize {
        1
    }

    fn project(&self, proj: &Box<&dyn Projection<DIMS, 2>>) -> Box<dyn Plot> {
        let new_p = (*proj).call(&self.point);
        return Box::new(FOPoint{
            point: new_p,
            option: self.option.clone(),
            content: self.content.clone()
        });
    }
}

impl Plot for FOPoint<2> {
    fn tikzify(&self) -> String {
        format!("\\node[{}] at {} {{}}", self.options().tikzify(), self.point)
    }
}