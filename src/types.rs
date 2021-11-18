#[derive(Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl From<[f32; 2]> for Point {
    fn from(o: [f32; 2]) -> Self {
        Point { x: o[0], y: o[1] }
    }
}
