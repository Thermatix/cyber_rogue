//! This struct represents a single map
//! This struct stores information about a single map, such as:
//!
//! + Attributes:
//!   - name
//!   - width
//!   - height
//!   - x
//!   - y
//! + tile set
//! + a 1D (length = x * y) Vec representing all the map tiles
//! + functions to make manipulating the map easier

use super::{HashMap, IndexMap, Map, MapGenerator, Tile, TileSet};
use specs::world::EntitiesRes;

type EntityId = u32;

impl Map {
    /// Returns a new map
    /// Accepts the following args:
    /// name: &str, tile_set: TileSet, x: usize, y: usize, initial_tile: &str
    pub fn new(name: &str, tile_set: TileSet, x: usize, y: usize, initial_tile: &str) -> Self {
        let tile = &tile_set.tile_index(initial_tile);
        Self {
            name: name.to_owned(),
            tile_set: tile_set,
            tiles: vec![*tile; x * y],
            entities: HashMap::new(),
            entrances: Vec::new(),
            width: x,
            x: x - 1,
            height: y,
            y: y - 1,
        }
    }

    /// Returns a a tile at a given x,y pos
    /// Accepts the following args:
    /// `x: usize, y: usize`
    /// Returns: `&Tile`
    pub fn t(&self, x: usize, y: usize) -> &Tile {
        self.tile_set.find(&self.tiles[self.xy_idx(x, y)])
    }

    /// Inserts a tile at the given x,y position
    /// Accepts the following args:
    /// `tile: &str, x: usize, y: usize`
    pub fn insert_tile(&mut self, tile: &str, x: usize, y: usize) {
        let xy_idx = self.xy_idx(x, y);
        self.tiles[xy_idx] = self.tile_set.tile_index(tile);
    }

    /// Transforms a given x,y into the index of the tile buffer
    /// Accepts the following args:
    /// `x: usize, y: usize`
    /// Returns: `usize`
    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    /// Transforms a given tile buffer index into x,y
    /// Accepts the following args:
    /// `idx: usize`
    /// Returns: `(x: usize, y: usize)`
    pub fn idx_xy(&self, idx: usize) -> (usize, usize) {
        (idx.rem_euclid(self.width), idx / self.width)
    }

    /// Generates a map with a given generator
    /// Accepts the following args:
    /// `gen: &impl MapGenerator`
    pub fn generate(&mut self, gen: &impl MapGenerator) {
        gen.create_map(self);
    }

    /// Moves an entity
    /// Accepts the following args:
    /// `original_X: usize, original_y: usize, new_x: usize, new_y: usize, entity: EntityId`
    pub fn move_entity(&mut self, original_x: usize, original_y: usize, new_x: usize, new_y: usize, entity: EntityId) {
        let oe = &mut self.entities.get_mut(&original_x).unwrap().get_mut(&original_y).unwrap();
        match oe.binary_search(&entity) {
            Ok(removal_index) => {
                oe.remove(removal_index);
                if oe.is_empty() {
                    self.entities.remove(&original_x);
                }
            }
            Err(_) => {}
        };
        self.insert_entity(entity, new_x, new_y);
    }

    /// Checks if tile at the given tile buffer index
    /// Accepts the following args:
    /// `xy_idx: usize`
    pub fn blocking(&self, xy_idx: usize) -> bool {
        self.tile_set.find(&self.tiles[xy_idx]).blocking
    }

    /// Checks if tile at the given x,y
    /// Accepts the following args:
    /// `x: usize, y: usize`
    /// Returns: `bool`
    pub fn blockingx_y(&self, x: usize, y: usize) -> bool {
        self.t(x, y).blocking
    }

    /// Insert entity at the given x,y
    /// Accepts the following args:
    /// `entity: EntityId`
    pub fn insert_entity(&mut self, entity: EntityId, x: usize, y: usize) {
        let row = match self.entities.get_mut(&x) {
            Some(row) => row,
            None => {
                self.entities.insert(x, HashMap::new());
                self.entities.get_mut(&x).unwrap()
            }
        };

        let entities = match row.get_mut(&y) {
            Some(row) => row,
            None => {
                row.insert(y, Vec::new());
                row.get_mut(&y).unwrap()
            }
        };

        entities.push(entity);
    }

    /// Checks if a given x,y is within the map
    /// Accepts the following args:
    /// `x: usize, y: usize`
    /// Returns: `bool`
    pub fn within_map(&self, x: usize, y: usize) -> bool {
        let idx = self.xy_idx(x, y);
        idx > 0 as usize && idx < (self.tiles.len() as usize)
    }
}

impl rltk::Algorithm2D for Map {
    /// Returns the dimentions of the map in rltk formating
    /// Returns: `rltk::Point`
    fn dimensions(&self) -> rltk::Point {
        rltk::Point::new(self.width, self.height)
    }
}

impl rltk::BaseMap for Map {
    /// For a given idx, checks if the tile is blocking
    fn is_opaque(&self, idx: usize) -> bool {
        self.blocking(idx)
    }
}

/// I can't recall what this is for...
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
