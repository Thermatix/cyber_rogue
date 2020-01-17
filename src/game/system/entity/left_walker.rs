use super::super::*;
use crate::game::entity::{
    LeftMover, Motion,
    Motions::{Left, Right},
    Position,
};

impl<'a> System<'a> for LeftWalker {
    type SystemData = (
        ReadStorage<'a, LeftMover>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Motion>,
    );
    fn run(&mut self, (lefty, positions, mut moves): Self::SystemData) {
        for (_lefty, pos, motion) in (&lefty, &positions, &mut moves).join() {
            if pos.x == 0 {
                motion.motions.push(Right(79));
            } else {
                motion.motions.push(Left(1));
            }
        }
    }
}

impl LeftWalker {
    pub fn new() -> Self {
        Self {}
    }
}
