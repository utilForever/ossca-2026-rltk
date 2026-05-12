use crate::prelude::PointF;
use std::convert::TryInto;
use std::ops;

/// Defines a rectangle with floating-point coordinates.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct RectF {
    /// The X position of the first point (typically the left)
    pub x1: f32,
    /// The X position of the second point (typically the right)
    pub x2: f32,
    /// The Y position of the first point (typically the top)
    pub y1: f32,
    /// The Y position of the second point (typically the bottom)
    pub y2: f32,
}

#[cfg(feature = "specs")]
impl specs::prelude::Component for RectF {
    type Storage = specs::prelude::VecStorage<Self>;
}

impl Default for RectF {
    fn default() -> RectF {
        RectF::zero()
    }
}

impl RectF {
    /// Create a new rectangle, specifying X/Y Width/Height
    ///
    /// # Panics
    ///
    /// This can panic if X, Y, Width, or Height are not convertible to an `f32`.
    pub fn with_size<T>(x: T, y: T, w: T, h: T) -> RectF
    where
        T: TryInto<f32>,
    {
        let x_f32: f32 = x.try_into().ok().unwrap();
        let y_f32: f32 = y.try_into().ok().unwrap();
        RectF {
            x1: x_f32,
            y1: y_f32,
            x2: x_f32 + w.try_into().ok().unwrap(),
            y2: y_f32 + h.try_into().ok().unwrap(),
        }
    }

    /// Create a new rectangle, specifying exact dimensions
    ///
    /// # Panics
    ///
    /// This can panic if X1, Y1, X2, or Y2 are not convertible to an `f32`.
    pub fn with_exact<T>(x1: T, y1: T, x2: T, y2: T) -> RectF
    where
        T: TryInto<f32>,
    {
        RectF {
            x1: x1.try_into().ok().unwrap(),
            y1: y1.try_into().ok().unwrap(),
            x2: x2.try_into().ok().unwrap(),
            y2: y2.try_into().ok().unwrap(),
        }
    }

    /// Creates a zero rectangle
    #[must_use]
    pub fn zero() -> RectF {
        RectF {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
        }
    }

    /// Returns a rectangle with ordered coordinates.
    #[must_use]
    pub fn normalized(&self) -> RectF {
        RectF {
            x1: self.x1.min(self.x2),
            y1: self.y1.min(self.y2),
            x2: self.x1.max(self.x2),
            y2: self.y1.max(self.y2),
        }
    }

    /// Returns the rectangle's area.
    #[must_use]
    pub fn area(&self) -> f32 {
        self.width() * self.height()
    }

    /// Returns true if the rectangle has zero width or height.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.width() <= 0.0 || self.height() <= 0.0
    }

    /// Returns true if the other rectangle's bounds are fully inside this rectangle.
    #[must_use]
    pub fn contains_rect(&self, other: &RectF) -> bool {
        let bounds = self.normalized();
        let candidate = other.normalized();

        candidate.x1 >= bounds.x1
            && candidate.x2 <= bounds.x2
            && candidate.y1 >= bounds.y1
            && candidate.y2 <= bounds.y2
    }

    /// Returns the non-empty overlapping area between this rectangle and another.
    #[must_use]
    pub fn intersection(&self, other: &RectF) -> Option<RectF> {
        let self_bounds = self.normalized();
        let other_bounds = other.normalized();

        let left = self_bounds.x1.max(other_bounds.x1);
        let top = self_bounds.y1.max(other_bounds.y1);
        let right = self_bounds.x2.min(other_bounds.x2);
        let bottom = self_bounds.y2.min(other_bounds.y2);

        if left < right && top < bottom {
            Some(RectF {
                x1: left,
                x2: right,
                y1: top,
                y2: bottom,
            })
        } else {
            None
        }
    }

    /// Returns true if this overlaps with other
    #[must_use]
    pub fn intersect(&self, other: &RectF) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    /// Returns the center of the rectangle
    #[must_use]
    pub fn center(&self) -> PointF {
        PointF {
            x: f32::midpoint(self.x1, self.x2),
            y: f32::midpoint(self.y1, self.y2),
        }
    }

    /// Returns true if a point is inside the rectangle
    #[must_use]
    pub fn point_in_rect(&self, point: PointF) -> bool {
        point.x >= self.x1 && point.x < self.x2 && point.y >= self.y1 && point.y < self.y2
    }

    /// Returns the rectangle's width
    #[must_use]
    pub fn width(&self) -> f32 {
        f32::abs(self.x2 - self.x1)
    }

    /// Returns the rectangle's height
    #[must_use]
    pub fn height(&self) -> f32 {
        f32::abs(self.y2 - self.y1)
    }
}

impl ops::Add<RectF> for RectF {
    type Output = RectF;
    fn add(mut self, rhs: RectF) -> RectF {
        let w = self.width();
        let h = self.height();
        self.x1 += rhs.x1;
        self.x2 = self.x1 + w;
        self.y1 += rhs.y1;
        self.y2 = self.y1 + h;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{PointF, RectF};

    fn assert_rectf_close(actual: RectF, expected: RectF) {
        assert!((actual.x1 - expected.x1).abs() < f32::EPSILON);
        assert!((actual.y1 - expected.y1).abs() < f32::EPSILON);
        assert!((actual.x2 - expected.x2).abs() < f32::EPSILON);
        assert!((actual.y2 - expected.y2).abs() < f32::EPSILON);
    }

    #[test]
    fn test_dimensions() {
        let rect = RectF::with_size(0.0, 0.0, 10.0, 10.0);
        assert!((rect.width() - 10.0).abs() < f32::EPSILON);
        assert!((rect.height() - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_normalized() {
        let rect = RectF::with_exact(10.0, 20.0, 5.0, 15.0);
        assert_rectf_close(rect.normalized(), RectF::with_exact(5.0, 15.0, 10.0, 20.0));
    }

    #[test]
    fn test_area() {
        let rect = RectF::with_size(0.0, 0.0, 10.0, 5.0);
        assert!((rect.area() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_is_empty() {
        assert!(RectF::with_size(0.0, 0.0, 0.0, 5.0).is_empty());
        assert!(RectF::with_size(0.0, 0.0, 5.0, 0.0).is_empty());
        assert!(!RectF::with_size(0.0, 0.0, 5.0, 5.0).is_empty());
    }

    #[test]
    fn test_contains_rect() {
        let bounds = RectF::with_size(0.0, 0.0, 10.0, 10.0);
        let inside = RectF::with_size(2.0, 2.0, 3.0, 3.0);
        let outside = RectF::with_size(8.0, 8.0, 3.0, 3.0);

        assert!(bounds.contains_rect(&inside));
        assert!(!bounds.contains_rect(&outside));
    }

    #[test]
    fn test_intersection() {
        let bounds = RectF::with_size(0.0, 0.0, 10.0, 10.0);
        let overlapping = RectF::with_size(5.0, 5.0, 10.0, 10.0);
        let touching = RectF::with_size(10.0, 10.0, 5.0, 5.0);

        assert_rectf_close(
            bounds.intersection(&overlapping).unwrap(),
            RectF::with_exact(5.0, 5.0, 10.0, 10.0),
        );
        assert!(bounds.intersection(&touching).is_none());
    }

    #[test]
    fn test_add() {
        let rect = RectF::with_size(0.0, 0.0, 10.0, 10.0) + RectF::with_size(1.0, 1.0, 1.0, 1.0);

        assert_rectf_close(rect, RectF::with_exact(1.0, 1.0, 11.0, 11.0));
    }

    #[test]
    fn test_intersect() {
        let r1 = RectF::with_size(0.0, 0.0, 10.0, 10.0);
        let r2 = RectF::with_size(5.0, 5.0, 10.0, 10.0);
        let r3 = RectF::with_size(100.0, 100.0, 5.0, 5.0);

        assert!(r1.intersect(&r2));
        assert!(!r1.intersect(&r3));
    }

    #[test]
    fn test_center() {
        let r1 = RectF::with_size(0.0, 0.0, 10.0, 10.0);
        let center = r1.center();

        assert!((center.x - 5.0).abs() < f32::EPSILON);
        assert!((center.y - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_point_in_rect() {
        let r1 = RectF::with_size(0.0, 0.0, 10.0, 10.0);

        assert!(r1.point_in_rect(PointF { x: 5.0, y: 5.0 }));
        assert!(!r1.point_in_rect(PointF { x: 100.0, y: 100.0 }));
    }
}
