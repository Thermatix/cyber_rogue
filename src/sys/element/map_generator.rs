//! This trait represents a map generator, contains built in helper methods
use super::Map;

use crate::sys::widgets::map::Rect;

pub trait MapGenerator {
    /// Generates a map for a given map struct
    /// Accepts the following args:
    /// `map: &mut Map`
    fn create_map<'m>(&'m self, map: &'m mut Map);

    /// Adds a rectangle of a given tile to the map
    /// Accepts the following args:
    /// `map: &mut Map, tile: &str, rect: &Rect`
    fn apply_rect_to_map(&self, map: &mut Map, tile: &str, rect: &Rect) {
        for y in rect.top_left_y + 1..=rect.bottom_right_y {
            for x in rect.top_left_x + 1..=rect.bottom_right_x {
                map.insert_tile(&tile, x as usize, y as usize);
            }
        }
    }

    /// Adds a horizontal line
    /// Accepts the following args:
    /// `map: &mut Map, tile: &str, left_x: i32, right_x: i32, y: i32`
    fn apply_horizontal_tunnel(&self, map: &mut Map, tile: &str, left_x: i32, right_x: i32, y: i32) {
        for x in left_x..=right_x {
            if map.within_map(x as usize, y as usize) {
                map.insert_tile(tile, x as usize, y as usize);
            }
        }
    }

    /// Adds a verticle line
    /// Accepts the following args:
    /// `map: &mut Map, tile: &str, top_y: i32, bottom_y: i32, x: i32`
    fn apply_vertical_tunnel(&self, map: &mut Map, tile: &str, top_y: i32, bottom_y: i32, x: i32) {
        for y in top_y..=bottom_y {
            if map.within_map(x as usize, y as usize) {
                map.insert_tile(tile, x as usize, y as usize);
            }
        }
    }
}
