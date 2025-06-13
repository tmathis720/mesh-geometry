use mesh_geometry::{polygon_area, prism_volume, Point2};

fn main() {
    // compute area of one cell in [3.3.3.3.6] tiling:
    // 4 triangles and 1 hexagon around a vertex; here just hexagon:
    let hex: Vec<_> = (0..6)
        .map(|i| {
            let theta = i as f64 * std::f64::consts::PI/3.0;
            Point2::new(theta.cos(), theta.sin())
        })
        .collect();
    println!("Hexagon area = {}", polygon_area(&hex));

    // extrude by height=1.0
    println!("Hexagonal prism vol = {}", prism_volume(&hex, 1.0));
}
