//! Point-in-polygon queries for mesh-geometry.

use crate::{Float, Point2};

/// Returns true if `pt` is inside the planar polygon `poly` (winding-number).
/// Assumes `poly` is closed (first != last) and simple (non-self-intersecting).
pub fn point_in_polygon<T: Float>(pt: Point2<T>, poly: &[Point2<T>]) -> bool {
    let mut winding: i32 = 0;
    let n = poly.len();
    for i in 0..n {
        let p1 = poly[i];
        let p2 = poly[(i + 1) % n];
        if p1.y <= pt.y {
            if p2.y > pt.y && is_left(p1, p2, pt) > T::zero() {
                winding += 1;
            }
        } else {
            if p2.y <= pt.y && is_left(p1, p2, pt) < T::zero() {
                winding -= 1;
            }
        }
    }
    winding != 0
}

#[inline]
fn is_left<T: Float>(a: Point2<T>, b: Point2<T>, c: Point2<T>) -> T {
    (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point2;

    #[test]
    fn inside_triangle() {
        let tri = [
            Point2::new(0.0, 0.0),
            Point2::new(5.0, 0.0),
            Point2::new(0.0, 5.0),
        ];
        assert!(point_in_polygon(Point2::new(1.0,1.0), &tri));
        assert!(!point_in_polygon(Point2::new(5.0,5.0), &tri));
    }

    #[test]
    fn inside_square_and_hole() {
        let square = [
            Point2::new(-1.0, -1.0),
            Point2::new( 1.0, -1.0),
            Point2::new( 1.0,  1.0),
            Point2::new(-1.0,  1.0),
        ];
        assert!(point_in_polygon(Point2::new(0.0,0.0), &square));
        assert!(!point_in_polygon(Point2::new(2.0,0.0), &square));
    }
}
