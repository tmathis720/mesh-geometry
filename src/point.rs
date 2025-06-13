//! Point2/Point3 definitions for mesh-geometry.

use core::ops::{Add, Sub};
use crate::vec::{Vec2, Vec3};
use crate::Float;

/// A 2D point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2<T: Float> {
    /// X coordinate
    pub x: T,
    /// Y coordinate
    pub y: T,
}

impl<T: Float> Point2<T> {
    /// Create a new Point2
    pub fn new(x: T, y: T) -> Self { Self { x, y } }
}

// vector difference = Vec2
impl<T: Float> Sub for Point2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Point2<T>) -> Vec2<T> {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// translate point by Vec2
impl<T: Float> Add<Vec2<T>> for Point2<T> {
    type Output = Point2<T>;
    fn add(self, v: Vec2<T>) -> Point2<T> {
        Point2::new(self.x + v.x, self.y + v.y)
    }
}

/// A 3D point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3<T: Float> {
    /// X coordinate
    pub x: T,
    /// Y coordinate
    pub y: T,
    /// Z coordinate
    pub z: T,
}

impl<T: Float> Point3<T> {
    /// Create a new Point3
    pub fn new(x: T, y: T, z: T) -> Self { Self { x, y, z } }
}

impl<T: Float> Sub for Point3<T> {
    type Output = Vec3<T>;
    fn sub(self, rhs: Point3<T>) -> Vec3<T> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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
    use crate::{Point2, Point3, Vec3};

    #[test]
    fn roundtrip_point3_vector_ops() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let q = Point3::new(4.0, 6.0, 5.0);
        let v = q - p;
        // subtraction gives correct Vec3
        assert_eq!(v, Vec3::new(3.0, 4.0, 2.0));
        // adding it back recovers q
        assert_eq!(p + v, q);
    }

    #[test]
    fn point2_sub_add_consistency() {
        let p = Point2::new(0.5, 1.5);
        let q = Point2::new(-1.0,  4.0);
        let v = q - p;
        assert_eq!(p + v, q);
    }
}
