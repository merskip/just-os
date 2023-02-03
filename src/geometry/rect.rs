use crate::geometry::position::Position;
use crate::geometry::size::Size;

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    origin: Position,
    size: Size,
}

impl Rect {
    pub fn new(origin: Position, size: Size) -> Self {
        Self { origin, size }
    }
}
