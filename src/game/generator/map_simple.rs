use super::MapSimple;
use crate::sys::element::*;

impl MapGenerator for MapSimple {
    fn create_map(&self, map: &mut Map) {
        for x in 0..map.width {
            map.insert_tile("wall", x, 0);
            map.insert_tile("wall", x, map.y);
        }

        for y in 0..map.height {
            map.insert_tile("wall", 0, y);
            map.insert_tile("wall", map.x, y);
        }

        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..(map.tiles.len() as f32 * 0.10) as i32 {
            let x = rng.roll_dice(1, (map.x) as i32);
            let y = rng.roll_dice(1, (map.y) as i32);
            if x != 40 || y != 25 {
                map.insert_tile("wall", x as usize, y as usize);
            }
        }
    }
}
