use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

use std::cmp::{max, min};

pub mod player;
mod position;
mod renderable;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct LeftMover {}

#[derive(Component, Debug)]
pub struct Player {
    pub movements: Vec<player::Movements>,
}
