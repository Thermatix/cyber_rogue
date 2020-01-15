#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

mod prelude;
use game::entity::components::*;
use prelude::*;

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Hello Rust World", "resources");
    let mut game_state = sys::State::new();
    game_state
        .ecs
        .create_entity()
        .with(Position::new(40, 25))
        .with(Renderable::new(
            rltk::to_cp437('@'),
            rltk::YELLOW,
            rltk::BLACK,
        ))
        .build();

    for i in 0..10 {
        game_state
            .ecs
            .create_entity()
            .with(Position::new(i * 7, 20))
            .with(Renderable::new(rltk::to_cp437('☺'), rltk::RED, rltk::BLACK))
            .with(LeftMover {})
            .build();
    }
    rltk::main_loop(context, game_state);
}
