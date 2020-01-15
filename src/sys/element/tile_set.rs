use super::{Tile, TileSet};
use crate::game::entity::Renderable;

impl<'tg> TileSet<'tg> {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn find(&self, name: &'tg str) -> Tile {
        self.list[self
            .list
            .binary_search_by(|tile| tile.name.cmp(name))
            .unwrap()]
    }

    // Todo: load tile data from data files
    pub fn load(&mut self) {
        self.list.push(Tile {
            blocking: true,
            name: "wall",
            visual: Renderable::new('▓', rltk::GREY, rltk::BROWN4),
        });

        self.list.push(Tile {
            blocking: true,
            name: "floor",
            visual: Renderable::new('░', rltk::GREY, rltk::BLACK),
        });
    }
}
