//! Implementation of a node. Our convention is to begin the name of every direct implementation of figure object
//! with the prefix FO-

use crate::figures::{Coordinates, PlotOptions, FigureObject, Serializable, Hashable, Plottable, tikzify_field, Projection, DimensionError, IsProjection};

pub struct FOPoint {
    point: Coordinates,
    option: PlotOptions,
    content: String,
}

impl FOPoint {
    fn tikz_options(&self) -> String {
        let mut s = String::new();
        let opt = self.options();
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

impl FigureObject for FOPoint {
    fn coordinates(&self) -> Vec<Coordinates> {
        vec![self.point.clone()]
    }

    fn options(&self) -> &PlotOptions {
        &self.option
    }

    fn len(&self) -> usize {
        1
    }

    fn project(&mut self, p: Projection) -> Result<(), DimensionError> {
        if p.input() != self.dims() {
            return Err(DimensionError {
                msg: format!("Cannot apply a projection with dimension {} on a point with {} dimensions", p.dims(), self.dims()),
                source: "project() from Point",
            })
        }
        let new_p = p.call(&self.point);

        if let Ok(c) = new_p {
            self.point = c;
            return Ok(());
        }
        else {
            return Err(new_p.err().unwrap());
        }
    }

    fn dims(&self) -> usize {
        return self.point.dims;
    }

    fn project_to_plot(&self, p: Projection) -> Result<Box<dyn Plottable>, DimensionError> {
        if p.input() != self.dims() {
            return Err(DimensionError {
                msg: format!("Expect the output dimension of the projection {} to be same as the dimension of the point ({})", p.dims(), self.dims()),
                source: "project_to_plot() from Point"
            });
        }

        if p.output() != 2 {
            return Err(DimensionError {
                msg: format!("Expect the output dimension of the projection {} to be 2", p.dims()),
                source: "project_to_plot() from Point"
            });
        }

        let new_p = FOPoint{
            point: p.call(&self.point).unwrap(),
            option: self.option.clone(),
            content: self.content.clone()
        };

        let res = Box::new(new_p) as Box<dyn Plottable>;
        return Ok(res);
    }
}

impl Serializable for FOPoint {
    fn from_str(s: &str) -> Option<Self> {
        let mut split = s.split("--");
        if split.next()? != "fpt" {
            return None;
        }

        let point = Coordinates::from_str(split.next()?)?;
        let option = PlotOptions::from_str(split.next()?)?;
        let content = String::from_str(split.next()?)?;

        return Some(Self {
            point, option, content
        });
    }

    fn into_str(&self) -> String {
        format!("fpt--{}--{}--{}", self.point.into_str(), self.option.into_str(), self.content)
    }
}