use super::{HashMap, IndexMap, Tile, TileSet};
use crate::game::entity::{GlyphType, Renderable};
use crate::sys::utils::FindBy; //TODO: once file based tilesets are implimented, this line can be removed

use std::convert::{From, Into};

impl TileSet {
    pub fn new() -> Self {
        Self {
            list: IndexMap::new(),
        }
    }

    pub fn tile_index<'r, K: Into<FindBy<'r>>>(&self, key: K) -> usize {
        match key.into() {
            FindBy::S(k) => match self.list.get_full(k) {
                Some((i, _, _)) => i,
                _ => todo!(),
            },
            FindBy::ST(k) => match self.list.get_full(k as &str) {
                Some((i, _, _)) => i,
                _ => todo!(),
            },
            _ => panic!("expected String or &str, was usize"),
        }
    }

    pub fn find<'r, 'ts: 'r, K: Into<FindBy<'r>>>(&'ts self, key: K) -> &'r Tile {
        match key.into() {
            FindBy::S(k) => &self.list.get(k).unwrap(),
            FindBy::ST(k) => &self.list.get(k).unwrap(),
            FindBy::U(k) => match &self.list.get_index(*k) {
                Some((_, v)) => &v,
                _ => todo!(),
            },
        }
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
