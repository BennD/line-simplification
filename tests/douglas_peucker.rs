use line_simplification::douglas_peucker::douglas_peucker_v1;
use line_simplification::types::Point;

/// Most simple test case.
///
/// Simplifying 3 non collinear points
/// - small threshold -> middle point is removed
/// - large threshold -> all points are kept
#[test]
fn triangle() {
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.5 },
        Point { x: 2.0, y: 0.0 },
    ];

    // threshold 1.0
    {
        let simplified = douglas_peucker_v1(&points, 1.0);

        let expected = vec![Point { x: 0.0, y: 0.0 }, Point { x: 2.0, y: 0.0 }];

        assert_eq!(expected, simplified);
    }

    // threshold 0.4
    {
        let simplified = douglas_peucker_v1(&points, 0.4);

        assert_eq!(points, simplified);
    }
}

/// Make sure the algorithm always works with the absolute value of the treshold.
#[test]
fn negative_threshold() {
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.5 },
        Point { x: 2.0, y: 0.0 },
    ];

    // threshold -1.0
    {
        let simplified = douglas_peucker_v1(&points, -1.0);

        let expected = vec![Point { x: 0.0, y: 0.0 }, Point { x: 2.0, y: 0.0 }];

        assert_eq!(expected, simplified);
    }
}

/// Straight lines are always optimized. Regardless of threshold.
#[test]
fn straight_line() {
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 3.0, y: 0.0 },
        Point { x: 4.0, y: 0.0 },
    ];

    // threshold 0.0
    {
        let simplified = douglas_peucker_v1(&points, 0.0);

        let expected = vec![Point { x: 0.0, y: 0.0 }, Point { x: 4.0, y: 0.0 }];

        assert_eq!(expected, simplified);
    }
}
