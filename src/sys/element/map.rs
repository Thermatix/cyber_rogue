use super::{HashMap, IndexMap, Map, MapGenerator, Tile, TileSet};
use specs::world::EntitiesRes;

impl Map {
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

    pub fn t(&self, x: usize, y: usize) -> &Tile {
        self.tile_set.find(&self.tiles[self.xy_idx(x, y)])
    }

    pub fn insert_tile(&mut self, tile: &str, x: usize, y: usize) {
        let xy_idx = self.xy_idx(x, y);
        self.tiles[xy_idx] = self.tile_set.tile_index(tile);
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    pub fn idx_xy(&self, idx: usize) -> (usize, usize) {
        (idx.rem_euclid(self.width), idx / self.width)
    }

    pub fn generate(&mut self, gen: &impl MapGenerator) {
        gen.create_map(self);
    }

    pub fn move_entity(&mut self, ox: usize, oy: usize, nx: usize, ny: usize, ent: u32) {
        let oe = &mut self.entities.get_mut(&ox).unwrap().get_mut(&oy).unwrap();
        match oe.binary_search(&ent) {
            Ok(removal_index) => {
                oe.remove(removal_index);
                if oe.is_empty() {
                    self.entities.remove(&ox);
                }
            }
            Err(_) => {}
        };
        self.insert_entity(ent, nx, ny);
    }

    pub fn blocking(&self, xy_idx: usize) -> bool {
        self.tile_set.find(&self.tiles[xy_idx]).blocking
    }

    pub fn blockingx_y(&self, x: usize, y: usize) -> bool {
        self.t(x, y).blocking
    }

    pub fn insert_entity(&mut self, ent: u32, x: usize, y: usize) {
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

        entities.push(ent);
    }

    pub fn within_map(&self, x: usize, y: usize) -> bool {
        let idx = self.xy_idx(x, y);
        idx > 0 as usize && idx < (self.tiles.len() as usize)
    }
}

impl rltk::Algorithm2D for Map {
    fn dimensions(&self) -> rltk::Point {
        rltk::Point::new(self.width, self.height)
    }
}

impl rltk::BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.blocking(idx)
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
