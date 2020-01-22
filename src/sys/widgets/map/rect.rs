use super::Rect;

use crate::sys::element::Container;

impl Rect {
    pub fn new(o_x: i32, o_y: i32, w: i32, h: i32) -> Self {
        Self {
            width: w,
            height: h,
            tlx: o_x,
            tly: o_y,
            brx: o_x + w,
            bry: o_y + h,
        }
    }

    pub fn intersect(&self, other: &Self) -> bool {
        self.tlx <= other.brx
            && self.brx >= other.tlx
            && self.tly <= other.bry
            && self.bry >= other.tly
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.tlx + self.brx) / 2, (self.tly + self.bry) / 2)
    }
}

impl Container for Rect {}
