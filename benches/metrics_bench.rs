use criterion::{criterion_group, criterion_main, Criterion};
use mesh_geometry::metrics::triangle_area;
#[cfg(feature = "bench")]
use nalgebra::Point3;

fn bench_triangle_custom(c: &mut Criterion) {
    let a = mesh_geometry::Point3::new(0.0,0.0,0.0);
    let b = mesh_geometry::Point3::new(1.0,0.0,0.0);
    let cc= mesh_geometry::Point3::new(0.0,1.0,0.0);
    c.bench_function("custom triangle_area", |bch| bch.iter(|| triangle_area(a,b,cc)));
}

#[cfg(feature = "bench")]
fn bench_triangle_nalgebra(c: &mut Criterion) {
    let a = Point3::new(0.0,0.0,0.0);
    let b = Point3::new(1.0,0.0,0.0);
    let cpt= Point3::new(0.0,1.0,0.0);
    c.bench_function("nalgebra triangle_area", |bch| {
        bch.iter(|| {
            let ab = b - a;
            let ac = cpt - a;
            let cross = ab.cross(&ac);
            cross.norm()/2.0
        })
    });
}

#[cfg(feature = "bench")]
criterion_group!(benches, bench_triangle_custom, bench_triangle_nalgebra);
#[cfg(not(feature = "bench"))]
criterion_group!(benches, bench_triangle_custom);
criterion_main!(benches);
