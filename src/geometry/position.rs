use core::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub const fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Point(x={}, y={})", self.x, self.y)
    }
}
