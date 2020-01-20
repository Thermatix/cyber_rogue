use super::super::*;

// TODO: Update this to be generic movement system
use super::Movement;
use crate::game::entity::{components::event_stream::EventValue, EventStream, Position};

impl<'a> System<'a> for Movement {
    type SystemData = (WriteStorage<'a, EventStream>, WriteStorage<'a, Position>);

    fn run(&mut self, (mut events, mut pos): Self::SystemData) {
        // TODO: Build FOV component/IMMIEDIATE VECINITY component and use that to determin
        // location
        for (event_stream, pos) in (&mut events, &mut pos).join() {
            match event_stream.get_channel("motions") {
                Some(motions) => {
                    let mut error_dir = "".to_owned();
                    for motion in motions.drain(0..) {
                        let amount: i32 = motion.value.into();
                        match motion.message.as_ref() {
                            "u" => pos.y -= amount,
                            "l" => pos.x -= amount,
                            "d" => pos.y += amount,
                            "r" => pos.x += amount,
                            _ => {
                                error_dir = motion.message;
                            }
                        }
                    }
                    if error_dir != "".to_owned() {
                        event_stream
                            .add_to_channel("errors", ("Not a valid movement type", error_dir));
                    };
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
