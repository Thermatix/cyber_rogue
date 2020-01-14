use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

use std::cmp::{max, min};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}
