//! Point2/Point3 definitions for mesh-geometry.

use core::ops::{Add, Sub};
use crate::{Vec2, Vec3, Float};

/// 2D point with generic float.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Point2<T> {
    /// Construct a new Point2
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// 3D point with generic float.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Point3<T> {
    /// Construct a new Point3
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

// Vector difference yields a Vec2/Vec3
impl<T: Float> Sub for Point2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Point2<T>) -> Vec2<T> {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Float> Sub for Point3<T> {
    type Output = Vec3<T>;
    fn sub(self, rhs: Point3<T>) -> Vec3<T> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// Translating point by vector
impl<T: Float> Add<Vec2<T>> for Point2<T> {
    type Output = Point2<T>;
    fn add(self, v: Vec2<T>) -> Point2<T> {
        Point2::new(self.x + v.x, self.y + v.y)
    }
}

impl<T: Float> Add<Vec3<T>> for Point3<T> {
    type Output = Point3<T>;
    fn add(self, v: Vec3<T>) -> Point3<T> {
        Point3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point2, Point3, Vec2, Vec3, Float};

    #[test]
    fn point2_sub_add_roundtrip() {
        let p = Point2::new(1.0, 2.0);
        let q = Point2::new(4.0, 6.0);
        let v = q - p;
        assert_eq!(v, Vec2::new(3.0, 4.0));
        assert_eq!(p + v, q);
    }

    #[test]
    fn point3_sub_add_roundtrip() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let q = Point3::new(4.0, 6.0, 3.0);
        let v = q - p;
        assert_eq!(v, Vec3::new(3.0, 4.0, 0.0));
        assert_eq!(p + v, q);
    }
}
