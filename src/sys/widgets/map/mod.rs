pub mod rect;

#[derive(PartialEq, Copy, Clone)]
pub struct Rect {
    pub top_left_x: i32,
    pub top_left_y: i32,
    pub bottom_right_x: i32,
    pub bottom_right_y: i32,
    pub height: i32,
    pub width: i32,
}
