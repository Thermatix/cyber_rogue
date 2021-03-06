use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

use std::cmp::{max, min};
use std::collections::HashMap;

use crate::sys::element::{Map, Tile, TileSet};

pub mod event_stream;
mod event_value;
mod field_of_view;
mod location;
mod player;
mod position;
mod renderable;
mod revealed_tiles;
// mod sense_of_touch;

#[derive(Component, PartialEq, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub use renderable::GlyphType;

#[derive(Component, PartialEq, Debug, Clone)]
#[storage(VecStorage)]
pub struct Renderable {
    pub glyph: Vec<u8>,
    pub kind: GlyphType,
    pub g_id: usize,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct LeftMover {}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Location {
    pub stack: Vec<String>,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct SenseOfTouch {
    pub vicinity: Vec<String>,
}

pub use event_stream::Event;
pub use event_value::EventValue;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct EventStream {
    pub stream: event_stream::Stream,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct FieldOfView {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct RevealedTiles {
    pub revealed: HashMap<String, Vec<bool>>,
    pub visible: HashMap<String, Vec<bool>>,
}
