use specs::world::World;
use specs::WorldExt;

pub mod components;
pub use components::*;

pub fn register_components<W>(mut world: W) -> W
where
    W: WorldExt,
{
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<LeftMover>();
    world.register::<Player>();
    world.register::<EventStream>();
    world.register::<FieldOfView>();
    world
}
