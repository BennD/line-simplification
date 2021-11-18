use crate::types::{Line, Point};
use crate::util::point_line_distance;

pub use douglas_peucker_v1 as douglas_peucker;

/// Simple Ramger-Douglas-Peucker implementation
/// > based on pseudo code on [Wikipedia](https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm)
///
/// > returns an empty vector in case `|points| < 2`
pub fn douglas_peucker_v1(points: &[Point], threshhold: f32) -> Vec<Point> {
    match points {
        [first, rest @ .., last] => {
            let line = Line {
                start: first.clone(),
                end: last.clone(),
            };

            let mut distance_max = 0.0;
            let mut index = 0;
            for (i, point) in rest.iter().enumerate() {
                let distance = point_line_distance(point, &line);
                if distance > distance_max {
                    index = i + 1; // correct index
                    distance_max = distance;
                }
            }

            if distance_max > threshhold {
                let left = douglas_peucker_v1(&points[0..index + 1], threshhold);
                let right = douglas_peucker_v1(&points[index..], threshhold);
                merge_lines(left, right)
            } else {
                vec![line.start, line.end]
            }
        }
        _ => vec![],
    }
}

/// Merges two lines
/// > \[left\) ++ \[right\]
fn merge_lines(mut left: Vec<Point>, mut right: Vec<Point>) -> Vec<Point> {
    left.pop();
    left.append(&mut right);
    left
    //[left.as_slice(), &right.as_slice()[1..]].concat() // TODO Check if this is slower
}
