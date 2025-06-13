# mesh-geometry

High-performance geometry utilities for coastal/ocean FVM meshes.

## Quickstart

```sh
cargo add mesh-geometry
```

```rust
use mesh_geometry::Point2;
let p = Point2::new(0.0, 1.0);
```

## Feature Flags
- `std` (default): enables standard library support
- `no_std`: for embedded/no_std contexts

## Documentation
- [docs.rs/mesh-geometry](https://docs.rs/mesh-geometry)

## License
MIT OR Apache-2.0
