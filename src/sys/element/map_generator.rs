use super::Map;

use crate::sys::widgets::map::Rect;

pub trait MapGenerator {
    fn create_map<'m>(&'m self, map: &'m mut Map);

    fn apply_rect_to_map(&self, map: &mut Map, tile: &str, rect: &Rect) {
        for y in rect.tly + 1..=rect.bry {
            for x in rect.tlx + 1..=rect.brx {
                map.insert_tile(&tile, x as usize, y as usize);
            }
        }
    }

    fn apply_horizontal_tunnel(&self, map: &mut Map, tile: &str, lx: i32, rx: i32, y: i32) {
        for x in lx..=rx {
            if map.within_map(x as usize, y as usize) {
                map.insert_tile(tile, x as usize, y as usize);
            }
        }
    }

    fn apply_vertical_tunnel(&self, map: &mut Map, tile: &str, ly: i32, ry: i32, x: i32) {
        for y in ly..=ry {
            if map.within_map(x as usize, y as usize) {
                map.insert_tile(tile, x as usize, y as usize);
            }
        }
    }
}
