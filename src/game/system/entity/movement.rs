use super::super::*;

// TODO: Update this to be generic movement system
use super::Movement;
use crate::game::entity::{components::event_stream::EventValue, EventStream, Position};
use crate::sys::element::{MapList, Tile, TileSetList};

impl<'a> System<'a> for Movement {
    type SystemData = (
        ReadExpect<'a, MapList>,
        ReadExpect<'a, TileSetList>,
        WriteStorage<'a, EventStream>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (maps, tile_list, mut events, mut pos): Self::SystemData) {
        let map = &maps.find("Test Map");
        let tile_set = &tile_list.find(&map.tileset);
        for (event_stream, mut pos) in (&mut events, &mut pos).join() {
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

                        let tile = &tile_set.find(&map.tiles[map.xy_idx(x as usize, y as usize)]);

                        if !tile.blocking {
                            pos.x = x;
                            pos.y = y
                        }
                    }
                }
                None => (),
            };
        }
    }
}

impl Movement {
    pub fn new() -> Self {
        Self {}
    }
}
