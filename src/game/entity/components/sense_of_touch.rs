use super::{SenseOfTouch, Tile, TileSet};

impl SenseOfTouch {
    pub fn new() -> Self {
        Self {
            vicinity: Vec::new(),
        }
    }

    pub fn can_move_to(&self, x: i32, y: i32, tile_set: &TileSet) -> bool {
        (tile_set.find(self.vicinity[SenseOfTouch::xy_idx(x, y)].as_ref())).blocking
    }

    fn xy_idx(x: i32, y: i32) -> usize {
        (x + (y * 3)) as usize
    }
}
