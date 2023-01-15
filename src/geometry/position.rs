#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub column: usize,
    pub row: usize,
}

impl Position {
    pub const fn zero() -> Self {
        Position { column: 0, row: 0 }
    }

    pub const fn new(column: usize, row: usize) -> Self {
        Position { column, row }
    }
}

impl Position {
    pub fn next(&self) -> Position {
        Position {
            column: self.column + 1,
            row: self.row,
        }

         
    }

    pub fn next_row(&self) -> Position {
        Position {
            column: 0,
            row: self.row + 1,
        }
    }
}
