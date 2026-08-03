use crate::maybe_parallelize;
#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use crate::ellipsoid::Ellipsoid;
use crate::scalar::mesh as scalar;

pub fn vertex_to_geographic(
    depth: u8,
    hashes: &[u64],
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::<(f64, f64)>::with_capacity(hashes.len());

    maybe_parallelize!(nthreads, hashes, result, |hash| {
        scalar::vertex_to_geographic(depth, hash, ellipsoid)
    });

    result
}
