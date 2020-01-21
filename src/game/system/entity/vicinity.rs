use super::super::*;
use crate::game::entity::{EventStream, Position, SenseOfTouch};
use crate::sys::element::{MapList, TileSetList};

impl<'a> System<'a> for Vicinity {
    type SystemData = (
        ReadExpect<'a, MapList>,
        ReadExpect<'a, TileSetList>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, SenseOfTouch>,
        WriteStorage<'a, EventStream>,
    );

    fn run(&mut self, (maps, tile_list, pos, mut sot, mut events): Self::SystemData) {
        let map = &maps.find("Test Map");
        let tile_set = &tile_list.find(&map.tileset);
        for (pos, sot, events) in (&pos, &mut sot, &mut events).join() {
            match events.get_channel("motions") {
                Some(mut motions) => {
                    let mut error = "".to_owned();
                    let mut can_move = false;
                    for motion in motions.drain(0..) {
                        let amount: i32 = motion.value.into();
                        match motion.message.as_ref() {
                            "u" => {
                                if sot.can_move_to(pos.x, pos.y - amount, tile_set) {
                                    can_move = true;
                                }
                            }
                            "l" => {
                                if sot.can_move_to(pos.x - amount, pos.y, tile_set) {
                                    can_move = true;
                                }
                            }
                            "d" => {
                                if sot.can_move_to(pos.x, pos.y + amount, tile_set) {
                                    can_move = true;
                                }
                            }
                            "r" => {
                                if sot.can_move_to(pos.x + amount, pos.y, tile_set) {
                                    can_move = true;
                                }
                            }
                            _ => {
                                error = motion.message;
                            }
                        }
                        if can_move {
                            events
                                .add_to_channel("move", (motion.message.clone().as_ref(), amount));
                        } else {
                            if error != "".to_owned() {
                                events
                                    .add_to_channel("errors", ("Not a valid movement type", error));
                            }
                        }
                    }
                }

                None => (),
            }
        }
    }
}
