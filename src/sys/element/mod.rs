use crate::game::entity::Renderable;

use std::collections::HashMap;

// structs
mod map;
mod map_list;
mod tile;
mod tile_set;
mod tile_set_list;

// traits
mod map_generator;

pub use map_generator::MapGenerator;

pub struct Tile {
    pub blocking: bool,
    pub name: String,
    pub visual: Renderable,
}

pub struct TileSet {
    pub list: HashMap<String, Tile>,
}

pub struct TileSetList {
    pub tilesets: HashMap<String, TileSet>,
}
pub struct Map {
    pub name: String,
    pub tileset: String,
    pub tiles: Vec<String>,
    pub blocking: Vec<bool>,
    pub height: usize,
    pub y: usize,
    pub width: usize,
    pub x: usize,
}

#[derive(Default)]
pub struct MapList {
    pub maps: HashMap<String, Map>,
}
