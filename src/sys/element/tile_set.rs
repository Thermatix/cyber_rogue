use super::{HashMap, Tile, TileSet};
use crate::game::entity::{GlyphType, Renderable}; //TODO: once file based tilesets are implimented, this line can be removed

impl TileSet {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    pub fn find(&self, name: &str) -> &Tile {
        &self.list[name]
    }

    pub fn insert_tile(&mut self, tile: Tile) {
        self.list.insert(tile.name.clone(), tile);
    }

    // Todo: load tile data from data files
    pub fn load(&mut self) {
        use GlyphType::*;
        self.insert_tile(Tile {
            blocking: true,
            name: "wall".to_owned(),
            visual: Renderable::new(vec!['▓'], Static, rltk::GREY10, rltk::GREY30),
        });

        self.insert_tile(Tile {
            blocking: false,
            name: "floor".to_owned(),
            visual: Renderable::new(vec!['░'], Static, rltk::DARK_BLUE, rltk::BLACK),
        });
    }
}
