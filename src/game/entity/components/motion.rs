use super::Motion;

#[derive(Debug)]
pub enum Motions {
    Up(i32),
    Left(i32),
    Down(i32),
    Right(i32),
}

impl Motion {
    pub fn new() -> Self {
        Self {
            motions: Vec::new(),
        }
    }
}
