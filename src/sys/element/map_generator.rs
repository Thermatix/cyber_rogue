use super::Map;

use crate::sys::widgets::map::Rect;

pub trait MapGenerator {
    fn create_map(&self, map: &mut Map);

    fn apply_rect_to_map(&self, map: &mut Map, tile: &str, rect: &Rect) {
        for y in rect.tly + 1..=rect.bry {
            for x in rect.tlx + 1..=rect.brx {
                map.insert_tile(&tile, x as usize, y as usize);
            }
        }
    }

    fn apply_horizontal_tunnel(&self, map: &mut Map, tile: &str, lx: i32, rx: i32, y: i32) {
        for x in lx..=rx {
            let idx = map.xy_idx(x as usize, y as usize);
            if self.within_map(idx, map) {
                map.tiles[idx].tile = tile.to_owned();
            }
        }
    }

    fn apply_vertical_tunnel(&self, map: &mut Map, tile: &str, ly: i32, ry: i32, x: i32) {
        for y in ly..=ry {
            let idx = map.xy_idx(x as usize, y as usize);
            if self.within_map(idx, map) {
                map.tiles[idx].tile = tile.to_owned();
            }
        }
    }

    fn within_map(&self, idx: usize, map: &Map) -> bool {
        idx > 0 as usize && idx < (map.tiles.len() as usize)
    }
}
