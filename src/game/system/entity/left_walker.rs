use super::super::*;

use super::LeftWalker;
use crate::game::entity::{LeftMover, Position};

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);
    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

impl LeftWalker {
    pub fn new() -> Self {
        Self {}
    }
}
