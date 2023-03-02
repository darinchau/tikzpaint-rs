use std::rc::Rc;
use std::fmt::Display;
use paste::paste;
use yew::Html;

use crate::figures::{CheapString, StringLike};

#[derive(Clone, Copy, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn to_string(&self) -> String {
        let Color(r, g, b) = self;
        return format!("rgb({r},{g},{b})");
    }
}

macro_rules! svg_properties {
    {$($name:ident : $t:ty, $id:expr),*} => {
        #[derive(Clone, Copy, PartialEq)]
        pub struct SVGProperty {
            $ (
                $name: Option<$t>,
            )*
        }

        impl SVGProperty {
            $ (
                paste::item! {
                    fn [< set_$name >] (&mut self, c: $t) -> &mut Self {
                        self.$name = Some(c);
                        return self;
                    }
                }
            )*

            pub fn to_string(&self) -> String {
                let mut s = String::new();

                $ (
                    if let Some(c) = self.$name {
                        s.push_str($id);
                        s.push_str(&format!(":{};", c.to_string()));
                    }
                )*

                return s;
            }
        }
    }
}

svg_properties! {
    fill: Color, "fill",
    stroke: Color, "stroke",
    stroke_width: f64, "stroke-width",
    fill_opacity: f64, "fill-opacity",
    stroke_opacity: f64, "stroke-opacity"
}

impl Display for SVGProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub trait SVGShape {
    fn draw(&self) -> String;
}

macro_rules! svg_shape {
    {$($name:ident: $fmt:expr, ($($x:ident), *)),*} => {
        $ (
            paste::item! {
                pub struct [< SVG$name >] {
                    $(
                        $x: f64,
                    )*
                    props: SVGProperty
                }


                impl SVGShape for [< SVG $name >] {
                    fn draw(&self) -> String {
                        let mut s = String::from("<");
                        s.push_str(stringify!($fmt));
                        $(
                            s.push_str(stringify!($x));
                            s.push_str(&format!("=\"{}\" ", self.$x));
                        )*
                        s.push_str(&format!("style=\"{}\"/>", self.props.to_string()));
                        return s;
                    }
                }
            }
        )*
    };
}

svg_shape!{
    Rectangle: "rect", (x, y, a, b),
    Circle: "circle", (cx, cy, r),
    Ellipse: "ellipse", (cx, cy, rx, ry),
    Line: "line", (x1, y1, x2, y2)
}

#[derive(Clone, Copy, PartialEq, Debug)]
/// We only support absolute position :)
pub enum SVGPathElements {
    MoveTo{x: f64, y: f64},
    LineTo{x: f64, y: f64},
    HorizontalLineTo{new_x: f64},
    VerticalLineTo{new_y: f64},
    CurveTo{ctrl_pt1_x: f64, ctrl_pt1_y: f64, ctrl_pt2_x: f64, ctrl_pt2_y: f64, x: f64, y: f64},
    SmoothCurveTo{ctrl_pt2_x: f64, ctrl_pt2_y: f64, x: f64, y: f64},
    QuadraticBezierCurve{ctrl_pt_x: f64, ctrl_pt_y: f64, x: f64, y: f64},
    SmoothQuadraticBezierCurveTo{x: f64, y: f64},
    EllipticalArc{rx: f64, ry: f64, x_axis_rotation: f64, large_arc_flag: bool, sweep_flag: bool, x:f64, y:f64},
    ClosePath,
}

pub struct SVGPath {
    v: Vec<SVGPathElements>
}

impl SVGPath {
    /// A path must start with a move so enter the first move coordinates
    pub fn new(x: f64, y: f64) -> Self {
        let v = vec![SVGPathElements::MoveTo{x, y}];
        SVGPath { v }
    }

    pub fn add(mut self, svg_element: SVGPathElements) -> Self {
        self.v.push(svg_element);
        return self;
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        let mut add_close = self.v[self.v.len() - 1] != SVGPathElements::ClosePath;

        for ee in self.v.iter() {
            match ee {
                SVGPathElements::MoveTo{x, y} => {s.push_str(&format!("M{x},{y}"))},
                SVGPathElements::LineTo{x, y} => {s.push_str(&format!("L{x},{y}"))},
                SVGPathElements::HorizontalLineTo{new_x} => {s.push_str(&format!("H{new_x}"))},
                SVGPathElements::VerticalLineTo{new_y} => {s.push_str(&format!("V{new_y}"))},
                SVGPathElements::CurveTo{ctrl_pt1_x, ctrl_pt1_y, ctrl_pt2_x, ctrl_pt2_y, x, y} => {s.push_str(&format!("C{ctrl_pt1_x},{ctrl_pt1_y} {ctrl_pt2_x},{ctrl_pt2_y} {x},{y}"))},
                SVGPathElements::SmoothCurveTo{ctrl_pt2_x, ctrl_pt2_y, x, y} => {s.push_str(&format!("S{ctrl_pt2_x},{ctrl_pt2_y} {x},{y}"))},
                SVGPathElements::QuadraticBezierCurve{ctrl_pt_x, ctrl_pt_y, x, y} => {s.push_str(&format!("Q{ctrl_pt_x},{ctrl_pt_y} {x},{y}"))},
                SVGPathElements::SmoothQuadraticBezierCurveTo{x, y} => {s.push_str(&format!("T{x},{y}"))},
                SVGPathElements::EllipticalArc{rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y} => {
                    let laf = if *large_arc_flag {"1"} else {"0"};
                    let sf = if *sweep_flag {"1"} else {"0"};
                    s.push_str(&format!("A{rx} {ry} {x_axis_rotation} {laf} {sf} {x},{y}"));
                },
                SVGPathElements::ClosePath => {s.push_str("Z")},
            }
        }

        if add_close {
            s.push_str("Z");
        }

        return s;
    }
}

impl SVGShape for SVGPath {
    fn draw(&self) -> String {
        self.to_string()
    }
}

pub struct SVG {
    data: Vec<Rc<dyn SVGShape>>
}

impl SVG {
    pub fn new() -> Self {
        Self {
            data: vec![]
        }
    }

    pub fn add<T: SVGShape + 'static>(&mut self, s: T) {
        self.data.push(Rc::new(s) as Rc<dyn SVGShape>);
    }

    pub fn add_from(&mut self, s: Rc<dyn SVGShape>) {
        self.data.push(s);
    }

    /// Draws the svg figure. We allow height and width to be Strings be
    pub fn draw<S1: StringLike, S2: StringLike>(&self, height: S1, width: S2) -> String {
        let h = self.data.iter().map(|x| x.draw()).collect::<Vec<String>>().join("\n");
        format!("<svg width=\"{width}\" height=\"{height}\">{h}</svg>")
    }

    pub fn get_components(&self) -> Vec<Rc<dyn SVGShape>> {
        return self.data.iter().map(|x| x.clone()).collect();
    }
}