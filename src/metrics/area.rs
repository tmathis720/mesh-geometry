//! Area/volume functions for mesh-geometry.

use crate::{Float, Point3};

/// Signed area of triangle (A, B, C).
pub fn triangle_area<T: Float>(a: Point3<T>, b: Point3<T>, c: Point3<T>) -> T {
    let ab = b - a;
    let ac = c - a;
    let cross = ab.cross(ac);
    cross.magnitude() * T::from(0.5).unwrap()
}

/// Area of a planar quadrilateral (A, B, C, D), assumed convex.
/// Computed as sum of two triangle areas: (A,B,C) + (A,C,D).
pub fn quad_area<T: Float>(
    a: Point3<T>,
    b: Point3<T>,
    c: Point3<T>,
    d: Point3<T>,
) -> T {
    triangle_area(a, b, c) + triangle_area(a, c, d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point3;

    #[test]
    fn triangle_area_unit() {
        let a = Point3::new(0.0_f64, 0.0, 0.0);
        let b = Point3::new(1.0, 0.0, 0.0);
        let c = Point3::new(0.0, 1.0, 0.0);
        let area = triangle_area(a, b, c);
        assert!((area - 0.5).abs() < 1e-8);
    }

    #[test]
    fn quad_area_square() {
        let a = Point3::new(0.0_f64, 0.0, 0.0);
        let b = Point3::new(1.0, 0.0, 0.0);
        let c = Point3::new(1.0, 1.0, 0.0);
        let d = Point3::new(0.0, 1.0, 0.0);
        let area = quad_area(a, b, c, d);
        assert!((area - 1.0).abs() < 1e-8);
    }
}
