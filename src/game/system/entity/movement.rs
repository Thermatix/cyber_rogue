use super::super::*;

use super::Movement;
use crate::game::entity::{components::EventValue, EventStream, Position};
use crate::sys::element::{MapList, TileSetList};
use specs::world::EntitiesRes;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteExpect<'a, MapList>,
        WriteStorage<'a, EventStream>,
        WriteStorage<'a, Position>,
        Read<'a, EntitiesRes>,
    );

    fn run(&mut self, (mut maps, mut events, mut pos, entities): Self::SystemData) {
        if let Some(map) = &mut maps.find_mut("Test Map") {
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

                            if !map.blocking(idx) {
                                map.move_entity(
                                    pos.x as usize,
                                    pos.y as usize,
                                    x as usize,
                                    y as usize,
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
