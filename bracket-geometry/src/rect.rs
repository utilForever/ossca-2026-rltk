use crate::prelude::Point;
use std::collections::HashSet;
use std::convert::TryInto;
use std::ops;

/// Defines a two-dimensional rectangle.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Rect {
    /// The X position of the first point (typically the left)
    pub x1: i32,
    /// The X position of the second point (typically the right)
    pub x2: i32,
    /// The Y position of the first point (typically the top)
    pub y1: i32,
    /// The Y position of the second point (typically the bottom)
    pub y2: i32,
}

#[cfg(feature = "specs")]
impl specs::prelude::Component for Rect {
    type Storage = specs::prelude::VecStorage<Self>;
}

impl Default for Rect {
    fn default() -> Rect {
        Rect::zero()
    }
}

impl Rect {
    /// Create a new rectangle, specifying X/Y Width/Height
    ///
    /// # Panics
    ///
    /// This can panic if X, Y, Width, or Height are not convertible to an `i32`.
    pub fn with_size<T>(x: T, y: T, w: T, h: T) -> Rect
    where
        T: TryInto<i32>,
    {
        let x_i32: i32 = x.try_into().ok().unwrap();
        let y_i32: i32 = y.try_into().ok().unwrap();
        Rect {
            x1: x_i32,
            y1: y_i32,
            x2: x_i32 + w.try_into().ok().unwrap(),
            y2: y_i32 + h.try_into().ok().unwrap(),
        }
    }

    /// Create a new rectangle, specifying exact dimensions
    ///
    /// # Panics
    ///
    /// This can panic if X1, Y1, X2, or Y2 are not convertible to an `i32`.
    pub fn with_exact<T>(x1: T, y1: T, x2: T, y2: T) -> Rect
    where
        T: TryInto<i32>,
    {
        Rect {
            x1: x1.try_into().ok().unwrap(),
            y1: y1.try_into().ok().unwrap(),
            x2: x2.try_into().ok().unwrap(),
            y2: y2.try_into().ok().unwrap(),
        }
    }

    /// Creates a zero rectangle
    #[must_use]
    pub fn zero() -> Rect {
        Rect {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        }
    }

    /// Returns a rectangle with ordered coordinates.
    #[must_use]
    pub fn normalized(&self) -> Rect {
        Rect {
            x1: self.x1.min(self.x2),
            y1: self.y1.min(self.y2),
            x2: self.x1.max(self.x2),
            y2: self.y1.max(self.y2),
        }
    }

    /// Returns the rectangle's area.
    #[must_use]
    pub fn area(&self) -> i32 {
        self.width().saturating_mul(self.height())
    }

    /// Returns true if the rectangle has zero width or height.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.width() == 0 || self.height() == 0
    }

    /// Returns true if the other rectangle's bounds are fully inside this rectangle.
    #[must_use]
    pub fn contains_rect(&self, other: &Rect) -> bool {
        let bounds = self.normalized();
        let candidate = other.normalized();

        candidate.x1 >= bounds.x1
            && candidate.x2 <= bounds.x2
            && candidate.y1 >= bounds.y1
            && candidate.y2 <= bounds.y2
    }

    /// Returns the non-empty overlapping area between this rectangle and another.
    #[must_use]
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let self_bounds = self.normalized();
        let other_bounds = other.normalized();

        let left = self_bounds.x1.max(other_bounds.x1);
        let top = self_bounds.y1.max(other_bounds.y1);
        let right = self_bounds.x2.min(other_bounds.x2);
        let bottom = self_bounds.y2.min(other_bounds.y2);

        if left < right && top < bottom {
            Some(Rect {
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
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    /// Returns the center of the rectangle
    #[must_use]
    pub fn center(&self) -> Point {
        Point::new(
            i32::midpoint(self.x1, self.x2),
            i32::midpoint(self.y1, self.y2),
        )
    }

    /// Returns true if a point is inside the rectangle
    #[must_use]
    pub fn point_in_rect(&self, point: Point) -> bool {
        point.x >= self.x1 && point.x < self.x2 && point.y >= self.y1 && point.y < self.y2
    }

    /// Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Point),
    {
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                f(Point::new(x, y));
            }
        }
    }

    /// Gets a set of all tiles in the rectangle
    #[must_use]
    pub fn point_set(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                result.insert(Point::new(x, y));
            }
        }
        result
    }

    /// Returns the rectangle's width
    #[must_use]
    pub fn width(&self) -> i32 {
        i32::abs(self.x2 - self.x1)
    }

    /// Returns the rectangle's height
    #[must_use]
    pub fn height(&self) -> i32 {
        i32::abs(self.y2 - self.y1)
    }
}

impl ops::Add<Rect> for Rect {
    type Output = Rect;
    fn add(mut self, rhs: Rect) -> Rect {
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
    use crate::prelude::{Point, Rect};

    #[test]
    fn test_default_is_zero() {
        let default_rect = Rect::default();
        let zero_rect = Rect::zero();
        assert_eq!(default_rect, zero_rect);
    }

    #[test]
    fn test_dimensions() {
        let rect = Rect::with_size(0, 0, 10, 10);
        assert!(rect.width() == 10);
        assert!(rect.height() == 10);
    }

    #[test]
    fn test_normalized() {
        let rect = Rect::with_exact(10, 20, 5, 15);
        assert_eq!(rect.normalized(), Rect::with_exact(5, 15, 10, 20));
    }

    #[test]
    fn test_area() {
        let rect = Rect::with_size(0, 0, 10, 5);
        assert_eq!(rect.area(), 50);
    }

    #[test]
    fn test_is_empty() {
        assert!(Rect::with_size(0, 0, 0, 5).is_empty());
        assert!(Rect::with_size(0, 0, 5, 0).is_empty());
        assert!(!Rect::with_size(0, 0, 5, 5).is_empty());
    }

    #[test]
    fn test_contains_rect() {
        let bounds = Rect::with_size(0, 0, 10, 10);
        let inside = Rect::with_size(2, 2, 3, 3);
        let outside = Rect::with_size(8, 8, 3, 3);

        assert!(bounds.contains_rect(&inside));
        assert!(!bounds.contains_rect(&outside));
    }

    #[test]
    fn test_intersection() {
        let bounds = Rect::with_size(0, 0, 10, 10);
        let overlapping = Rect::with_size(5, 5, 10, 10);
        let touching = Rect::with_size(10, 10, 5, 5);

        assert_eq!(
            bounds.intersection(&overlapping),
            Some(Rect::with_exact(5, 5, 10, 10))
        );
        assert_eq!(bounds.intersection(&touching), None);
    }

    #[test]
    fn test_add() {
        let rect = Rect::with_size(0, 0, 10, 10) + Rect::with_size(1, 1, 1, 1);
        assert!(rect.x1 == 1 && rect.y1 == 1);
        assert!(rect.x2 == 11 && rect.y2 == 11);
    }

    #[test]
    fn test_intersect() {
        // case 1: r2 is strictly inside r1
        let r1 = Rect::with_size(0, 0, 10, 10);
        let r2 = Rect::with_size(2, 2, 5, 5);
        assert!(r1.intersect(&r2));

        // case 2: r1 and r2 partially overlap
        let r2 = Rect::with_size(5, 5, 10, 10);
        assert!(r1.intersect(&r2));

        // case 3: r1 and r2 are identical
        let r2 = Rect::with_size(0, 0, 10, 10);
        assert!(r1.intersect(&r2));

        // case 4: r1 and r2 share an edge
        let r2 = Rect::with_size(10, 0, 10, 10);
        assert!(r1.intersect(&r2));

        // case 5: r1 and r2 share a vertex
        let r2 = Rect::with_size(10, 10, 10, 10);
        assert!(r1.intersect(&r2));

        // case 6: r1 and r2 do not overlap at all
        let r2 = Rect::with_size(100, 100, 5, 5);
        assert!(!r1.intersect(&r2));
    }

    #[test]
    fn test_center() {
        let r1 = Rect::with_size(0, 0, 10, 10);
        assert_eq!(r1.center(), Point::new(5, 5));

        let r2 = Rect::with_size(0, 0, 11, 11);
        assert_eq!(r2.center(), Point::new(5, 5));

        let r3 = Rect::with_size(-4, -5, 10, 14);
        assert_eq!(r3.center(), Point::new(1, 2));
    }

    #[test]
    fn test_point_in_rect() {
        let r1 = Rect::with_size(0, 0, 10, 10);
        // case 1: point is strictly inside the rectangle
        assert!(r1.point_in_rect(Point::new(5, 5)));
        // case 2: point is on the edge
        assert!(r1.point_in_rect(Point::new(0, 5)));
        // case 3: point is in the rect only if x1 < x2 and y1 < y2
        assert!(!r1.point_in_rect(Point::new(10, 5)));
        assert!(!r1.point_in_rect(Point::new(5, 10)));
        // case 4: point is outside the rectangle
        assert!(!r1.point_in_rect(Point::new(100, 100)));
    }

    #[test]
    fn test_rect_set() {
        let r1 = Rect::with_size(0, 0, 1, 1);
        let points = r1.point_set();
        assert!(points.contains(&Point::new(0, 0)));
        assert!(!points.contains(&Point::new(1, 0)));
        assert!(!points.contains(&Point::new(0, 1)));
        assert!(!points.contains(&Point::new(1, 1)));
    }

    #[test]
    fn test_rect_callback() {
        use std::vec::Vec;

        // test for_each's sequential callback by pushing points into a vector
        let r1 = Rect::with_size(0, 0, 2, 2);
        let mut points = Vec::new();
        r1.for_each(|p| {
            points.push(p);
        });

        assert_eq!(
            points,
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(1, 1),
            ]
        );
    }
}
