use super::{Map, MapGenerator, Tile, TileSet};

impl<'m> Map<'m> {
    pub fn new(
        name: &'m str,
        tile_set_name: &'m str,
        x: usize,
        y: usize,
        initial_tile: &'m str,
    ) -> Self {
        Self {
            name: name,
            tileset: tile_set_name,
            tiles: vec![initial_tile; x * y],
            blocking: vec![false; x * y],
            width: x,
            x: x - 1,
            height: y,
            y: y - 1,
        }
    }

    pub fn insert_tile(&mut self, tile: &'m str, x: usize, y: usize) {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = tile;
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    pub fn generate(&mut self, gen: impl MapGenerator) {
        gen.create_map(self);
    }
}
