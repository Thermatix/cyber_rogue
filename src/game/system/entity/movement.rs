use super::super::*;

use super::Movement;
use crate::game::entity::{components::event_stream::EventValue, EventStream, Position};
use crate::sys::element::{MapList, Tile, TileSetList};
use specs::world::EntitiesRes;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteExpect<'a, MapList>,
        ReadExpect<'a, TileSetList>,
        WriteStorage<'a, EventStream>,
        WriteStorage<'a, Position>,
        Read<'a, EntitiesRes>,
    );

    fn run(&mut self, (mut maps, tile_list, mut events, mut pos, entities): Self::SystemData) {
        if let Some(map) = &mut maps.find_mut("Test Map") {
            let tile_set = &tile_list.find(&map.tileset);
            for (event_stream, mut pos, entity) in (&mut events, &mut pos, &*entities).join() {
                match event_stream.get_channel("motions") {
                    Some(motions) => {
                        for motion in motions.drain(0..) {
                            let amount: i32 = motion.value.into();
                            let (x, y) = {
                                if motion.message == "x" {
                                    (pos.x + amount, pos.y)
                                } else {
                                    (pos.x, pos.y + amount)
                                }
                            };

                            let idx = map.xy_idx(x as usize, y as usize);

                            let tile = &tile_set.find(map.tiles[idx].t());

                            if !tile.blocking {
                                map.move_entity(
                                    map.xy_idx(pos.x as usize, pos.y as usize),
                                    idx,
                                    entity.id(),
                                );
                                pos.x = x;
                                pos.y = y
                            }
                        }
                    }
                    None => (),
                };
            }
        } else {
        }
    }
}

impl Movement {
    pub fn new() -> Self {
        Self {}
    }
}
