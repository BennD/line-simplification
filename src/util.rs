use crate::types::{Line, Point};

pub fn point_line_distance(point: &Point, line: &Line) -> f32 {
    let (x0, y0) = &(point.x, point.y);
    let (x1, y1) = &(line.start.x, line.start.y);
    let (x2, y2) = &(line.end.x, line.end.y);

    let dx = x2 - x1;
    let dy = y2 - y1;
    f32::abs(dx * (y1 - y0) - (x1 - x0) * dy) / f32::sqrt(dx.powi(2) + dy.powi(2))
}
