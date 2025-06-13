//! Vec2/Vec3 definitions and operations for mesh-geometry.

use core::ops::{Add, Sub, Mul};
use crate::Float;

/// A 2D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T: Float> {
    /// X component
    pub x: T,
    /// Y component
    pub y: T,
}

impl<T: Float> Vec2<T> {
    /// Create a new Vec2
    pub fn new(x: T, y: T) -> Self { Self { x, y } }

    /// Dot product
    pub fn dot(self, other: Vec2<T>) -> T {
        self.x * other.x + self.y * other.y
    }

    /// Length (magnitude)
    pub fn magnitude(self) -> T {
        self.dot(self).sqrt()
    }

    /// Cross‐scalar (2D “pseudo‐cross” = scalar)
    pub fn cross(self, other: Vec2<T>) -> T {
        self.x * other.y - self.y * other.x
    }
}
// Vector + Vector
impl<T: Float> Add for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
// Vector - Vector
impl<T: Float> Sub for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
// Scalar multiplication
impl<T: Float> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: T) -> Vec2<T> {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}
/// A 3D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T: Float> {
    /// X component
    pub x: T,
    /// Y component
    pub y: T,
    /// Z component
    pub z: T,
}

impl<T: Float> Vec3<T> {
    /// Create a new Vec3
    pub fn new(x: T, y: T, z: T) -> Self { Self { x, y, z } }

    /// Dot product
    pub fn dot(self, other: Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product
    pub fn cross(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Length (magnitude)
    pub fn magnitude(self) -> T {
        self.dot(self).sqrt()
    }
}
impl<T: Float> Add for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl<T: Float> Sub for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vec2, Vec3};

    #[test]
    fn vec2_dot_and_cross() {
        let u = Vec2::new(1.0, 0.0);
        let v = Vec2::new(0.0, 1.0);
        assert_eq!(u.dot(v), 0.0);
        // 2D cross returns +1 for (1,0)x(0,1)
        assert_eq!(u.cross(v), 1.0);
    }

    #[test]
    fn vec3_dot_and_cross() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(i.dot(j), 0.0);
        assert_eq!(i.cross(j), k);
    }

    #[test]
    fn vec_scaling_and_magnitude() {
        let v = Vec3::new(2.0, -3.0, 6.0);
        let scaled = v * 2.0;
        assert_eq!(scaled, Vec3::new(4.0, -6.0, 12.0));
        let len = (4.0_f64*4.0 + 6.0*6.0 + 12.0*12.0).sqrt();
        assert!((scaled.magnitude() - len).abs() < 1e-12);
    }
}
