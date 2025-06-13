//! Example: Compute cell volumes for a simple mesh.
use mesh_geometry::{Point3, metrics::volume::tetrahedron_volume};

fn main() {
    // Vertices of a unit tetrahedron
    let a = Point3::new(0.0, 0.0, 0.0);
    let b = Point3::new(1.0, 0.0, 0.0);
    let c = Point3::new(0.0, 1.0, 0.0);
    let d = Point3::new(0.0, 0.0, 1.0);
    let vol = tetrahedron_volume(a, b, c, d);
    println!("Tetrahedron volume: {}", vol);
    assert!((vol - 1.0/6.0_f32).abs() < 1e-12_f32);
}
