use super::{Map, MapGenerator, Tile, TileSet};
use specs::world::EntitiesRes;

impl Map {
    pub fn new(name: &str, tile_set_name: &str, x: usize, y: usize, initial_tile: &str) -> Self {
        Self {
            name: name.to_owned(),
            tileset: tile_set_name.to_owned(),
            tiles: vec![MapLoc::new(initial_tile.to_owned()); x * y],
            entrances: Vec::new(),
            blocking: vec![false; x * y],
            width: x,
            x: x - 1,
            height: y,
            y: y - 1,
        }
    }

    pub fn insert_tile(&mut self, tile: &str, x: usize, y: usize) {
        let idx = self.xy_idx(x, y);
        self.tiles[idx].tile = tile.to_owned();
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    pub fn generate(&mut self, gen: impl MapGenerator) {
        gen.create_map(self);
    }

    pub fn move_entity(&mut self, old_idx: usize, new_idx: usize, ent: u32) {
        match self.tiles[old_idx].entities.binary_search(&ent) {
            Ok(removal_index) => {
                self.tiles[old_idx].entities.remove(removal_index);
            }
            Err(_) => {}
        }
        self.tiles[new_idx].entities.push(ent);
    }

    pub fn insert_entity(&mut self, ent: u32, x: i32, y: i32) {
        let idx = self.xy_idx(x as usize, y as usize);
        self.tiles[idx].entities.push(ent);
    }
}

#[derive(Clone)]
pub struct MapLoc {
    pub tile: String,
    pub entities: Vec<u32>,
}

impl MapLoc {
    pub fn new(tile: String) -> Self {
        Self {
            tile: tile,
            entities: Vec::new(),
        }
    }

    pub fn t(&self) -> &String {
        &self.tile
    }
}
