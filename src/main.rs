#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

mod prelude;
use game::entity::components::*;
use prelude::*;

fn main() {
    use game::entity::components::GlyphType;
    let context = Rltk::init_simple8x8(80, 50, "Hello Rust World", "resources");
    let mut game_state = sys::State::new(
        &game::system::build_dispatcher,
        &game::entity::register_components,
    );
    {
        let mut tile_set = sys::element::TileSet::new();
        tile_set.load();
        let mut tile_set_list = sys::element::TileSetList::new();
        tile_set_list.insert("Test Tile Set", tile_set);

        game_state.ecs.insert(tile_set_list);
        game_state.ecs.insert(sys::element::MapList::new());
        game_state.ecs.insert(game::entity::EventStream::new());

        game_state.insert_map("Test Tile Set", |tile_set| {
            let mut map = sys::element::Map::new("Test Map", tile_set, 80, 50, "wall");
            map.generate(&game::generator::DungeonBasic {});
            map
        });
        let player_entrance = {
            let map_list = &game_state.ecs.fetch::<sys::element::MapList>();
            let map = &map_list.find("Test Map");

            map.entrances.first().unwrap().clone()
        };

        let player_entity = game_state.insert_entity(|eb| {
            eb.with(Position::new(player_entrance.0, player_entrance.1))
                .with(Renderable::new(
                    vec!['@'],
                    GlyphType::Static,
                    rltk::YELLOW,
                    rltk::BLACK,
                ))
                .with(Location::new("Test Map"))
                .with(EventStream::new())
                .with(Player::new())
                .with(FieldOfView::new(6))
        });
        {
            let map_list = &mut game_state.ecs.fetch_mut::<sys::element::MapList>();
            let map = &mut map_list.find_mut("Test Map").unwrap();
            map.insert_entity(
                player_entity,
                player_entrance.0 as usize,
                player_entrance.1 as usize,
            );
        }
    }

    rltk::main_loop(context, game_state);
}
