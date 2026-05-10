#![allow(clippy::similar_names)]

use crate::prelude::Point;

/// Returns all points inside a filled ellipse.
///
/// The ellipse is centered on `center`, extends `radius_x` tiles horizontally
/// and `radius_y` tiles vertically, and includes points on the boundary.
/// Negative radii are treated as their absolute values.
///
/// Points are returned in a deterministic top-to-bottom, left-to-right order.
#[must_use]
pub fn ellipse2d(center: Point, radius_x: i32, radius_y: i32) -> Vec<Point> {
    let radius_x = i64::from(radius_x).abs();
    let radius_y = i64::from(radius_y).abs();

    match (radius_x, radius_y) {
        (0, 0) => vec![center],
        (0, _) => vertical_line(center, radius_y),
        (_, 0) => horizontal_line(center, radius_x),
        _ => ellipse_area(center, radius_x, radius_y),
    }
}

fn horizontal_line(center: Point, radius_x: i64) -> Vec<Point> {
    (-radius_x..=radius_x)
        .filter_map(|dx| offset_point(center, dx, 0))
        .collect()
}

fn vertical_line(center: Point, radius_y: i64) -> Vec<Point> {
    (-radius_y..=radius_y)
        .filter_map(|dy| offset_point(center, 0, dy))
        .collect()
}

fn ellipse_area(center: Point, radius_x: i64, radius_y: i64) -> Vec<Point> {
    let radius_x_squared = i128::from(radius_x) * i128::from(radius_x);
    let radius_y_squared = i128::from(radius_y) * i128::from(radius_y);
    let threshold = radius_x_squared * radius_y_squared;
    let mut points = Vec::new();

    for dy in -radius_y..=radius_y {
        for dx in -radius_x..=radius_x {
            let dx_squared = i128::from(dx) * i128::from(dx);
            let dy_squared = i128::from(dy) * i128::from(dy);
            let distance = dx_squared * radius_y_squared + dy_squared * radius_x_squared;
            if distance <= threshold {
                if let Some(point) = offset_point(center, dx, dy) {
                    points.push(point);
                }
            }
        }
    }

    points
}

fn offset_point(center: Point, dx: i64, dy: i64) -> Option<Point> {
    let x = i32::try_from(i64::from(center.x) + dx).ok()?;
    let y = i32::try_from(i64::from(center.y) + dy).ok()?;
    Some(Point::new(x, y))
}

#[cfg(test)]
mod tests {
    use crate::prelude::{ellipse2d, Point};
    use rstest::rstest;

    #[rstest]
    #[case(Point::new(2, -3), 0, 0, vec![Point::new(2, -3)])]
    #[case(
        Point::new(5, 5),
        2,
        0,
        vec![
            Point::new(3, 5),
            Point::new(4, 5),
            Point::new(5, 5),
            Point::new(6, 5),
            Point::new(7, 5)
        ]
    )]
    #[case(
        Point::new(5, 5),
        0,
        2,
        vec![
            Point::new(5, 3),
            Point::new(5, 4),
            Point::new(5, 5),
            Point::new(5, 6),
            Point::new(5, 7)
        ]
    )]
    #[case(
        Point::new(0, 0),
        2,
        1,
        vec![
            Point::new(0, -1),
            Point::new(-2, 0),
            Point::new(-1, 0),
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(0, 1)
        ]
    )]
    fn ellipse_returns_expected_points(
        #[case] center: Point,
        #[case] radius_x: i32,
        #[case] radius_y: i32,
        #[case] expected: Vec<Point>,
    ) {
        assert_eq!(ellipse2d(center, radius_x, radius_y), expected);
    }

    #[rstest]
    #[case(-3, -2, 3, 2)]
    #[case(3, -2, 3, 2)]
    #[case(-3, 2, 3, 2)]
    fn negative_radii_match_positive_radii(
        #[case] radius_x: i32,
        #[case] radius_y: i32,
        #[case] positive_radius_x: i32,
        #[case] positive_radius_y: i32,
    ) {
        assert_eq!(
            ellipse2d(Point::new(0, 0), radius_x, radius_y),
            ellipse2d(Point::new(0, 0), positive_radius_x, positive_radius_y)
        );
    }

    #[test]
    fn every_point_is_inside_the_ellipse() {
        let radius_x = 4;
        let radius_y = 3;
        let radius_x_squared = radius_x * radius_x;
        let radius_y_squared = radius_y * radius_y;
        let threshold = radius_x_squared * radius_y_squared;

        for point in ellipse2d(Point::new(10, 20), radius_x, radius_y) {
            let dx = point.x - 10;
            let dy = point.y - 20;
            let distance = dx * dx * radius_y_squared + dy * dy * radius_x_squared;
            assert!(distance <= threshold);
        }
    }

    #[rstest]
    #[case(
        Point::new(i32::MAX, 7),
        2,
        0,
        vec![
                Point::new(i32::MAX - 2, 7),
                Point::new(i32::MAX - 1, 7),
                Point::new(i32::MAX, 7),
            ]
    )]
    #[case(
        Point::new(-3, i32::MIN),
        0,
        2,
        vec![
                Point::new(-3, i32::MIN),
                Point::new(-3, i32::MIN + 1),
                Point::new(-3, i32::MIN + 2),
            ]
    )]
    fn points_that_overflow_i32_bounds_are_skipped(
        #[case] center: Point,
        #[case] radius_x: i32,
        #[case] radius_y: i32,
        #[case] expected: Vec<Point>,
    ) {
        assert_eq!(ellipse2d(center, radius_x, radius_y), expected);
    }
}
