pub mod rect;

#[derive(PartialEq, Copy, Clone)]
pub struct Rect {
    pub tlx: i32,
    pub tly: i32,
    pub brx: i32,
    pub bry: i32,
    pub height: i32,
    pub width: i32,
}
