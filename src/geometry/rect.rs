use core::fmt::{Display, Formatter};

use crate::geometry::position::Point;
use crate::geometry::size::Size;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

#[allow(dead_code)]
impl Rect {
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    pub fn from(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            origin: Point::new(x, y),
            size: Size::new(width, height)
        }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Rect(origin={}, size={})", self.origin, self.size)
    }
}

impl Rect {
    pub fn min_x(&self) -> usize {
        self.origin.x
    }

    pub fn max_x(&self) -> usize {
        self.origin.x + self.size.width - 1
    }

    pub fn min_y(&self) -> usize {
        self.origin.y
    }

    pub fn max_y(&self) -> usize {
        self.origin.y + self.size.height - 1
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.min_x() && point.x <= self.max_x()
            && point.y >= self.min_y() && point.y <= self.max_y()
    }
}

#[allow(dead_code)]
impl Rect {
    pub fn corner_upper_left(&self) -> Point {
        Point::new(self.min_x(), self.min_y())
    }

    pub fn corner_upper_right(&self) -> Point {
        Point::new(self.max_x(), self.min_y())
    }

    pub fn corner_lower_left(&self) -> Point {
        Point::new(self.min_x(), self.max_y())
    }

    pub fn corner_lower_right(&self) -> Point {
        Point::new(self.max_x(), self.max_y())
    }
}

#[test_case]
fn test_rect_min_and_max() {
    let rect = Rect::from(2, 2, 3, 3);

    assert_eq!(rect.min_x(), 2);
    assert_eq!(rect.min_y(), 2);
    assert_eq!(rect.max_x(), 4);
    assert_eq!(rect.max_y(), 4);
}

#[test_case]
fn test_rect_contains() {
    let rect = Rect::from(2, 2, 3, 3);

    assert!(rect.contains(Point::new(2, 2)));
    assert!(rect.contains(Point::new(4, 2)));
    assert!(rect.contains(Point::new(2, 4)));
    assert!(rect.contains(Point::new(4 ,4)));

    assert!(!rect.contains(Point::new(1, 1)));
    assert!(!rect.contains(Point::new(2, 1)));
    assert!(!rect.contains(Point::new(1, 2)));

    assert!(!rect.contains(Point::new(5, 4)));
    assert!(!rect.contains(Point::new(5, 5)));
    assert!(!rect.contains(Point::new(4, 5)));
}

#[test_case]
fn test_rect_corner() {
    let rect = Rect::from(2, 2, 3, 3);

    assert_eq!(rect.corner_upper_left(), Point::new(2, 2));
    assert_eq!(rect.corner_upper_right(), Point::new(4, 2));
    assert_eq!(rect.corner_lower_left(), Point::new(2, 4));
    assert_eq!(rect.corner_lower_right(), Point::new(4, 4));
}