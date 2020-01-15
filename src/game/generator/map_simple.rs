use super::MapSimple;
use crate::sys::element::*;

impl MapGenerator for MapSimple {
    fn create_map(&self, map: &mut Map, tile_set: &TileSet) {
        let wall = &tile_set.find("wall");

        for x in 0..map.width {
            map.insert_tile(&wall, x, 0);
            map.insert_tile(&wall, x, map.width);
        }

        for y in 0..map.height {
            map.insert_tile(&wall, 0, y);
            map.insert_tile(&wall, map.height, y);
        }

        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..400 {
            let x = rng.roll_dice(1, (map.width - 1) as i32);
            let y = rng.roll_dice(1, (map.height - 1) as i32);
            if x != 40 || y != 25 {
                map.insert_tile(&wall, x as usize, y as usize);
            }
        }
    }
}
