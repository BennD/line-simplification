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

// ----------------------------------------------------------------------------
// Compatibility and convenience conversions to and from [f32; 2]:

impl From<[f32; 2]> for Point {
    fn from(v: [f32; 2]) -> Self {
        Point { x: v[0], y: v[1] }
    }
}

impl From<&[f32; 2]> for Point {
    fn from(v: &[f32; 2]) -> Self {
        Point { x: v[0], y: v[1] }
    }
}

impl From<Point> for [f32; 2] {
    fn from(v: Point) -> [f32; 2] {
        [v.x, v.y]
    }
}

impl From<&Point> for [f32; 2] {
    fn from(v: &Point) -> [f32; 2] {
        [v.x, v.y]
    }
}

// ----------------------------------------------------------------------------
// Compatibility and convenience conversions to and from (f32, f32):

impl From<(f32, f32)> for Point {
    fn from(v: (f32, f32)) -> Self {
        Point { x: v.0, y: v.1 }
    }
}

impl From<&(f32, f32)> for Point {
    fn from(v: &(f32, f32)) -> Self {
        Point { x: v.0, y: v.1 }
    }
}

impl From<Point> for (f32, f32) {
    fn from(v: Point) -> (f32, f32) {
        (v.x, v.y)
    }
}

impl From<&Point> for (f32, f32) {
    fn from(v: &Point) -> (f32, f32) {
        (v.x, v.y)
    }
}

// ----------------------------------------------------------------------------
// egui compatibility and convenience conversions

#[cfg(feature = "emath")]
impl From<emath::Pos2> for Point {
    fn from(v: emath::Pos2) -> Self {
        Point { x: v.x, y: v.y }
    }
}

#[cfg(feature = "emath")]
impl From<&emath::Pos2> for Point {
    fn from(v: &emath::Pos2) -> Self {
        Point { x: v.x, y: v.y }
    }
}

#[cfg(feature = "emath")]
impl From<Point> for emath::Pos2 {
    fn from(v: Point) -> Self {
        emath::Pos2 { x: v.x, y: v.y }
    }
}

#[cfg(feature = "emath")]
impl From<&Point> for emath::Pos2 {
    fn from(v: &Point) -> Self {
        emath::Pos2 { x: v.x, y: v.y }
    }
}
