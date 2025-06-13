//! Cell metrics: area, centroid, volume, normals, etc.

/// Centroid and center-of-mass calculations.
pub mod centroid;
/// Volume calculations for 3D cells.
pub mod volume;
pub mod area;
pub mod normals;

// Re-export for convenience:
pub use area::{triangle_area, quad_area};
pub use centroid::{triangle_centroid, quad_centroid, tetrahedron_centroid, hexahedron_centroid};
pub use volume::{tetrahedron_volume, hexahedron_volume};
pub use normals::{face_normal, projected_area};
