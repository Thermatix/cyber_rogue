use specs::prelude::*;
use specs::Dispatcher;

pub mod entity;
pub mod player;

pub fn build_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with(entity::LeftWalker, "left_walker", &[])
        .with(player::Movement, "p_movement", &[])
        .with(entity::Movement, "e_movement", &["p_movement"])
        .with(entity::Visibility, "field_of_view", &["e_movement"])
        .build()
}
