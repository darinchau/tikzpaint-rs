//! The core logic of the wrapper app, responsible for synchronization of everything.
//! The actual "render on all different types of canvas" logic is the figures part of the app
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
//!
//! This also handles the side bar state and how we draw certain objects across multiple frames

use yew::prelude::*;
use gloo::console::log;

use crate::figures::*;
use crate::app::*;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

use crate::core::*;

enum DrawState {
    Point,
    Curve,
    None
}

#[derive(Clone)]
/// A figure object complex is a string plus interior mutable drawable object.
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

#[derive(Debug)]
pub enum FigureRenderType {
    DoNothing,
    Render,
    Rerender,
    RerenderLast,
    Error(CheapString)
}

/// A figure-terminal complex is a proxy for the figure but houses figureobject complexes
pub struct FigureComplex {
    basis: Vec<FigureObjectComplex>,
    fig: Figure,
    ttext: TerminalTextRenderer,
    state: DrawState,
    trail: ScopedVec<Coordinates>
}

impl FigureComplex {
    pub fn new() -> Self {
        FigureComplex {
            basis: vec![],
            fig: Figure::new(),
            ttext: TerminalTextRenderer::new(),
            state: DrawState::None,
            trail: ScopedVec::new()
        }
    }

    /// Draws a figure object complex on self.
    fn draw(&mut self, d: FigureObjectComplex) {
        self.basis.push(d.clone());
        self.fig.draw(d.fo.borrow().clone());
        self.ttext.push(d.st.clone());
    }

    /// Undos the last drawn figure
    pub fn undo(&mut self) -> Option<FigureObjectComplex> {
        self.fig.undo();
        self.ttext.pop();
        return self.basis.pop();
    }

    /// Draws a figure with the text prompt. Offloads the text to the parser
    pub fn draw_with_text<S1: StringLike>(&mut self, s: S1) -> Result<(), ParserError> {
        log!(format!("Trying to draw {}", s));
        let wrapped_text = s.wrap();

        // Draw on the figure
        if let Some(focs) = parse(wrapped_text.clone())? {
            for foc in focs.into_iter() {
                log!(format!("Drawing {:?}", foc));
                self.fig.draw(foc.fo.borrow().clone());
            }
        }
        else {
            // Nothing to draw but we weren't dead - means its a valid command that expands to something that we do not have to draw
            log!(format!("Pushing {}", s));
        }

        self.ttext.push(wrapped_text.clone());

        Ok(())
    }

    /// This unpacks the figure complex into a bunch of terminal commands.
    /// Main method used to render terminal text
    pub fn get_terminal_text(&self) -> Html {
        self.ttext.unpack_html()
    }

    /// Renders the canvas
    pub fn render(&self, canvas: HtmlCanvas) -> Result<(), DrawError> {
        let y = self.fig.render(|x| {
            x.draw_on_canvas(canvas.clone())
        });

        for x in y {
            x?;
        }

        Ok(())
    }

    /// Rerenders the canvas
    pub fn rerender(&self, canvas: HtmlCanvas) -> Result<(), DrawError> {
        let y = self.fig.load_all(|x| {
            x.draw_on_canvas(canvas.clone())
        });

        for x in y {
            x?;
        }

        Ok(())
    }

    /// Rerenders the last object of the canvas
    pub fn rerender_last(&self, canvas: HtmlCanvas) -> Result<(), DrawError> {
        let y = self.fig.load_last(|x| {
            x.draw_on_canvas(canvas.clone())
        });

        for x in y {
            x?;
        }

        Ok(())
    }

    pub fn set_state(&mut self, state: SideBarType) {
        self.state = match state {
            SideBarType::Path => DrawState::Curve,
            SideBarType::Point => DrawState::Point,
            _ => DrawState::None
        }
    }

    /// Resets the trail by creating a new Rc
    fn reset_trail(&mut self) {
        self.trail.close();
        self.trail = ScopedVec::new();
    }

    /// This represents a click on the canvas. Handles this click on the local coordinates and transforms it into an object
    pub fn click(&mut self, v: Coordinates) -> FigureRenderType {
        match self.state {
            DrawState::Point => {
                log!("I should draw a point");
                let p = Point::new(v);
                let s = p.repr();
                self.draw(FigureObjectComplex::new(p.wrap(), s));
                return FigureRenderType::Render;
            },

            _ => {
                return FigureRenderType::DoNothing;
            }
        };
    }

    /// This is called when we sense that the user starts dragging
    pub fn start_dragging(&mut self, v: Coordinates) -> FigureRenderType {
        match self.state {
            DrawState::Curve => {
                self.trail.push(v);
                let c = Curve::new(self.trail.shallow_copy());
                let s = c.repr();
                self.draw(FigureObjectComplex::new(c.wrap(), s));
                return FigureRenderType::Render;
            },

            _ => ()
        }
        return FigureRenderType::DoNothing
    }

    /// This is called when we sense that the user is dragging
    pub fn dragging(&mut self, v: Coordinates) -> FigureRenderType {

        match self.state {
            DrawState::Curve => {
                self.trail.push(v);
                // Ok this is actually handwavy unsafe rust. What (ought to) happen is that the curve
                // stores a copy of self.trail, which we have just appended the new coordinates
                // Now we just need to rerender the canvas and the curve should draw itself with
                // the new coordinates

                return FigureRenderType::RerenderLast;
            },

            _ => ()
        }
        return FigureRenderType::DoNothing;
    }

    /// This is called when we sense that the user stops dragging
    pub fn stop_dragging(&mut self, v: Coordinates) -> FigureRenderType {
        // Reset_trail resets the trail by completely making a new Rc
        // so the Rc in the curve stays in the curve and is not affected.
        self.reset_trail();

        match self.state {
            DrawState::Curve => {
                FigureRenderType::Rerender
            },

            // The render method for a point will be fired during the click event instead of the stop-dragging eent
            DrawState::Point => {
                FigureRenderType::DoNothing
            }

            _ => FigureRenderType::DoNothing
        }
    }
}
