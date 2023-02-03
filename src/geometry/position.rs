#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

impl Point {
    #[deprecated]
    pub fn next(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    #[deprecated]
    pub fn next_row(&self) -> Point {
        Point {
            x: 0,
            y: self.y + 1,
        }
    }
}
