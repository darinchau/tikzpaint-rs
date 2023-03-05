//! This module serves as means to translate between coordinate systems.
//! There are currently 3 in total:
//! - screen coordinates (e.g. mouse click)
//! - local coordinates (e.g. figure)
//! - client coordiantes (e.g. canvas)

use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone, Copy)]
pub struct Transform {
    /// Screen size according to inner_width and inner_height
    pub screen_size: (i32, i32),

    /// scale x is the number such that 1 unit in the coordinate system horizontally = scale_x pixels
    pub scale_x: f64,

    /// scale y is the number such that 1 unit in the coordinate system vertically = scale_y pixels
    pub scale_y: f64,

    /// Where the origin ought to be in world coordinates. Coordinates can be very big or negative - means the origin is out of sight
    pub origin: (i32, i32),

    pub margins: (i32, i32, i32, i32),

    pub initialized: bool,
}

impl Transform {
    /// Abuse of notation
    pub fn new(header_height: usize, sidebar_width: usize, terminal_height: usize) -> Self {
        let h = header_height as i32;
        let w = sidebar_width as i32;
        let th = terminal_height as i32;

        Self {
            screen_size: (1920, 1080),
            scale_x: 25.,
            scale_y: 25.,
            origin: (0, 0),
            margins: (h, 0, th, w),
            initialized: false,
        }
    }

    /// Returns size of main canvas, in width and height
    pub fn get_self_size(&self) -> (i32, i32) {
        let (x, y) = self.screen_size;
        let (top, right, bottom, left) = self.margins;
        let w = (x - left - right);
        let h = (y - bottom - top);
        (w, h)
    }

    /// Sets the origin right at the middle of the transform
    pub fn reset_origin(&mut self) {
        let (top, right, bottom, left) = self.margins;
        let (x, y) = self.get_self_size();
        let ox = left + x/2;
        let oy = top + y/2;
        self.origin = (ox, oy);
    }

    pub fn set_screen_size(&mut self, x: i32, y: i32) {
        self.screen_size = (x, y);
        if !self.initialized {
            self.initialized = true;
            self.reset_origin();
        }
    }

    pub fn set_margin_top(&mut self, m: i32) {
        self.margins.0 = m;
    }

    pub fn set_margin_right(&mut self, m: i32) {
        self.margins.1 = m;
    }

    pub fn set_margin_down(&mut self, m: i32) {
        self.margins.2 = m;
    }

    pub fn set_margin_left(&mut self, m: i32) {
        self.margins.3 = m;
    }

    pub fn set_scale_x(&mut self, m: f64) {
        self.scale_x = m;
    }

    pub fn set_scale_y(&mut self, m: f64) {
        self.scale_y = m;
    }

    /// Transforms screen_x and screen_y into local_coordinates
    pub fn world_to_local(&self, x: i32, y: i32) -> (f64, f64) {
        // Satisfies x = origin + a * scale_x;
        let a = (x - self.origin.0) as f64/self.scale_x;
        let b = (y - self.origin.1) as f64/self.scale_y;
        (a, b)
    }

    /// Transforms local x and y to screen_x, screen_y
    pub fn local_to_world(&self, a: f64, b: f64) -> (i32, i32) {
        let x = self.origin.0 as f64 + a * self.scale_x;
        let y = self.origin.1 as f64 + a * self.scale_y;

        (x.round() as i32, y.round() as i32)
    }

    /// Transforms local x and y to client coordinates (render coordinates)
    pub fn local_to_client(&self, a: f64, b: f64) -> (f64, f64) {
        let (top, right, bottom, left) = self.margins;
        let x = self.origin.0 as f64 + a * self.scale_x - top as f64;
        let y = self.origin.1 as f64 + a * self.scale_y - left as f64;

        (x, y)
    }
}
