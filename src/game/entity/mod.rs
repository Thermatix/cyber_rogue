use specs::WorldExt;

pub mod components;
use components::*;

pub fn register_components(ecs: &mut impl WorldExt) {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
}
