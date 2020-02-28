use super::super::*;

use super::Movement;
use crate::game::entity::{components::EventValue, EventStream, FieldOfView, Position};
use crate::sys::element::{MapList, TileSetList};
use specs::world::EntitiesRes;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteExpect<'a, MapList>,
        WriteStorage<'a, EventStream>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Position>,
        Read<'a, EntitiesRes>,
    );

    fn run(&mut self, (mut maps, mut events, mut fov, mut pos, entities): Self::SystemData) {
        if let Some(map) = &mut maps.find_mut("Test Map") {
            for (event_stream, fov, mut pos, entity) in
                (&mut events, &mut fov, &mut pos, &*entities).join()
            {
                match event_stream.get_channel("motions") {
                    Some(motions) => {
                        for motion in motions.drain(0..) {
                            let point: (i32, i32) = motion.value.into();
                            let (x, y) = pos.add(point);

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
                                pos.y = y;
                                fov.dirty = true;
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
