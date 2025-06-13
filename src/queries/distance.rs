use crate::{Float, Point2, Point3};

/// Distance from point `p` to segment [a,b] in 2D.
pub fn point_to_segment_distance<T: Float>(
    p: Point2<T>,
    a: Point2<T>,
    b: Point2<T>,
) -> T {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.dot(ab))
        .max(T::zero())
        .min(T::one());
    let proj = a + ab * t;
    (p - proj).magnitude()
}

/// Distance from point `p` to triangle (a,b,c) in 3D.
pub fn point_to_triangle_distance<T: Float>(
    p: Point3<T>,
    a: Point3<T>,
    b: Point3<T>,
    c: Point3<T>,
) -> T {
    // 1) project onto plane
    let n = (b - a).cross(c - a);
    let norm = n.magnitude();
    let normal = if norm != T::zero() {
        n * (T::one() / norm)
    } else {
        n
    };
    let d = (p - a).dot(normal);
    let proj = Point3::new(p.x - normal.x * d, p.y - normal.y * d, p.z - normal.z * d);
    // 2) if proj inside tri, return |d|, otherwise min to edges
    use crate::queries::point_in_polygon;
    let tri2d = [a, b, c].map(|v| Point2::new(v.x, v.y));
    let proj2d = Point2::new(proj.x, proj.y);
    if point_in_polygon(proj2d, &tri2d) {
        d.abs()
    } else {
        let d_ab = point_to_segment_distance(proj2d, tri2d[0], tri2d[1]);
        let d_bc = point_to_segment_distance(proj2d, tri2d[1], tri2d[2]);
        let d_ca = point_to_segment_distance(proj2d, tri2d[2], tri2d[0]);
        let min2d = d_ab.min(d_bc).min(d_ca);
        (d.abs().powi(2) + min2d.powi(2)).sqrt()
    }
}

/// Distance from point `p` to polygon in 2D (zero if inside).
pub fn point_to_polygon_distance<T: Float>(
    p: Point2<T>,
    poly: &[Point2<T>],
) -> T {
    if crate::queries::point_in_polygon(p, poly) {
        T::zero()
    } else {
        poly.windows(2)
            .map(|w| point_to_segment_distance(p, w[0], w[1]))
            .chain(std::iter::once(point_to_segment_distance(p, poly[poly.len()-1], poly[0])))
            .fold(T::infinity(), |a, b| a.min(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point2;

    #[test]
    fn seg_dist() {
        let a = Point2::new(0.0_f64,0.0);
        let b = Point2::new(1.0,0.0);
        let d = point_to_segment_distance(Point2::new(0.5,1.0), a, b);
        assert!((d - 1.0).abs() < 1e-8);
    }

    #[test]
    fn poly_dist_inside() {
        let square = [
            Point2::new(-1.0, -1.0),
            Point2::new( 1.0, -1.0),
            Point2::new( 1.0,  1.0),
            Point2::new(-1.0,  1.0),
        ];
        assert_eq!(point_to_polygon_distance(Point2::new(0.0,0.0), &square), 0.0);
    }

    #[test]
    fn poly_dist_outside() {
        let square = [
            Point2::new(-1.0, -1.0),
            Point2::new( 1.0, -1.0),
            Point2::new( 1.0,  1.0),
            Point2::new(-1.0,  1.0),
        ];
        let d = point_to_polygon_distance(Point2::new(2.0,0.0), &square);
        assert!((d - 1.0_f64).abs() < 1e-8);
    }
}
