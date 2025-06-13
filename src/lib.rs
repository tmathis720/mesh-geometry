#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

//! mesh-geometry — core geometry primitives & metrics for FVM meshes.

pub mod prelude;
pub use prelude::*;
pub mod float;
pub use float::Float;
pub mod point;
pub mod vec;
pub mod metrics;
pub mod queries;
pub mod transforms;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
