//! Geometry queries: point-in-polygon, ray-triangle, distances, etc.

pub mod point_in_polygon;
pub mod ray_triangle;
/// Distance queries for points and cells.
pub mod distance;

// Re-exports:
pub use point_in_polygon::point_in_polygon;
pub use ray_triangle::{ray_intersects_triangle, Ray};
pub use distance::{point_to_segment_distance, point_to_triangle_distance, point_to_polygon_distance};
