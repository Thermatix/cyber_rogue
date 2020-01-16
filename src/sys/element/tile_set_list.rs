use super::{HashMap, TileSet, TileSetList};

impl TileSetList {
    pub fn new() -> Self {
        Self {
            tilesets: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &'static str, tileset: TileSet) {
        self.tilesets.insert(name, tileset);
    }

    pub fn find(&self, name: &str) -> &TileSet {
        &self.tilesets[name]
    }
}
