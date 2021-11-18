use line_simplification::douglas_peucker::douglas_peucker_v1;
use line_simplification::types::Point;

#[test]
fn simple_line_test() {
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
