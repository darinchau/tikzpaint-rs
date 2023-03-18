//! This module serves as means to translate between coordinate systems.
//! There are currently 3 in total:
//! - screen coordinates (e.g. mouse click)
//! - local coordinates (e.g. figure)
//! - client coordiantes (e.g. canvas)

use std::cell::RefCell;
use std::rc::Rc;
use gloo::console::log;

use crate::figures::*;

#[derive(PartialEq, Clone, Copy)]
pub struct Transform {
    /// Screen size according to inner_width and inner_height
    pub screen_size: (i32, i32),

    /// scale x is the number such that 1 unit in the coordinate system horizontally = scale_x pixels
    pub scale: f64,

    /// Where the origin ought to be in world coordinates. Coordinates can be very big or negative - means the origin is out of sight
    pub origin: (i32, i32),

    pub margins: (i32, i32, i32, i32),
}

impl Transform {
    /// Abuse of notation
    pub fn new(header_height: usize, sidebar_width: usize, terminal_height: usize) -> Self {
        let h = header_height as i32;
        let w = sidebar_width as i32;
        let th = terminal_height as i32;

        Self {
            screen_size: (1920, 1080),
            scale: 100.,
            origin: (0, 0),
            margins: (h, 0, th, w),
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
        log!(format!("Setting origin to {}, {}", ox, oy));
    }

    pub fn set_screen_size(&mut self, x: i32, y: i32) {
        self.screen_size = (x, y);
        self.reset_origin();

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

    pub fn set_scale(&mut self, m: f64) {
        self.scale = m;
    }

    /// Transforms screen_x and screen_y into local_coordinates
    pub fn world_to_local(&self, x: i32, y: i32) -> Coordinates {
        // Satisfies x = origin + a * scale_x;
        let a = (x - self.origin.0) as f64/self.scale;
        let b = (self.origin.1 - y) as f64/self.scale;
        Coordinates::new(a, b)
    }

    #[inline(always)]
    fn ltw(&self, v: Coordinates) -> (f64, f64) {
        let x = self.origin.0 as f64 + v[0] * self.scale;
        let y = self.origin.1 as f64 - v[1] * self.scale;
        (x, y)
    }

    /// Transforms local x and y to screen_x, screen_y
    pub fn local_to_world(&self, v: Coordinates) -> (i32, i32) {
        let (x, y) = self.ltw(v);
        (x.round() as i32, y.round() as i32)
    }

    /// Transforms local x and y to client coordinates (render coordinates)
    pub fn local_to_client(&self, v: Coordinates) -> (f64, f64) {
        let (x, y) = self.ltw(v);

        // Subtract the margins
        let (top, _, bottom, left) = self.margins;
        (x - left as f64, y - top as f64 - bottom as f64 / 2.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_1() {
        let mut tf = Transform::new(60, 190, 150);
        tf.set_screen_size(1016, 746);

        // ------- 60 ---------
        //  |                 |
        // 190                |
        //  |                 |
        //  | ---- 150 --------

        // x: 1016 - 190 = 826
        // y: 746 - 60 - 150 = 536

        // origin position on screen:
        // x: 190 + 826/2
        // y: 60 + 536/2

        assert_eq!(tf.origin, (603, 328));
        assert_eq!(tf.local_to_world(Coordinates::new(0., 0.)), (603, 328));
    }

    #[test]
    fn test_coord_1() {
        let mut tf = Transform::new(60, 190, 150);
        tf.set_screen_size(1016, 746);
        tf.set_scale(100.);

        // ------- 60 ---------
        //  |                 |
        // 190                |
        //  |                 |
        //  | ---- 150 --------

        // x: 1016 - 190 = 826
        // y: 746 - 60 - 150 = 536

        // Position of (0, 1) on screen: origin + 1 * scale * (0, 1)

        assert_eq!(tf.local_to_world(Coordinates::new(0., 1.)), (603, 228));
        assert_eq!(tf.world_to_local(603, 228), Coordinates::new(0., 1.));

        assert_eq!(tf.local_to_world(Coordinates::new(0., 2.)), (603, 128));
        assert_eq!(tf.world_to_local(603, 128), Coordinates::new(0., 2.));

        assert_eq!(tf.local_to_world(Coordinates::new(0., -1.)), (603, 428));
        assert_eq!(tf.world_to_local(603, 428), Coordinates::new(0., -1.));
    }

    #[test]
    fn test_coord_2() {
        let mut tf = Transform::new(60, 190, 150);
        tf.set_screen_size(1016, 746);
        tf.set_scale(100.);

        assert_eq!(tf.local_to_world(Coordinates::new(1., 0.)), (703, 328));
        assert_eq!(tf.world_to_local(703, 328), Coordinates::new(1., 0.));

        assert_eq!(tf.local_to_world(Coordinates::new(2., 0.)), (803, 328));
        assert_eq!(tf.world_to_local(803, 328), Coordinates::new(2., 0.));

        assert_eq!(tf.local_to_world(Coordinates::new(-1., 0.)), (503, 328));
        assert_eq!(tf.world_to_local(503, 328), Coordinates::new(-1., 0.));
    }
}