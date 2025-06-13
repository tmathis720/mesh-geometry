//! Vec2/Vec3 definitions and operations for mesh-geometry.

use core::ops::{Add, Sub, Mul};
use crate::Float;

/// 2D vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Vec2<T> {
    /// Construct a new Vec2
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Dot product
    pub fn dot(self, other: Vec2<T>) -> T {
        self.x * other.x + self.y * other.y
    }

    /// Magnitude (length)
    pub fn magnitude(self) -> T {
        self.dot(self).sqrt()
    }

    /// Normalize to unit length
    pub fn normalize(self) -> Self {
        let m = self.magnitude();
        Vec2::new(self.x / m, self.y / m)
    }
}

// Basic ops
impl<T: Float> Add for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Float> Sub for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Float> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: T) -> Vec2<T> {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

/// 3D vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vec3<T> {
    /// Construct a new Vec3
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

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

    /// Magnitude (length)
    pub fn magnitude(self) -> T {
        self.dot(self).sqrt()
    }

    /// Normalize to unit length
    pub fn normalize(self) -> Self {
        let m = self.magnitude();
        Vec3::new(self.x / m, self.y / m, self.z / m)
    }
}

// Basic ops
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
    use crate::{Vec2, Vec3, Float};

    #[test]
    fn vec2_dot_identity() {
        let v = Vec2::new(3.0_f64, 4.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn vec3_dot_cross_identity() {
        let v = Vec3::new(1.0, 0.0, 0.0);
        let w = Vec3::new(0.0, 1.0, 0.0);
        let cross = v.cross(w);
        assert_eq!(cross, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(cross.magnitude(), 1.0);
    }
}
