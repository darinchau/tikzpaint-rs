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

use yew::Html;

use crate::figures::*;
use crate::app::*;
use std::rc::Rc;
use std::cell::RefCell;


// ================================================================================================================
// =============================== Implement figure-terminal complex ==============================================
// ================================================================================================================

/// A figure object complex is a Interior mutable string, interior mutable drawable object, along with the world coordinates of this object
/// We can look up a figure object complex by nearest points.
pub struct FigureObjectComplex {
    st: Rc<RefCell<CheapString>>,
    fo: Rc<RefCell<DrawableObject>>
}

impl FigureObjectComplex {
    pub fn new(x: DrawableObject, s: String) -> Self {
        let st = Rc::new(RefCell::new(CheapString::new(s)));
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
}

impl FigureComplex {
    pub fn new(dims: usize) -> Self {
        FigureComplex {
            basis: vec![],
            fig: Figure::new(dims),
            ttext: TerminalTextRenderer::new()
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

    /// This unpacks the figure complex into a bunch of terminal commands.
    /// Main method used to render terminal text
    pub fn unpack_html(&self) -> Html {
        self.ttext.unpack_html()
    }

    /// This unpacks the figure complex into an svg figure.
    /// We allow string like types (CheapString, &str, String) because we prinarily pass in calc expressions from html.
    pub fn unpack_svg<S1: StringLike, S2: StringLike, P: IsProjection>(&self, height: S1, width: S2, proj: P) -> Result<Html, DimensionError> {
        let h = height.wrap();
        let w = width.wrap();

        let y = self.fig.load(|x| {
            let y = x.get_svg().draw(h.clone(), w.clone());
            return Html::from_html_unchecked(y.into());
        }, proj)?;

        return Ok(y.iter().map(|x| x.clone()).collect::<Html>());
    }
}