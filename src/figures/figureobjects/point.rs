//! Implementation of a node. Our convention is to begin the name of every direct implementation of figure object
//! with the prefix FO-

use crate::figures::*;

#[derive(Clone)]
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

impl<const DIMS: usize> FO<DIMS> for FOPoint<DIMS> {
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

impl<const DIMS: usize> Hashable for FOPoint<DIMS>  {
    fn hash(&self) -> i64 {
        return 17 * self.point.hash();
    }
}

impl Plot for FOPoint<2> {
    fn tikzify(&self) -> String {
        format!("\\node[{}] at {} {{}}", self.tikz_options(), self.point)
    }

    fn tikz_options(&self) -> String {
        let mut s = String::new();
        let opt = self.options();
        tikzify_field(&mut s, &opt.fill_color, "fill=");
        tikzify_field(&mut s, &opt.thickness, "width=");
        return s;
    }
}