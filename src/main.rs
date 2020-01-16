#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

mod prelude;
use game::entity::components::*;
use prelude::*;

fn main() {
    use game::entity::components::GlyphType;
    let context = Rltk::init_simple8x8(80, 50, "Hello Rust World", "resources");
    let mut game_state = sys::State::new();

    let mut tile_set = sys::element::TileSet::new();
    tile_set.load();
    let mut tile_set_list = sys::element::TileSetList::new();
    tile_set_list.insert("Test Tile Set", tile_set);

    let mut map_list = sys::element::MapList::new();
    let mut map = sys::element::Map::new("Test Map", "Test Tile Set", 80, 50, "floor");
    map.generate(game::generator::MapSimple {});
    map_list.insert(map);

    game_state.ecs.insert(tile_set_list);
    game_state.ecs.insert(map_list);

    game_state
        .ecs
        .create_entity()
        .with(Position::new(40, 25))
        .with(Renderable::new(
            vec!['@'],
            GlyphType::Static,
            rltk::YELLOW,
            rltk::BLACK,
        ))
        .with(Player::new())
        .build();

    for i in 0..10 {
        game_state
            .ecs
            .create_entity()
            .with(Position::new(i * 7, 20))
            .with(Renderable::new(
                vec!['☺'],
                GlyphType::Static,
                rltk::RED,
                rltk::BLACK,
            ))
            .with(LeftMover {})
            .build();
    }
    rltk::main_loop(context, game_state);
}
