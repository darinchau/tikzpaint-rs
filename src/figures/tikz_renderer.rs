//! Creates Tikz command bindings to Rust so we don't have to manipulate raw Tikz in our code

use std::rc::Rc;
use std::fmt::Display;
use crate::figures::*;

pub trait TikzShape {
    /// This returns the command and the preamble
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
