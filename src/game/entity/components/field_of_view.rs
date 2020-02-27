use specs::prelude::*;

use super::{FieldOfView, Position};

impl FieldOfView {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: Vec::new(),
            range: range,
        }
    }
}
