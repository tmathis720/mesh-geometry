//! Ray-triangle intersection queries for mesh-geometry.

use crate::{Float, Point3, Vec3};

/// A 3D ray: origin + t·dir, with dir not necessarily normalized.
#[derive(Debug, Clone, Copy)]
pub struct Ray<T: Float> {
    /// Ray origin point
    pub origin: Point3<T>,
    /// Ray direction vector (not necessarily normalized)
    pub dir: Vec3<T>,
}

/// If the ray intersects the triangle (a,b,c), returns `Some((t,u,v))`
/// where intersection = origin + t·dir, and (u,v) are barycentric coords.
/// Otherwise returns `None`.
pub fn ray_intersects_triangle<T: Float>(
    ray: Ray<T>,
    a: Point3<T>,
    b: Point3<T>,
    c: Point3<T>,
) -> Option<(T, T, T)> {
    let epsilon = T::from(1e-8).unwrap();
    let edge1 = b - a;
    let edge2 = c - a;
    let pvec = ray.dir.cross(edge2);
    let det = edge1.dot(pvec);
    if det.abs() < epsilon {
        return None; // parallel
    }
    let inv_det = det.recip();
    let tvec = ray.origin - a;
    let u = tvec.dot(pvec) * inv_det;
    if u < T::zero() || u > T::one() {
        return None;
    }
    let qvec = tvec.cross(edge1);
    let v = ray.dir.dot(qvec) * inv_det;
    if v < T::zero() || u + v > T::one() {
        return None;
    }
    let t = edge2.dot(qvec) * inv_det;
    if t > epsilon {
        Some((t, u, v))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point3, Vec3};

    #[test]
    fn hit_center() {
        let ray = Ray {
            origin: Point3::new(0.1_f64, 0.1, -1.0),
            dir: Vec3::new(0.0, 0.0, 1.0),
        };
        let a = Point3::new(0.0, 0.0, 0.0);
        let b = Point3::new(1.0, 0.0, 0.0);
        let c = Point3::new(0.0, 1.0, 0.0);
        let hit = ray_intersects_triangle(ray, a, b, c).unwrap();
        assert!((hit.0 - 1.0).abs() < 1e-8);
        // hit.1, hit.2 are barycentric coords; should sum < 1
        assert!(hit.1 + hit.2 < 1.0);
    }

    #[test]
    fn miss_ray() {
        let ray = Ray {
            origin: Point3::new(2.0, 2.0, -1.0),
            dir: Vec3::new(0.0, 0.0, 1.0),
        };
        assert!(ray_intersects_triangle(
            ray,
            Point3::new(0., 0., 0.),
            Point3::new(1., 0., 0.),
            Point3::new(0., 1., 0.)
        )
        .is_none());
    }
}
