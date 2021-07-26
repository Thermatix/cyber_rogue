//! For maps that that have fog, shadow, etc will keep a record of visible tiles
use specs::prelude::*;

use super::{FieldOfView, Position};

impl FieldOfView {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: Vec::new(),
            range: range,
            dirty: true,
        }
    }
}
