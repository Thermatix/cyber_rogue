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

#[derive(PartialEq, Debug)]
pub struct Tile {
    pub blocking: bool,
    pub name: &'static str,
    pub visual: Renderable,
}

pub struct TileSet {
    pub list: HashMap<&'static str, Tile>,
}

pub struct TileSetList {
    pub tilesets: HashMap<&'static str, TileSet>,
}
pub struct Map<'m> {
    pub name: &'m str,
    pub tileset: &'m str,
    pub tiles: Vec<&'m str>,
    pub blocking: Vec<bool>,
    pub height: usize,
    pub y: usize,
    pub width: usize,
    pub x: usize,
}

pub struct MapList<'ml> {
    pub maps: HashMap<&'ml str, Map<'ml>>,
}
