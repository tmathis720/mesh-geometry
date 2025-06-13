//! Face normals and projected area for mesh-geometry.

use crate::{Float, Point3, Vec3};

/// Compute un-normalized face normal for planar polygon verts in order.
/// For triangles, returns the cross product of two edges.
/// For n>3, returns the sum of cross(edges[i], edges[i+1]).
pub fn face_normal<T: Float>(verts: &[Point3<T>]) -> Vec3<T> {
    match verts.len() {
        3 => {
            let ab = verts[1] - verts[0];
            let ac = verts[2] - verts[0];
            ab.cross(ac)
        }
        n if n > 3 => {
            let mut nrm = Vec3::new(T::zero(), T::zero(), T::zero());
            for i in 0..verts.len() {
                let a = verts[i];
                let b = verts[(i+1) % verts.len()];
                nrm = nrm + (b - a).cross(verts[(i+2)%verts.len()] - a);
            }
            nrm
        }
        _ => Vec3::new(T::zero(), T::zero(), T::zero()),
    }
}

/// Projected (planar) area = ½‖normal‖.
pub fn projected_area<T: Float>(verts: &[Point3<T>]) -> T {
    face_normal(verts).magnitude() * T::from(0.5).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point3;

    #[test]
    fn projected_area_triangle() {
        let tri = [
            Point3::new(0.0_f64, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ];
        let area = projected_area(&tri);
        if (area - 0.5).abs() >= 1e-8 {
            eprintln!("projected_area_triangle: got {} (expected 0.5)", area);
        }
        assert!((area - 0.5).abs() < 1e-8);
    }
}
