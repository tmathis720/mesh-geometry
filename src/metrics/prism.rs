//! Prism metrics: volume and centroid for extruded prisms.
//!
//! This module provides functions to calculate the volume and centroid of
//! straight prisms (also known as cylindrical prisms or right prisms).
//!
//! # Examples
//!
//! ```rust
//! use mesh_geometry::{prism_volume, Point2};
//! let base = [
//!     Point2::new(0.0_f64, 0.0_f64),
//!     Point2::new(1.0_f64, 0.0_f64),
//!     Point2::new(0.0_f64, 1.0_f64),
//! ];
//! let vol = prism_volume(&base, 2.0_f64);
//! assert!((vol - 1.0_f64).abs() < 1e-8);
//! ```

use crate::{Float, Point2, Point3};
use crate::metrics::{polygon_area, polygon_centroid};

/// Volume of a straight prism: base_area × height.
pub fn prism_volume<T: Float>(base: &[Point2<T>], height: T) -> T {
    polygon_area(base) * height
}

/// Centroid of a prism: (Cx, Cy, Cz) where (Cx,Cy) = base centroid, Cz = height/2.
pub fn prism_centroid<T: Float>(
    base: &[Point2<T>],
    height: T,
    z0: T,   // base plane z-coordinate
) -> Point3<T> {
    let c2 = polygon_centroid(base);
    Point3::new(c2.x, c2.y, z0 + height * T::from(0.5).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point2;

    #[test]
    fn triangular_prism_unit_height() {
        // base = right triangle area=0.5
        let base = [Point2::new(0.0_f64,0.0_f64), Point2::new(1.0_f64,0.0_f64), Point2::new(0.0_f64,1.0_f64)];
        let vol = prism_volume::<f64>(&base, 2.0_f64);
        assert!((vol - 1.0_f64).abs() < 1e-8);
        let cen = prism_centroid::<f64>(&base, 2.0_f64, 0.0_f64);
        // base centroid at (1/3,1/3), z=1.0
        assert!((cen.x - 1.0_f64/3.0_f64).abs() < 1e-8);
        assert!((cen.z - 1.0_f64).abs() < 1e-8);
    }

    #[test]
    fn hexagonal_prism() {
        // regular hexagon radius=1
        let hex: Vec<_> = (0..6)
            .map(|i| {
                let theta = (i as f64) * std::f64::consts::PI / 3.0;
                Point2::new(theta.cos(), theta.sin())
            })
            .collect();
        let h = 5.0_f64;
        let vol = prism_volume::<f64>(&hex, h);
        // expected = hex_area * h
        let expected_area = 3.0_f64 * 3_f64.sqrt() / 2.0_f64;
        assert!((vol - expected_area * 5.0_f64).abs() < 1e-6);
    }
}
