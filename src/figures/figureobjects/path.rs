//! A path created with the tikz path as reference. It will be capable of drawing circles and so many more stuff
use crate::figures::*;
use crate::core::*;

pub enum PathDrawStyle {
    /// Draws the line
    Draw,

    /// Fills the inner parts
    Fill,

    /// Fill and draw
    FillDraw
}

impl PathDrawStyle {
    fn tikzify(&self) -> String {
        let s = match self {
            PathDrawStyle::Draw => "\\draw ",
            PathDrawStyle::Fill => "\\fill ",
            PathDrawStyle::FillDraw => "\\filldraw "
        };

        String::from(s)
    }

    fn repr(&self) -> &'static str {
        match self {
            PathDrawStyle::Draw => "d ",
            PathDrawStyle::Fill => "f ",
            PathDrawStyle::FillDraw => "fd "
        }
    }
}

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

    /// This path is made up of two segments: going the x direction first then the y direction
    LineXY{to: Coordinates},

    /// This path is made up of two segments: going the y direction first then the x direction
    LineYX{to: Coordinates},

    /// Circle
    Circle{radius: f64},

    /// Ellipse and general arcs
    Arc{start_angle: f64, end_angle: f64, x_radius: f64, y_radius: f64}
}

impl PathType {
    /// Takes canvas and coordinates. Return new pen coordinates
    fn draw_on_canvas(&self, c: HtmlCanvas, coord: Coordinates) -> Result<Coordinates, DrawError> {
        let z = match self {
            PathType::Arc { start_angle, end_angle, x_radius, y_radius } => {
                todo!()
            },

            PathType::Circle { radius } => {
                c.draw_circle(coord, *radius)?;
                coord
            },

            PathType::Cubic { control_start, control_end, to } => {
                todo!()
            },

            PathType::Line { to } => {
                c.draw_line(coord, *to)?;
                *to
            },

            PathType::LineXY { to } => {
                let intermediate = Coordinates::new(to[0], coord[1]);
                c.draw_line(coord, intermediate)?;
                c.draw_line(intermediate, *to)?;
                *to
            },

            PathType::LineYX { to } => {
                let intermediate = Coordinates::new(coord[0], to[1]);
                c.draw_line(coord, intermediate)?;
                c.draw_line(intermediate, *to)?;
                *to
            },

            PathType::Quadratic { control, to } => {
                todo!()
            },

            PathType::Rectangle { to } => {
                c.draw_rectangles(coord, *to);
                *to
            }
        };

        Ok(z)
    }

    fn repr(&self) -> String {
        match self {
            PathType::Arc { start_angle, end_angle, x_radius, y_radius } => format!("Arc({start_angle},{end_angle},{x_radius},{y_radius})"),
            PathType::Circle { radius } => format!("Circ({radius})"),
            PathType::Cubic { control_start, control_end, to } => format!("Cb{control_start}{control_end}{to}"),
            PathType::Line { to } => format!("L{to}"),
            PathType::LineXY { to } => format!("Lxy{to}"),
            PathType::LineYX { to } => format!("Lyx{to}"),
            PathType::Quadratic { control, to } => format!("Qbezier{control}{to}"),
            PathType::Rectangle { to } => format!("Rect{to}")
        }
    }

    fn tikzify(&self) -> String {
        match self {
            PathType::Arc { start_angle, end_angle, x_radius, y_radius } => format!("arc ({start_angle}:{end_angle}:{x_radius} and {y_radius})"),
            PathType::Circle { radius } => format!("circle ({radius})"),
            PathType::Cubic { control_start, control_end, to } => format!(".. controls {control_start} and {control_end}..{to}"),
            PathType::Line { to } => format!("-- {to}"),
            PathType::LineXY { to } => format!("-| {to}"),
            PathType::LineYX { to } => format!("|- {to}"),
            PathType::Quadratic { control, to } => format!(".. controls {control} .. {to};"),
            PathType::Rectangle { to } => format!("rectangle {to}")
        }
    }
}

/// Implementation of a tikz path
pub struct FOPath {
    style: PathDrawStyle,
    start: Coordinates,
    data: Vec<PathType>
}

impl FigureObject for FOPath {
    fn draw_on_canvas(&self, c: HtmlCanvas) -> Result<(), DrawError> {
        let mut current_coords = self.start;

        for x in self.data.iter() {
            match self.style {
                PathDrawStyle::Draw => {
                    current_coords = x.draw_on_canvas(c.clone(), current_coords)?;
                }

                _ => todo!()
            }
        }

        Ok(())
    }

    fn repr(&self) -> String {
        let mut s = String::from(self.style.repr());
        s.push_str(&self.start.to_string());

        for x in self.data.iter() {
            s.push_str(&x.repr());
        }

        return s
    }

    fn tikzify(&self) -> (String, Option<String>) {
        let mut s = self.style.tikzify();
        s.push_str(&self.start.to_string());

        for x in self.data.iter() {
            s.push_str(&x.repr());
        }

        return (s, None)
    }
}