//! Affine transforms and Jacobians for mesh-geometry.

use crate::{Float, Point3, Vec3};

/// 3×3 matrix + translation = affine transform in ℝ³.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform3<T: Float> {
    /// 3x3 matrix for the linear part of the transform
    pub m: [[T;3];3],
    /// Translation vector
    pub t: Vec3<T>,
}

impl<T: Float> Transform3<T> {
    /// Identity transform
    pub fn identity() -> Self {
        Transform3 {
            m: [
                [T::one(), T::zero(), T::zero()],
                [T::zero(), T::one(), T::zero()],
                [T::zero(), T::zero(), T::one()],
            ],
            t: Vec3::new(T::zero(), T::zero(), T::zero()),
        }
    }

    /// Construct from 3×3 matrix and translation
    pub fn new(m: [[T;3];3], t: Vec3<T>) -> Self {
        Transform3 { m, t }
    }

    /// Apply to a point: x ↦ M·x + t
    pub fn transform_point(&self, p: Point3<T>) -> Point3<T> {
        let x = self.m[0][0]*p.x + self.m[0][1]*p.y + self.m[0][2]*p.z + self.t.x;
        let y = self.m[1][0]*p.x + self.m[1][1]*p.y + self.m[1][2]*p.z + self.t.y;
        let z = self.m[2][0]*p.x + self.m[2][1]*p.y + self.m[2][2]*p.z + self.t.z;
        Point3::new(x,y,z)
    }

    /// Apply to a vector (no translation)
    pub fn transform_vec(&self, v: Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.m[0][0]*v.x + self.m[0][1]*v.y + self.m[0][2]*v.z,
            self.m[1][0]*v.x + self.m[1][1]*v.y + self.m[1][2]*v.z,
            self.m[2][0]*v.x + self.m[2][1]*v.y + self.m[2][2]*v.z,
        )
    }

    /// Invert (requires det(M) ≠ 0)
    pub fn inverse(&self) -> Option<Transform3<T>> {
        // compute adjugate/det of 3×3 matrix
        let m = &self.m;
        let det = 
            m[0][0]*(m[1][1]*m[2][2] - m[1][2]*m[2][1]) -
            m[0][1]*(m[1][0]*m[2][2] - m[1][2]*m[2][0]) +
            m[0][2]*(m[1][0]*m[2][1] - m[1][1]*m[2][0]);
        if det.abs() < T::epsilon() { return None; }
        let inv_det = T::one()/det;
        let adj = [
            [
                ( m[1][1]*m[2][2] - m[1][2]*m[2][1]) * inv_det,
               -(m[0][1]*m[2][2] - m[0][2]*m[2][1]) * inv_det,
                ( m[0][1]*m[1][2] - m[0][2]*m[1][1]) * inv_det,
            ],
            [
               -(m[1][0]*m[2][2] - m[1][2]*m[2][0]) * inv_det,
                ( m[0][0]*m[2][2] - m[0][2]*m[2][0]) * inv_det,
               -(m[0][0]*m[1][2] - m[0][2]*m[1][0]) * inv_det,
            ],
            [
                ( m[1][0]*m[2][1] - m[1][1]*m[2][0]) * inv_det,
               -(m[0][0]*m[2][1] - m[0][1]*m[2][0]) * inv_det,
                ( m[0][0]*m[1][1] - m[0][1]*m[1][0]) * inv_det,
            ],
        ];
        // Inverse translation: -M⁻¹·t
        let inv_t = {
            let t = &self.t;
            Vec3::new(
                -(adj[0][0]*t.x + adj[0][1]*t.y + adj[0][2]*t.z),
                -(adj[1][0]*t.x + adj[1][1]*t.y + adj[1][2]*t.z),
                -(adj[2][0]*t.x + adj[2][1]*t.y + adj[2][2]*t.z),
            )
        };
        Some(Transform3 { m: adj, t: inv_t })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point3, Vec3};

    #[test]
    fn transform_point_vec_roundtrip() {
        let m = [
            [2.0, 0.0, 0.0],
            [0.0, 3.0, 0.0],
            [0.0, 0.0, 4.0],
        ];
        let t = Vec3::new(1.0, -1.0, 2.0);
        let tf = Transform3::new(m, t);
        let p = Point3::new(1.0, 1.0, 1.0);
        let p2 = tf.transform_point(p);
        assert_eq!(p2, Point3::new(3.0, 2.0, 6.0));
    }
}
