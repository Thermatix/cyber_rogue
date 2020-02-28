use super::super::*;

use super::Movement;
use crate::game::entity::{components::EventValue, EventStream, Player};
use crate::sys::element::{MapList, TileSetList};
use specs::world::EntitiesRes;

impl<'a> System<'a> for Movement {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, EventStream>,
        WriteExpect<'a, EventStream>,
    );
    fn run(&mut self, (player, mut entity_events, mut global_events): Self::SystemData) {
        for (_, entity_event) in (&player, &mut entity_events).join() {
            match global_events.get_channel("key_press") {
                Some(key_presses) => {
                    for key_press in key_presses.drain(0..) {
                        let k: i32 = key_press.value.into();
                        match k {
                            0 => entity_event.add_to_channel("motions", ("up", (0, -1))),
                            1 => entity_event.add_to_channel("motions", ("up", (-1, 0))),
                            2 => entity_event.add_to_channel("motions", ("up", (0, 1))),
                            3 => entity_event.add_to_channel("motions", ("up", (1, 0))),
                            _ => {}
                        };
                    }
                }
                _ => {}
            };
        }
    }
}
