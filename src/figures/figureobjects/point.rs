//! Implementation of a node. Our convention is to begin the name of every direct implementation of figure object
//! with the prefix FO-

use crate::figures::*;

pub struct FOPoint {
    point: Coordinates,
    option: PlotOptions,
    content: String,
}

impl FOPoint {
    fn tikz_options(&self) -> String {
        let mut s = String::new();
        let opt = &self.option;
        tikzify_field(&mut s, &opt.fill_color, "fill=");
        tikzify_field(&mut s, &opt.thickness, "width=");
        return s;
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
        let new_p = p.call(&self.point).unwrap();
        let new_self = Self {
            point: new_p,
            option: self.option.clone(),
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
