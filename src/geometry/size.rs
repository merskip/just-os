#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub const fn new(width: usize, height: usize) -> Self {
        Size { width, height }
    }
}
