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
pub fn ellipse(center: Point, radius_x: i32, radius_y: i32) -> Vec<Point> {
    let radius_x = radius_x.abs();
    let radius_y = radius_y.abs();

    match (radius_x, radius_y) {
        (0, 0) => vec![center],
        (0, _) => vertical_line(center, radius_y),
        (_, 0) => horizontal_line(center, radius_x),
        _ => ellipse_area(center, radius_x, radius_y),
    }
}

fn horizontal_line(center: Point, radius_x: i32) -> Vec<Point> {
    (-radius_x..=radius_x)
        .map(|dx| offset_point(center, dx, 0))
        .collect()
}

fn vertical_line(center: Point, radius_y: i32) -> Vec<Point> {
    (-radius_y..=radius_y)
        .map(|dy| offset_point(center, 0, dy))
        .collect()
}

fn ellipse_area(center: Point, radius_x: i32, radius_y: i32) -> Vec<Point> {
    let radius_x_squared = radius_x * radius_x;
    let radius_y_squared = radius_y * radius_y;
    let threshold = radius_x_squared * radius_y_squared;
    let mut points = Vec::new();

    for dy in -radius_y..=radius_y {
        for dx in -radius_x..=radius_x {
            let dx_squared = dx * dx;
            let dy_squared = dy * dy;
            let distance = dx_squared * radius_y_squared + dy_squared * radius_x_squared;
            if distance <= threshold {
                points.push(offset_point(center, dx, dy));
            }
        }
    }

    points
}

fn offset_point(center: Point, dx: i32, dy: i32) -> Point {
    Point::new(center.x + dx, center.y + dy)
}

#[cfg(test)]
mod tests {
    use crate::prelude::{ellipse, Point};
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
        assert_eq!(ellipse(center, radius_x, radius_y), expected);
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
            ellipse(Point::new(0, 0), radius_x, radius_y),
            ellipse(Point::new(0, 0), positive_radius_x, positive_radius_y)
        );
    }

    #[test]
    fn every_point_is_inside_the_ellipse() {
        let radius_x = 4;
        let radius_y = 3;
        let radius_x_squared = radius_x * radius_x;
        let radius_y_squared = radius_y * radius_y;
        let threshold = radius_x_squared * radius_y_squared;

        for point in ellipse(Point::new(10, 20), radius_x, radius_y) {
            let dx = point.x - 10;
            let dy = point.y - 20;
            let distance = dx * dx * radius_y_squared + dy * dy * radius_x_squared;
            assert!(distance <= threshold);
        }
    }
}
