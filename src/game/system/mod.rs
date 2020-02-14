use specs::prelude::*;
use specs::Dispatcher;

pub mod entity;

pub fn build_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with(entity::LeftWalker, "left_walker", &[])
        .with(entity::Movement, "movement", &[])
        .with(entity::Visibility, "field_of_view", &["movement"])
        .build()
}
