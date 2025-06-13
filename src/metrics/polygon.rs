//! Polygon metrics: area and centroid for arbitrary planar polygons.
//!
//! This module provides functions to calculate the signed area and centroid of
//! arbitrary planar polygons using the shoelace formula. The polygon vertices
//! should be ordered either in a clockwise or counter-clockwise direction, and
//!
//! # Examples
//!
//! ```rust
//! use mesh_geometry::{polygon_area, Point2};
//! let tri = [
//!     Point2::new(0.0_f64, 0.0_f64),
//!     Point2::new(2.0_f64, 0.0_f64),
//!     Point2::new(1.0_f64, 1.7320508_f64),
//! ];
//! let area = polygon_area(&tri);
//! assert!((area - 1.7320508_f64).abs() < 1e-6);
//! ```

use crate::{Float, Point2};

/// Signed area of an arbitrary planar polygon (shoelace formula).
/// Vertices ordered CCW or CW, first and last need not repeat.
pub fn polygon_area<T: Float>(verts: &[Point2<T>]) -> T {
    let n = verts.len();
    assert!(n >= 3, "polygon_area requires ≥3 vertices");
    let mut sum = T::zero();
    for i in 0..n {
        let a = verts[i];
        let b = verts[(i + 1) % n];
        sum = sum + (a.x * b.y - b.x * a.y);
    }
    (sum * T::from(0.5).unwrap()).abs()
}

/// Centroid of a planar polygon: (Cx, Cy) = (1/(6A)) Σ (xi + xi+1)(xi yi+1 − xi+1 yi)
pub fn polygon_centroid<T: Float>(verts: &[Point2<T>]) -> Point2<T> {
    let n = verts.len();
    assert!(n >= 3, "polygon_centroid requires ≥3 vertices");
    let mut a_twice = T::zero();
    let mut cx = T::zero();
    let mut cy = T::zero();
    for i in 0..n {
        let xi = verts[i].x;
        let yi = verts[i].y;
        let xj = verts[(i + 1) % n].x;
        let yj = verts[(i + 1) % n].y;
        let cross = xi * yj - xj * yi;
        a_twice = a_twice + cross;
        cx = cx + (xi + xj) * cross;
        cy = cy + (yi + yj) * cross;
    }
    let a = a_twice * T::from(0.5).unwrap();
    let factor = T::one() / (T::from(6.0).unwrap() * a);
    Point2::new(cx * factor, cy * factor)
}
