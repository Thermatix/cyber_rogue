use super::Player;

#[derive(Debug)]
pub enum Movements {
    Up(i32),
    Left(i32),
    Down(i32),
    Right(i32),
}

impl Player {
    pub fn new() -> Self {
        Self {
            movements: Vec::new(),
        }
    }
}
