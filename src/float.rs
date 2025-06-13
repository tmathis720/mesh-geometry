//! Float trait alias for mesh-geometry
use num_traits::Float as _NumFloat;

/// Our floating‐point bound (f32 or f64), must be Copy.
pub trait Float: _NumFloat + Copy {}
impl<T: _NumFloat + Copy> Float for T {}
