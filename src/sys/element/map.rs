use super::{Map, MapGenerator, Tile, TileSet};

impl Map {
    pub fn new(name: &str, tile_set_name: &str, x: usize, y: usize, initial_tile: &str) -> Self {
        Self {
            name: name.to_owned(),
            tileset: tile_set_name.to_owned(),
            tiles: vec![initial_tile.to_owned(); x * y],
            blocking: vec![false; x * y],
            width: x,
            x: x - 1,
            height: y,
            y: y - 1,
        }
    }

    pub fn insert_tile(&mut self, tile: &str, x: usize, y: usize) {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = tile.to_owned();
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    pub fn generate(&mut self, gen: impl MapGenerator) {
        gen.create_map(self);
    }
}
