use crate::game::entity::Renderable;
use indexmap::IndexMap;

use std::collections::HashMap;

// structs
mod map;
mod map_list;
mod tile;
mod tile_set;
mod tile_set_list;

// traits
mod containers;
mod map_generator;

pub use containers::Container;
pub use map_generator::MapGenerator;

#[derive(Clone)]
pub struct Tile {
    pub blocking: bool,
    pub name: String,
    pub visual: Renderable,
}

#[derive(Clone)]
pub struct TileSet {
    pub list: IndexMap<String, Tile>,
}

pub struct TileSetList {
    // TODO: change this to tile_sets
    pub tilesets: HashMap<String, TileSet>,
}

pub struct Map {
    pub name: String,
    pub tile_set: TileSet,
    pub tiles: Vec<usize>,
    pub entities: HashMap<usize, HashMap<usize, Vec<u32>>>,
    pub entrances: Vec<(i32, i32)>,
    pub width: usize,
    pub x: usize,
    pub height: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct MapList {
    pub maps: HashMap<String, Map>,
}
