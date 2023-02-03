use crate::geometry::position::Point;
use crate::geometry::size::Size;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

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

impl Rect {
    pub fn min_x(&self) -> usize {
        self.origin.x
    }

    pub fn max_x(&self) -> usize {
        self.origin.x + self.size.width
    }

    pub fn min_y(&self) -> usize {
        self.origin.y
    }

    pub fn max_y(&self) -> usize {
        self.origin.y + self.size.height
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.min_x() && point.x <= self.max_x()
            && point.y >= self.min_y() && point.y <= self.max_y()
    }
}

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
    let rect = Rect::from(50, 100, 150, 200);

    assert_eq!(rect.min_x(), 50);
    assert_eq!(rect.max_x(), 200);
    assert_eq!(rect.min_y(), 100);
    assert_eq!(rect.max_y(), 300);
}

#[test_case]
fn test_rect_contains() {
    let rect = Rect::from(50, 100, 150, 200);

    assert!(rect.contains(Point::new(50, 100)));
    assert!(rect.contains(Point::new(200, 100)));
    assert!(rect.contains(Point::new(50, 300)));
    assert!(rect.contains(Point::new(200, 300)));

    assert!(!rect.contains(Point::new(0, 0)));
    assert!(!rect.contains(Point::new(300, 300)));
}

#[test_case]
fn test_rect_corner() {
    let rect = Rect::from(50, 100, 150, 200);

    assert_eq!(rect.corner_upper_left(), Point::new(50, 100));
    assert_eq!(rect.corner_upper_right(), Point::new(200, 100));
    assert_eq!(rect.corner_lower_left(), Point::new(50, 300));
    assert_eq!(rect.corner_lower_right(), Point::new(200, 300));
}