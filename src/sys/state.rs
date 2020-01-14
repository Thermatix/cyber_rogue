use crate::game::entity;
use specs::prelude::*;

struct State {
    ecs: World,
}

impl State {
    fn new() -> Self {
        let mut s = Self { ecs: World::new() };
        entity::register_components(&mut s.ecs);
        s
    }
}
