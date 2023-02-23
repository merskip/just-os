use core::fmt::{Display, Formatter};

use crate::geometry::position::Point;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub const fn new(width: usize, height: usize) -> Self {
        Size { width, height }
    }

    pub const fn area(&self) -> usize {
        self.width * self.height
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Size(width={}, height={})", self.width, self.height)
    }
}

impl Size {
    pub fn points(&self) -> impl Iterator<Item=Point> {
        SizePointsIterator::new(self.clone())
    }
}

struct SizePointsIterator {
    size: Size,
    next_x: usize,
    next_y: usize
}

impl SizePointsIterator {
    fn new(size: Size) -> Self {
        Self {
            size,
            next_x: 0,
            next_y: 0
        }
    }
}

impl Iterator for SizePointsIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_x >= self.size.width {
            self.next_x = 0;
            self.next_y += 1;
        }

        if self.next_y >= self.size.height {
            return None;
        }

        let point = Point::new(self.next_x, self.next_y);
        self.next_x += 1;
        Some(point)
    }
}
