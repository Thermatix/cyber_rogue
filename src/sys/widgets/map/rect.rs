//! This struct is used to store dimensions for a rectangle 
use super::Rect;

use crate::sys::element::Container;

impl Rect {
    /// Returns a new rectangle
    /// Accepts the following args:
    /// `origin_x: i32, origin_y: i32, width: i32, height: i32`
    /// Returns: `Rect`
    pub fn new(origin_x: i32, origin_y: i32, w: i32, h: i32) -> Self {
        Self {
            width: w,
            height: h,
            top_left_x: origin_x,
            top_left_y: origin_y,
            bottom_right_x: origin_x + w,
            bottom_right_y: origin_y + h,
        }
    }

    /// Used to check the this rect intersects a given rect
    /// Accepts the following args:
    /// `other: Rect`
    /// returns: `bool`
    pub fn intersect(&self, other: &Self) -> bool {
        self.top_left_x <= other.bottom_right_x
            && self.bottom_right_x >= other.top_left_x
            && self.top_left_y <= other.bottom_right_y
            && self.bottom_right_y >= other.top_left_y
    }

    /// Returns the center x,y of the rect
    /// returns: `(i32, i32)`
    pub fn center(&self) -> (i32, i32) {
        ((self.top_left_x + self.bottom_right_x) / 2, (self.top_left_y + self.bottom_right_y) / 2)
    }
}

impl Container for Rect {}
