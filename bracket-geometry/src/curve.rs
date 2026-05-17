use crate::prelude::Point;

/// Defines a two-dimensional curve by its control points.
///
/// This type stores the shared curve data used by higher-level curve
/// algorithms. It does not assume a specific interpolation or evaluation
/// strategy.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Curve {
    /// The points controlling the curve shape.
    pub control_points: Vec<Point>,
}

impl Curve {
    /// Creates a new curve from a set of control points.
    #[must_use]
    pub fn new(control_points: Vec<Point>) -> Self {
        Self { control_points }
    }

    /// Returns the control points as a slice.
    #[must_use]
    pub fn control_points(&self) -> &[Point] {
        &self.control_points
    }

    /// Returns the number of control points in the curve.
    #[must_use]
    pub fn len(&self) -> usize {
        self.control_points.len()
    }

    /// Returns true if the curve has no control points.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.control_points.is_empty()
    }

    /// Returns the first control point, if one is defined.
    #[must_use]
    pub fn first(&self) -> Option<Point> {
        self.control_points.first().copied()
    }

    /// Returns the last control point, if one is defined.
    #[must_use]
    pub fn last(&self) -> Option<Point> {
        self.control_points.last().copied()
    }
}

impl From<Vec<Point>> for Curve {
    fn from(control_points: Vec<Point>) -> Self {
        Self::new(control_points)
    }
}

impl From<&[Point]> for Curve {
    fn from(control_points: &[Point]) -> Self {
        Self::new(control_points.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Curve, Point};

    #[test]
    fn new_curve_stores_control_points() {
        let points = vec![Point::new(0, 0), Point::new(4, 8), Point::new(9, 2)];
        let curve = Curve::new(points.clone());

        assert_eq!(curve.control_points(), points.as_slice());
        assert_eq!(curve.len(), 3);
        assert!(!curve.is_empty());
    }

    #[test]
    fn empty_curve_has_no_endpoints() {
        let curve = Curve::default();

        assert!(curve.is_empty());
        assert_eq!(curve.len(), 0);
        assert_eq!(curve.first(), None);
        assert_eq!(curve.last(), None);
    }

    #[test]
    fn curve_exposes_endpoints() {
        let curve = Curve::new(vec![Point::new(1, 2), Point::new(3, 4), Point::new(5, 6)]);

        assert_eq!(curve.first(), Some(Point::new(1, 2)));
        assert_eq!(curve.last(), Some(Point::new(5, 6)));
    }

    #[test]
    fn curve_can_be_created_from_points() {
        let points = [Point::new(2, 3), Point::new(4, 5)];
        let curve = Curve::from(points.as_slice());

        assert_eq!(curve.control_points(), &points);
    }
}
