# mesh-geometry

[![crates.io](https://img.shields.io/crates/v/mesh-geometry.svg)](https://crates.io/crates/mesh-geometry)
[![docs.rs](https://docs.rs/mesh-geometry/badge.svg)](https://docs.rs/mesh-geometry)
[![CI](https://github.com/your-org/mesh-geometry/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/mesh-geometry/actions)

High-performance, no_std-compatible geometry utilities for coastal/ocean FVM meshes.

## Features
- Core types: `Point2`, `Point3`, `Vec2`, `Vec3` with arithmetic, dot/cross, and conversion utilities.
- Cell metrics: triangle/quad area, centroids, tetrahedron/hexahedron volume, face normals, projected area.
- Geometry queries: point-in-polygon, ray-triangle intersection, point-to-cell distance.
- Advanced utilities: Jacobians, AABB, 3D affine transforms.
- `no_std` compatible (default: `std` enabled).
- Comprehensive documentation and examples.

## Installation

```sh
cargo add mesh-geometry
```

## Quickstart

```rust
use mesh_geometry::Point2;
let p = Point2::new(0.0, 1.0);
```

## Examples

Compute the volume of a tetrahedron:

```sh
cargo run --example compute_cell_volumes
```

```rust
use mesh_geometry::{Point3, metrics::volume::tetrahedron_volume};

let a = Point3::new(0.0, 0.0, 0.0);
let b = Point3::new(1.0, 0.0, 0.0);
let c = Point3::new(0.0, 1.0, 0.0);
let d = Point3::new(0.0, 0.0, 1.0);
let vol = tetrahedron_volume(a, b, c, d);
assert!((vol - 1.0/6.0).abs() < 1e-12);
```

See more in [examples/](examples/).

## Documentation
- [docs.rs/mesh-geometry](https://docs.rs/mesh-geometry)

## CI & Contributing
- CI: [GitHub Actions](https://github.com/your-org/mesh-geometry/actions)
- Issues and PRs welcome!

## License
MIT OR Apache-2.0
