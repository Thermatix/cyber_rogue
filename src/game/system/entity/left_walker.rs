//! This is an example system that simply moves an entity westward
use super::super::*;
use super::LeftWalker;
use crate::game::entity::{EventStream, LeftMover, Position};

impl<'a> System<'a> for LeftWalker {
    type SystemData = (
        ReadStorage<'a, LeftMover>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, EventStream>,
    );
    fn run(&mut self, (lefty, positions, mut events): Self::SystemData) {
        for (_lefty, pos, events) in (&lefty, &positions, &mut events).join() {
            if pos.x == 0 {
                events.add_to_channel("motions", ("x", 79));
            } else {
                events.add_to_channel("motions", ("x", -1));
            }
        }
    }
}

impl LeftWalker {
    pub fn new() -> Self {
        Self {}
    }
}
