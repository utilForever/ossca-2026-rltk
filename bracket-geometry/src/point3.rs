use std::convert::{From, TryInto};
use std::ops;
use ultraviolet::Vec3;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
/// Helper struct defining a 3D point in space.
pub struct Point3 {
    /// The 3D point's X location
    pub x: i32,
    /// The 3D point's Y location
    pub y: i32,
    /// The 3D point's Z location
    pub z: i32,
}

#[cfg(feature = "specs")]
impl specs::prelude::Component for Point3 {
    type Storage = specs::prelude::VecStorage<Self>;
}

#[cfg(feature = "bevy")]
impl bevy::ecs::component::Component for Point3 {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;
    type Mutability = bevy::ecs::component::Mutable;
}

impl Point3 {
    /// Create a new point from an x/y/z coordinate.
    #[inline]
    #[must_use]
    pub fn new<T>(x: T, y: T, z: T) -> Point3
    where
        T: TryInto<i32>,
    {
        Point3 {
            x: x.try_into().ok().unwrap_or(0),
            y: y.try_into().ok().unwrap_or(0),
            z: z.try_into().ok().unwrap_or(0),
        }
    }

    /// Create a new point from i32, this can be constant
    #[must_use]
    pub const fn constant(x: i32, y: i32, z: i32) -> Point3 {
        Point3 { x, y, z }
    }

    /// Create a zero point
    #[inline]
    #[must_use]
    pub fn zero() -> Point3 {
        Point3 { x: 0, y: 0, z: 0 }
    }

    /// Create a point from an x/y/z tuple.
    #[inline]
    #[must_use]
    pub fn from_tuple<T>(t: (T, T, T)) -> Self
    where
        T: TryInto<i32>,
    {
        Point3::new(t.0, t.1, t.2)
    }

    /// Helper for 3D map index conversion
    ///
    /// # Panics
    ///
    /// This can panic if X, Y, or Z are not convertible to a `usize`, or if width or height are not convertible to a `usize`.
    #[inline]
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub fn to_index<T>(self, width: T, height: T) -> usize
    where
        T: TryInto<usize>,
    {
        let x: usize = self.x.try_into().ok().unwrap();
        let y: usize = self.y.try_into().ok().unwrap();
        let z: usize = self.z.try_into().ok().unwrap();
        let w: usize = width.try_into().ok().unwrap();
        let h: usize = height.try_into().ok().unwrap();
        (z * w * h) + (y * w) + x
    }

    /// Converts the point to an i32 tuple
    #[must_use]
    pub fn to_tuple(self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }

    /// Converts the point to a usize tuple
    ///
    /// # Panics
    ///
    /// This can panic if X, Y, or Z are not convertible to a `usize`.
    #[must_use]
    pub fn to_unsigned_tuple(self) -> (usize, usize, usize) {
        (
            self.x.try_into().ok().unwrap(),
            self.y.try_into().ok().unwrap(),
            self.z.try_into().ok().unwrap(),
        )
    }

    /// Converts into an `UltraViolet` Vec3
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Creates a point from an `UltraViolet` Vec3
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn from_vec3(v: Vec3) -> Self {
        Self::new(v.x as i32, v.y as i32, v.z as i32)
    }

    /// Returns a point containing the saturating absolute value of each component.
    #[must_use]
    pub fn abs(self) -> Self {
        Self::new(
            self.x.saturating_abs(),
            self.y.saturating_abs(),
            self.z.saturating_abs(),
        )
    }

    /// Returns a point containing the smaller value of each component.
    #[must_use]
    pub fn component_min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    /// Returns a point containing the larger value of each component.
    #[must_use]
    pub fn component_max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    /// Clamps each component of this point between the matching components of `min` and `max`.
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        let lower = min.component_min(max);
        let upper = min.component_max(max);
        Self::new(
            self.x.clamp(lower.x, upper.x),
            self.y.clamp(lower.y, upper.y),
            self.z.clamp(lower.z, upper.z),
        )
    }

    /// Returns the sign of each component as -1, 0, or 1.
    #[must_use]
    pub fn signum(self) -> Self {
        Self::new(self.x.signum(), self.y.signum(), self.z.signum())
    }

    /*
    /// Converts into an UltraViolet Vec3
    pub fn to_vec3i(&self) -> Vec3i {
        Vec3i::new(self.x, self.y, self.z)
    }
    */
}

impl From<(i32, i32, i32)> for Point3 {
    fn from(item: (i32, i32, i32)) -> Self {
        Self {
            x: item.0,
            y: item.1,
            z: item.2,
        }
    }
}

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
impl From<(f32, f32, f32)> for Point3 {
    fn from(item: (f32, f32, f32)) -> Self {
        Self {
            x: item.0 as i32,
            y: item.1 as i32,
            z: item.2 as i32,
        }
    }
}

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
impl From<Vec3> for Point3 {
    fn from(item: Vec3) -> Self {
        Self {
            x: item.x as i32,
            y: item.y as i32,
            z: item.z as i32,
        }
    }
}

/*
impl From<Vec3i> for Point3 {
    fn from(item: Vec3i) -> Self {
        Self {
            x: item.x,
            y: item.y,
            z: item.z,
        }
    }
}
*/

///////////////////////////////////////////////////////////////////////////////////////
// Overloads: We support basic point math

/// Support adding a point to a point
impl ops::Add<Point3> for Point3 {
    type Output = Point3;
    fn add(mut self, rhs: Point3) -> Point3 {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

/// Support adding an int to a point
impl ops::Add<i32> for Point3 {
    type Output = Point3;
    fn add(mut self, rhs: i32) -> Point3 {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self
    }
}

/// Support subtracting a point from a point
impl ops::Sub<Point3> for Point3 {
    type Output = Point3;
    fn sub(mut self, rhs: Point3) -> Point3 {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

/// Support subtracting an int from a point
impl ops::Sub<i32> for Point3 {
    type Output = Point3;
    fn sub(mut self, rhs: i32) -> Point3 {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self
    }
}

/// Support multiplying a point by a point
impl ops::Mul<Point3> for Point3 {
    type Output = Point3;
    fn mul(mut self, rhs: Point3) -> Point3 {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self
    }
}

/// Support multiplying a point by an int
impl ops::Mul<i32> for Point3 {
    type Output = Point3;
    fn mul(mut self, rhs: i32) -> Point3 {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

/// Support multiplying a point by an f32
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
impl ops::Mul<f32> for Point3 {
    type Output = Point3;
    fn mul(mut self, rhs: f32) -> Point3 {
        self.x = (self.x as f32 * rhs) as i32;
        self.y = (self.y as f32 * rhs) as i32;
        self.z = (self.z as f32 * rhs) as i32;
        self
    }
}

/// Support dividing a point by a point
impl ops::Div<Point3> for Point3 {
    type Output = Point3;
    fn div(mut self, rhs: Point3) -> Point3 {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self
    }
}

/// Support dividing a point by an int
impl ops::Div<i32> for Point3 {
    type Output = Point3;
    fn div(mut self, rhs: i32) -> Point3 {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self
    }
}

/// Support dividing a point by an f32
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
impl ops::Div<f32> for Point3 {
    type Output = Point3;
    fn div(mut self, rhs: f32) -> Point3 {
        self.x = (self.x as f32 / rhs) as i32;
        self.y = (self.y as f32 / rhs) as i32;
        self.z = (self.z as f32 / rhs) as i32;
        self
    }
}

// Support AddAssign for Point3
impl ops::AddAssign for Point3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

// Support SubAssign for Point3
impl ops::SubAssign for Point3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

// Support MulAssign for Point3
impl ops::MulAssign for Point3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

// Support DivAssign for Point3
impl ops::DivAssign for Point3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::Point3;

    #[test]
    fn new_point3() {
        let pt = Point3::new(1, 2, 3);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
        assert_eq!(pt.z, 3);
    }

    #[test]
    fn new_point3_defaults_to_zero_on_failed_conversion() {
        let pt = Point3::new(i64::MAX, 2_i64, 3_i64);
        assert_eq!(pt, Point3::constant(0, 2, 3));
    }

    #[test]
    fn constant_point3() {
        let pt = Point3::constant(1, 2, 3);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
        assert_eq!(pt.z, 3);
    }

    #[test]
    fn zero_point3() {
        assert_eq!(Point3::zero(), Point3::constant(0, 0, 0));
    }

    #[test]
    fn point3_from_tuple() {
        assert_eq!(Point3::from_tuple((1, 2, 3)), Point3::constant(1, 2, 3));
    }

    #[test]
    fn point3_from_generic_tuple() {
        assert_eq!(
            Point3::from_tuple((1_u16, 2_u16, 3_u16)),
            Point3::constant(1, 2, 3)
        );
    }

    #[test]
    fn point3_from_tuple_defaults_to_zero_on_failed_conversion() {
        let pt = Point3::from_tuple((1_i64, i64::MAX, 3_i64));
        assert_eq!(pt, Point3::constant(1, 0, 3));
    }

    #[test]
    fn point3_to_index() {
        let pt = Point3::new(2, 1, 3);
        assert_eq!(pt.to_index(10, 5), 162);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn point3_to_index_panics_for_negative_coordinates() {
        let pt = Point3::new(-1, 0, 0);
        let _ = pt.to_index(10, 5);
    }

    #[test]
    fn point3_to_tuple() {
        let pt = Point3::new(1, 2, 3);
        assert_eq!(pt.to_tuple(), (1, 2, 3));
    }

    #[test]
    fn point3_to_unsigned_tuple() {
        let pt = Point3::new(1, 2, 3);
        assert_eq!(pt.to_unsigned_tuple(), (1_usize, 2_usize, 3_usize));
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn point3_to_unsigned_tuple_panics_for_negative_coordinates() {
        let pt = Point3::new(1, -2, 3);
        let _ = pt.to_unsigned_tuple();
    }

    #[test]
    fn point3_to_vec3() {
        let vec = Point3::new(1, 2, 3).to_vec3();
        assert!((vec.x - 1.0).abs() < f32::EPSILON);
        assert!((vec.y - 2.0).abs() < f32::EPSILON);
        assert!((vec.z - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn point3_from_vec3() {
        let vec = ultraviolet::Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Point3::from_vec3(vec), Point3::from(vec));
    }

    #[test]
    fn point3_from_i32_tuple() {
        assert_eq!(Point3::from((1, 2, 3)), Point3::constant(1, 2, 3));
    }

    #[test]
    fn point3_from_f32_tuple() {
        assert_eq!(Point3::from((1.0, 2.0, 3.0)), Point3::constant(1, 2, 3));
    }

    #[test]
    fn point3_abs() {
        assert_eq!(Point3::new(-3, 2, -5).abs(), Point3::new(3, 2, 5));
        assert_eq!(
            Point3::new(i32::MIN, -3, i32::MIN).abs(),
            Point3::new(i32::MAX, 3, i32::MAX)
        );
    }

    #[test]
    fn point3_component_min() {
        assert_eq!(
            Point3::new(10, 3, -1).component_min(Point3::new(5, 8, 2)),
            Point3::new(5, 3, -1)
        );
    }

    #[test]
    fn point3_component_max() {
        assert_eq!(
            Point3::new(10, 3, -1).component_max(Point3::new(5, 8, 2)),
            Point3::new(10, 8, 2)
        );
    }

    #[test]
    fn point3_clamp() {
        assert_eq!(
            Point3::new(15, -3, 4).clamp(Point3::new(0, 0, 0), Point3::new(10, 10, 3)),
            Point3::new(10, 0, 3)
        );
    }

    #[test]
    fn point3_clamp_accepts_reversed_bounds() {
        assert_eq!(
            Point3::new(15, -3, 4).clamp(Point3::new(10, 10, 3), Point3::new(0, 0, 0)),
            Point3::new(10, 0, 3)
        );
    }

    #[test]
    fn point3_signum() {
        assert_eq!(Point3::new(3, -2, 0).signum(), Point3::new(1, -1, 0));
    }

    #[test]
    fn add_point_to_point3() {
        let pt = Point3::new(0, 0, 0);
        let p2 = pt + Point3::new(1, 2, 3);
        assert_eq!(p2.x, 1);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 3);
    }

    #[test]
    fn add_assign_point_to_point3() {
        let mut pt = Point3::new(0, 0, 0);
        pt += Point3::new(1, 2, 3);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
        assert_eq!(pt.z, 3);
    }

    #[test]
    fn add_point3_to_int() {
        let pt = Point3::new(0, 0, 0);
        let p2 = pt + 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 2);
    }

    #[test]
    fn sub_point3_to_point() {
        let pt = Point3::new(0, 0, 0);
        let p2 = pt - Point3::new(1, 2, 3);
        assert_eq!(p2.x, -1);
        assert_eq!(p2.y, -2);
        assert_eq!(p2.z, -3);
    }

    #[test]
    fn sub_assign_point3_to_point() {
        let mut pt = Point3::new(0, 0, 0);
        pt -= Point3::new(1, 2, 3);
        assert_eq!(pt.x, -1);
        assert_eq!(pt.y, -2);
        assert_eq!(pt.z, -3);
    }

    #[test]
    fn sub_point3_to_int() {
        let pt = Point3::new(0, 0, 0);
        let p2 = pt - 2;
        assert_eq!(p2.x, -2);
        assert_eq!(p2.y, -2);
        assert_eq!(p2.z, -2);
    }

    #[test]
    fn mul_point3_to_point() {
        let pt = Point3::new(1, 1, 1);
        let p2 = pt * Point3::new(1, 2, 4);
        assert_eq!(p2.x, 1);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 4);
    }

    #[test]
    fn mul_assign_point3_to_point() {
        let mut pt = Point3::new(1, 1, 1);
        pt *= Point3::new(1, 2, 4);
        assert_eq!(pt.x, 1);
        assert_eq!(pt.y, 2);
        assert_eq!(pt.z, 4);
    }

    #[test]
    fn mul_point3_to_int() {
        let pt = Point3::new(1, 1, 1);
        let p2 = pt * 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 2);
    }

    #[test]
    fn mul_point3_to_float() {
        let pt = Point3::new(1, 1, 1);
        let p2 = pt * 4.0;
        assert_eq!(p2.x, 4);
        assert_eq!(p2.y, 4);
        assert_eq!(p2.z, 4);
    }

    #[test]
    fn div_point3_to_point() {
        let pt = Point3::new(4, 4, 4);
        let p2 = pt / Point3::new(2, 4, 1);
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 1);
        assert_eq!(p2.z, 4);
    }

    #[test]
    fn div_assign_point3_to_point() {
        let mut pt = Point3::new(4, 4, 4);
        pt /= Point3::new(2, 4, 1);
        assert_eq!(pt.x, 2);
        assert_eq!(pt.y, 1);
        assert_eq!(pt.z, 4);
    }

    #[test]
    fn div_point3_to_int() {
        let pt = Point3::new(4, 4, 4);
        let p2 = pt / 2;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 2);
    }

    #[test]
    fn div_point3_to_float() {
        let pt = Point3::new(4, 4, 4);
        let p2 = pt / 2.0;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 2);
        assert_eq!(p2.z, 2);
    }
}
