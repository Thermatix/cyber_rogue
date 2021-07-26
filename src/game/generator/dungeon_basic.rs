//! This is an example map generator, it generates a super basic map layout
//! of rooms and corridors
use super::element::*;

use super::DungeonBasic;

use crate::sys::widgets::map::Rect;

impl MapGenerator for DungeonBasic {
    fn create_map<'m>(&'m self, mut map: &'m mut Map) {
        let mut rooms: Vec<Rect> = Vec::new();
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = rltk::RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.x as i32 - w - 1);
            let y = rng.roll_dice(1, (map.y) as i32 - w - 2);
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;

            rooms.iter().any(|other_room| {
                if new_room.intersect(&other_room) {
                    ok = false;
                    true
                } else {
                    false
                }
            });

            if ok {
                self.apply_rect_to_map(&mut map, "floor", &new_room);

                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        self.apply_horizontal_tunnel(&mut map, "floor", prev_x, new_x, prev_y);
                        self.apply_vertical_tunnel(&mut map, "floor", prev_y, new_y, new_x);
                    } else {
                        self.apply_vertical_tunnel(&mut map, "floor", prev_y, new_y, prev_x);
                        self.apply_horizontal_tunnel(&mut map, "floor", prev_x, new_x, new_y);
                    }
                } else {
                    map.entrances.push(new_room.center());
                }

                rooms.push(new_room);
            }
        }
    }
}
