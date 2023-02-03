use crate::geometry::position::Point;
use crate::geometry::size::Size;

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    origin: Point,
    size: Size,
}

impl Rect {
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
}
