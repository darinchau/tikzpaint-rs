//! Handles the current draw state and the logic of drawing different shapes

use crate::{app::*, core::Coordinates};

enum DrawState {
    Point,
    Curve,
    None
}

pub struct DrawHandler {
    state: DrawState,
    current: Option<FigureObjectComplex>
}

impl DrawHandler {
    pub fn new() -> Self {
        Self {
            state: DrawState::None,
            current: None
        }
    }

    pub fn set_state(&mut self, state: SideBarType) {
        self.state = match state {
            SideBarType::Path => DrawState::Curve,
            SideBarType::Point => DrawState::Point,
            _ => DrawState::None
        }
    }

    pub fn get_foc(&self, v: Coordinates) -> FigureObjectComplex {
        todo!()
    }
}

