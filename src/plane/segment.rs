use super::Point;

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub a: Point,
    pub b: Point,
}

impl Segment {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
}
