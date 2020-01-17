use super::super::*;

// TODO: Update this to be generic movement system
use super::Movement;
use crate::game::entity::{Motion, Motions, Position};

impl<'a> System<'a> for Movement {
    type SystemData = (WriteStorage<'a, Motion>, WriteStorage<'a, Position>);

    fn run(&mut self, (mut movement, mut pos): Self::SystemData) {
        use Motions::*;
        // TODO: Build FOV component/IMMIEDIATE VECINITY component and use that to determin
        // location
        for (movement, pos) in (&mut movement, &mut pos).join() {
            for movement in movement.motions.drain(0..) {
                match movement {
                    Up(amount) => pos.y -= amount,
                    Left(amount) => pos.x -= amount,
                    Down(amount) => pos.y += amount,
                    Right(amount) => pos.x += amount,
                }
            }
        }
    }
}

impl Movement {
    pub fn new() -> Self {
        Self {}
    }
}
