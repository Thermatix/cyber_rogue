use specs::WorldExt;

pub mod components;
pub use components::*;

pub fn register_components(ecs: &mut impl WorldExt) {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<LeftMover>();
    ecs.register::<Player>();
    ecs.register::<Motion>();
}
