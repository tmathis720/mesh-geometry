use criterion::{criterion_group, criterion_main, Criterion};
use mesh_geometry::{Point3, metrics::triangle_area};

fn bench_triangle(c: &mut Criterion) {
    let a = Point3::new(0.0, 0.0, 0.0);
    let b = Point3::new(1.0, 0.0, 0.0);
    let cpt = Point3::new(0.0, 1.0, 0.0);
    c.bench_function("triangle_area", |b| b.iter(|| triangle_area(a, b, cpt)));
}

criterion_group!(benches, bench_triangle);
criterion_main!(benches);
