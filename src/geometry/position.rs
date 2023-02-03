#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub const fn zero() -> Self {
        Point { x: 0, y: 0 }
    }

    pub const fn new(column: usize, row: usize) -> Self {
        Point { x: column, y: row }
    }
}

impl Point {
    pub fn next(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }

         
    }

    pub fn next_row(&self) -> Point {
        Point {
            x: 0,
            y: self.y + 1,
        }
    }
}
