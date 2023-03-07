//! Creates Tikz command bindings to Rust so we don't have to manipulate raw Tikz in our code

use std::rc::Rc;
use std::fmt::Display;
use crate::figures::*;

pub trait TikzShape {
    fn draw(&self) -> (String, Option<String>);
}

pub enum TikzDrawMode {
    Draw,
    Fill,
    FillDraw
}

impl TikzDrawMode {
    pub fn to_start_command(&self) -> &'static str {
        match self {
            TikzDrawMode::Draw => "\\draw",
            TikzDrawMode::Fill => "\\fill",
            TikzDrawMode::FillDraw => "\\filldraw",
        }
    }
}

// ===============================================================================================================
// ===============================================================================================================
// ===============================================================================================================

macro_rules! tikz_shape {
    {$($name:ident: $fmt:expr, ($($x:ident), *)),*} => {
        $ (
            paste::item! {
                pub struct [< Tikz $name >] {
                    $(
                        $x: f64,
                    )*
                    mode: TikzDrawMode
                }

                impl [< Tikz $name >] {
                    pub fn new($(
                        $x: f64,
                    )* mode: Option<TikzDrawMode>) -> Self {
                        if let Some(m) = mode {
                            return Self {$(
                                $x,
                            )* mode: m};
                        }

                        return Self {$(
                            $x,
                        )* mode: TikzDrawMode::Draw};
                    }
                }

                impl TikzShape for [< Tikz $name >] {
                    fn draw(&self) -> (String, Option<String>) {
                        let expr = format!($fmt, $(
                            self.$x,
                        )*);
                        let rs = format!("{}, {};", self.mode.to_start_command(), expr);
                        return (rs, None);
                    }
                }
            }
        )*
    };
}

tikz_shape!{
    Rectangle: "({}, {}) rectangle ({}, {})", (start_x, start_y, end_x, end_y),
    Circle: "({}, {}) circle ({})", (x, y, radius),
    Ellipse: "({}, {}) ellipse ({} and {})", (x, y, radius_x, radius_y)
}


pub struct TikzFigure {
    data: Vec<Rc<dyn TikzShape>>
}

impl TikzFigure {
    pub fn new() -> Self {
        Self {
            data: vec![]
        }
    }

    /// Adds an element to the svg
    pub fn draw<T: TikzShape + 'static>(mut self, s: T) -> Self {
        self.data.push(Rc::new(s) as Rc<dyn TikzShape>);
        return self;
    }

    /// Draws the svg figure. We allow height and width to be Strings be
    pub fn output(&self) -> String {
        let mut body = vec![];
        let mut preamble = vec![];

        for x in self.data.iter() {
            let (b, p) = x.draw();
            body.push(b);
            if let Some(pre) = p {
                preamble.push(pre);
            }
        }

        let body_text = format!(r#"
        \\begin{{tikzpicture}}
            {}
        \\end{{tikzpicture}}
        "#, body.join("\n"));

        if preamble.len() > 0 {
            preamble.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            let preams = preamble.join("\n");
            let result = format!(r#"
                % This is the preamble section. Please include it in the beginning of your document
                {preams}

                % This is the body of your Tikz Figure
                {body_text}
            "#);

            return result;
        }

        return body_text;
    }
}
