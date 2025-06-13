#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

//! # mesh-geometry
//!
//! High-performance, no_std-compatible geometry primitives and metrics for spatially discrete meshes.
//!
//! ## Features
//! - Core types: `Point2`, `Point3`, `Vec2`, `Vec3` with arithmetic, dot/cross, and conversion utilities.
//! - Cell metrics: triangle/quad area, centroids, tetrahedron/hexahedron volume, face normals, projected area.
//! - Geometry queries: point-in-polygon, ray-triangle intersection, point-to-cell distance.
//! - Advanced utilities: Jacobians, AABB, 3D affine transforms.
//! - `no_std` compatible (default: `std` enabled).
//! - Comprehensive documentation and examples.
//!
//! ## Example: Compute Cell Volumes
//!
//! ```rust
//! use mesh_geometry::{Point3};
//! use mesh_geometry::tetrahedron_volume;
//!
//! let a = Point3::new(0.0, 0.0, 0.0);
//! let b = Point3::new(1.0, 0.0, 0.0);
//! let c = Point3::new(0.0, 1.0, 0.0);
//! let d = Point3::new(0.0, 0.0, 1.0);
//! let vol = tetrahedron_volume(a, b, c, d);
//! assert!((vol - 1.0_f64/6.0_f64).abs() < 1e-12_f64);
//! ```
//!
//! More examples: see [examples/](https://github.com/tmathis720/mesh-geometry/tree/main/examples) and the [README](https://github.com/your-org/mesh-geometry#examples).
//!
//! ## Documentation
//! - [docs.rs/mesh-geometry](https://docs.rs/mesh-geometry)
//!
//! ## License
//! MIT

/// Prelude for ergonomic imports (core types, traits, and utilities).
pub mod prelude;
pub use prelude::*;

/// Float trait alias for generic float support.
pub mod float;
pub use float::Float;

/// 2D/3D point types and constructors.
pub mod point;

/// 2D/3D vector types and operations.
pub mod vec;

/// Cell metrics: area, centroid, volume, normals, polygon, prism
pub mod metrics;
pub use metrics::{
    triangle_area, quad_area,
    triangle_centroid, quad_centroid, tetrahedron_centroid, hexahedron_centroid,
    tetrahedron_volume, hexahedron_volume,
    face_normal, projected_area,
    polygon_area, polygon_centroid,
    prism_volume, prism_centroid,
};

/// Geometry queries: point-in-polygon, ray-triangle, distance, etc.
pub mod queries;

/// 3D affine transforms.
pub mod transforms;

/// Advanced utilities: Jacobians, AABB, etc.
pub mod utils;
pub use utils::{jacobian, aabb};
