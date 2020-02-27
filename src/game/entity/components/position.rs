use super::Position;
use std::ops::{Add, Sub};

#[derive(PartialEq, Debug)]
pub enum Types {
    Pos(Position),
    Point((i32, i32)),
}

impl From<(i32, i32)> for Types {
    fn from(v: (i32, i32)) -> Self {
        Types::Point(v)
    }
}

impl From<Types> for (i32, i32) {
    fn from(v: Types) -> Self {
        match v {
            Types::Point(n) => n,
            _ => panic!("{:?} was not a Position", v),
        }
    }
}

impl From<Position> for Types {
    fn from(v: Position) -> Self {
        Types::Pos(v)
    }
}

impl From<Types> for Position {
    fn from(v: Types) -> Self {
        match v {
            Types::Pos(n) => n,
            _ => panic!("{:?} was not a Position", v),
        }
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add<K: Into<Types>>(&mut self, other: K) -> (i32, i32) {
        match other.into() {
            Types::Point(o) => (self.x + o.0, self.y + o.1),
            Types::Pos(o) => (self.x + o.x, self.y + o.y),
        }
    }

    pub fn sub<K: Into<Types>>(&mut self, other: K) -> (i32, i32) {
        match other.into() {
            Types::Point(o) => (self.x - o.0, self.y - o.1),
            Types::Pos(o) => (self.x - o.x, self.y - o.y),
        }
    }
}
