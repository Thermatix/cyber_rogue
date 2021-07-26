//! Represents a list of tilesets

use super::{HashMap, TileSet, TileSetList};

impl TileSetList {
    /// Returns a new tileset
    pub fn new() -> Self {
        Self {
            tilesets: HashMap::new(),
        }
    }

    /// Inserts a tileset
    /// Accepts the following:
    /// `name: &str`, tileset: TileSet
    pub fn insert(&mut self, name: &str, tileset: TileSet) {
        self.tilesets.insert(name.to_owned(), tileset);
    }

    /// Returns a tileset for the given name
    /// Accepts the following:
    /// `name: &str`
    /// returns: `&TileSet`
    pub fn find(&self, name: &str) -> &TileSet {
        &self.tilesets[name]
    }
}
