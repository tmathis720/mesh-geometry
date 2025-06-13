use crate::{Float, Point3};

/// Signed volume of tetrahedron (A,B,C,D) = |dot((B−A)×(C−A), D−A)|/6
pub fn tetrahedron_volume<T: Float>(
    a: Point3<T>, b: Point3<T>, c: Point3<T>, d: Point3<T>
) -> T {
    let v = (b - a).cross(c - a).dot(d - a).abs();
    v * T::from(1.0/6.0).unwrap()
}

/// Volume of axis-aligned hexahedron (brick) with 8 verts:
/// assume hexa is affine so volume = sum of five tetrahedra, or
/// simpler: |det(J)| where J from reference cube to physical.  
/// For now, approximate as (face_area_xy_min × thickness).
pub fn hexahedron_volume<T: Float>(verts: [Point3<T>;8]) -> T {
    // simple split into 5 tets (example pattern)
    tetrahedron_volume(verts[0], verts[1], verts[3], verts[4])
      + tetrahedron_volume(verts[1], verts[2], verts[3], verts[6])
      + tetrahedron_volume(verts[1], verts[5], verts[4], verts[6])
      + tetrahedron_volume(verts[3], verts[7], verts[4], verts[6])
      + tetrahedron_volume(verts[1], verts[3], verts[4], verts[6])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point3;

    #[test]
    fn tet_unit_volume() {
        let a = Point3::new(0.0_f64, 0.0, 0.0);
        let b = Point3::new(1.0, 0.0, 0.0);
        let c = Point3::new(0.0, 1.0, 0.0);
        let d = Point3::new(0.0, 0.0, 1.0);
        let vol = tetrahedron_volume(a, b, c, d);
        assert!((vol - 1.0/6.0).abs() < 1e-8);
    }
}
