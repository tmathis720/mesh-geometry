use crate::{Float, Point3};

/// Centroid of triangle = (A + B + C) / 3.
pub fn triangle_centroid<T: Float>(
    a: Point3<T>,
    b: Point3<T>,
    c: Point3<T>,
) -> Point3<T> {
    Point3::new(
        (a.x + b.x + c.x) / T::from(3.0).unwrap(),
        (a.y + b.y + c.y) / T::from(3.0).unwrap(),
        (a.z + b.z + c.z) / T::from(3.0).unwrap(),
    )
}

/// Centroid of planar quad = area-weighted average of two triangles.
pub fn quad_centroid<T: Float>(
    a: Point3<T>,
    b: Point3<T>,
    c: Point3<T>,
    d: Point3<T>,
) -> Point3<T> {
    let c1 = triangle_centroid(a,b,c);
    let c2 = triangle_centroid(a,c,d);
    let w1 = (b - a).cross(c - a).magnitude();
    let w2 = (c - a).cross(d - a).magnitude();
    Point3::new(
        (c1.x * w1 + c2.x * w2) / (w1 + w2),
        (c1.y * w1 + c2.y * w2) / (w1 + w2),
        (c1.z * w1 + c2.z * w2) / (w1 + w2),
    )
}

/// Centroid of tetrahedron = (A + B + C + D)/4
pub fn tetrahedron_centroid<T: Float>(
    a: Point3<T>,
    b: Point3<T>,
    c: Point3<T>,
    d: Point3<T>,
) -> Point3<T> {
    let inv4 = T::from(0.25).unwrap();
    Point3::new(
        (a.x + b.x + c.x + d.x) * inv4,
        (a.y + b.y + c.y + d.y) * inv4,
        (a.z + b.z + c.z + d.z) * inv4,
    )
}

/// Centroid of a hexahedron (assumed affine) = average of its 8 vertices.
pub fn hexahedron_centroid<T: Float>(verts: [Point3<T>;8]) -> Point3<T> {
    let sum = verts.iter().fold(
        Point3::new(T::zero(), T::zero(), T::zero()),
        |acc, p| Point3::new(acc.x + p.x, acc.y + p.y, acc.z + p.z)
    );
    let inv8 = T::from(0.125).unwrap();
    Point3::new(sum.x*inv8, sum.y*inv8, sum.z*inv8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point3;

    #[test]
    fn triangle_centroid_correct() {
        let a=Point3::new(0.0,0.0,0.0);
        let b=Point3::new(1.0,0.0,0.0);
        let c=Point3::new(0.0,1.0,0.0);
        let cen = triangle_centroid(a,b,c);
        assert_eq!(cen, Point3::new(1.0/3.0,1.0/3.0,0.0));
    }
}
