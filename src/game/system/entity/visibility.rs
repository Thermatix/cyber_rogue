use super::super::*;
use super::Visibility;
use crate::game::entity::{FieldOfView, Position};

use specs::prelude::*;

impl<'a> System<'a> for Visibility {
    type SystemData = (WriteStorage<'a, FieldOfView>, WriteStorage<'a, Position>);

    fn run(&mut self, (mut fov, pos): Self::SystemData) {
        for (fov, pos) in (&mut fov, &pos).join() {}
    }
}
