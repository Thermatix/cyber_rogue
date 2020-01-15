use crate::game::entity::Renderable;

mod tile;
#[derive(PartialEq, Copy, Clone)]
pub struct Tile<'t> {
    pub blocking: bool,
    pub name: &'t str,
    pub visual: Renderable,
}

mod tile_set;
pub struct TileSet<'ts> {
    pub list: Vec<Tile<'ts>>,
}
mod map;
pub struct Map<'m> {
    pub tiles: Vec<Tile<'m>>,
    pub blocking: Vec<bool>,
    pub height: usize,
    pub width: usize,
}

mod map_generator;
pub use map_generator::MapGenerator;
