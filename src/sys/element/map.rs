use super::{Map, MapGenerator, Tile, TileSet};

impl<'m> Map<'m> {
    pub fn new(x: usize, y: usize, initial_tile: &'m Tile) -> Self {
        Self {
            tiles: vec![initial_tile.clone(); x * y],
            blocking: vec![false; x * y],
            height: x,
            width: y,
        }
    }

    pub fn insert_tile(&mut self, tile: &'m Tile, x: usize, y: usize) {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = tile.clone();
        self.blocking[idx] = tile.blocking.clone();
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn generate(&mut self, gen: impl MapGenerator, tile_set: &'m TileSet) {
        gen.create_map(self, tile_set);
    }
}
