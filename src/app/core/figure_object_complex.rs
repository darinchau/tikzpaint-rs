//! A figureobject complex is an object along with it's string representation.
//!
//! What happens when we click on the main canvas (say, to initialize a point):
//!  1. The canvas sensor detects the mouse input and returns the coordinates back to the canvas manager
//!  2. The canvas manager parses the mouse input along with the side bar mode etc. to make it into a figure object complex (FOC)
//!  3. The canvas manager draws the figure object complex
//!  4. The canvas manager triggers a rerender on the canvas renderer and the terminal
//!
//! What happens when we type something on the terminal (say, point(1, 2)):
//!  1. The terminal returns the text back to the canvas manager
//!  2. The canvas manager passes the text to parse() which performs the parsing and returns a Result<FOC, Error>
//!  3. If the result contains a FOC, draw it
//!  4. The canvas manager triggers a rerender on the canvas renderer and the terminal

use yew::prelude::*;
use gloo::console::log;

use crate::figures::*;
use crate::app::*;
use crate::renderer::CanvasStateHandle;
use crate::renderer::DrawError;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;


// ================================================================================================================
// =============================== Implement figure-terminal complex ==============================================
// ================================================================================================================

/// A figure object complex is a Interior mutable string, interior mutable drawable object, along with the world coordinates of this object
/// We can look up a figure object complex by nearest points.
pub struct FigureObjectComplex {
    pub st: CheapString,
    pub fo: Rc<RefCell<DrawableObject>>
}

impl Debug for FigureObjectComplex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FOC(command:{}, obj:{:?})", self.st, self.fo.borrow())
    }
}

impl FigureObjectComplex {
    pub fn new(x: DrawableObject, s: String) -> Self {
        let st = s.wrap();
        let fo = Rc::new(RefCell::new(x));
        return Self {
            st, fo
        }
    }
}

/// A figure-terminal complex is a proxy for the figure but houses figureobject complexes
pub struct FigureComplex {
    basis: Vec<FigureObjectComplex>,
    fig: Figure,
    ttext: TerminalTextRenderer,
    newly_drawn: Vec<FigureObjectComplex>,
}

impl FigureComplex {
    pub fn new(dims: usize) -> Self {
        FigureComplex {
            basis: vec![],
            fig: Figure::new(dims),
            ttext: TerminalTextRenderer::new(),
            newly_drawn: vec![]
        }
    }

    /// Draws a figure object complex on self. Returns nothing if it is successful, and bubbles an error if the dimension is wrong.
    pub fn draw(&mut self, d: FigureObjectComplex) -> Result<(), DimensionError> {
        let obj = self.fig.draw(d.fo.borrow().clone());
        if let Err(x) = obj {
            return Err(x);
        }

        let text_copy = d.st.clone();
        self.ttext.push(text_copy);

        return Ok(());
    }

    /// Draws a figure with the text prompt. Offloads the text to the parser
    pub fn draw_with_text<S1: StringLike>(&mut self, s: S1) -> Result<(), ParserError> {
        log!(format!("Trying to draw {}", s));
        let wrapped_text = s.wrap();
        let foc = parse(wrapped_text.clone())?;

        log!(format!("Translates to {:?}", foc));

        // Draw on the figure
        if let Err(e) = self.fig.draw(foc.fo.borrow().clone()) {
            let er_msg = format!("Dimension error: {}", e.msg);
            return Err(ParserError {
                error_type: ParserErrorType::DimensionError,
                msg: er_msg,
                src: e.source
            });
        }

        self.ttext.push(wrapped_text);
        Ok(())
    }

    /// This unpacks the figure complex into a bunch of terminal commands.
    /// Main method used to render terminal text
    pub fn unpack_html(&self) -> Html {
        self.ttext.unpack_html()
    }

    /// Renders the canvas
    pub fn render(&self, canvas: CanvasStateHandle) -> Result<(), DrawError> {
        let y = self.fig.render(|x| {
            x.draw_on_canvas(canvas.clone())
        }, Identity{dims: 2}).unwrap();

        for x in y {
            x?;
        }

        Ok(())
    }

    /// Rerenders the canvas
    pub fn rerender(&self, canvas: CanvasStateHandle) -> Result<(), DrawError> {
        log!("Redrawing canvas");
        let y = self.fig.load_all(|x| {
            x.draw_on_canvas(canvas.clone())
        }, Identity{dims: 2}).unwrap();

        for x in y {
            x?;
        }

        log!("Successfully redrawn canvas");

        Ok(())
    }
}