use super::super::*;

use super::PlayerMovement;
use crate::game::entity::{player::Movements, Player, Position};

impl<'a> System<'a> for PlayerMovement {
    type SystemData = (WriteStorage<'a, Player>, WriteStorage<'a, Position>);
    fn run(&mut self, (mut player, mut pos): Self::SystemData) {
        use Movements::*;
        for (player, pos) in (&mut player, &mut pos).join() {
            for movement in player.movements.drain(0..) {
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

impl PlayerMovement {
    pub fn new() -> Self {
        Self {}
    }
}
