use super::{Map, TileSet};

pub trait MapGenerator {
    fn create_map(&self, map: &mut Map, tile_set: &TileSet);
}
