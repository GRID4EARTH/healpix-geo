use crate::maybe_parallelize;
use crate::scalar::ring::mesh as scalar;
#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

pub fn vertex_indices(depth: u8, hashes: &[u64], nthreads: usize) -> Vec<(u64, u64, u64, u64)> {
    let mut result = Vec::<(u64, u64, u64, u64)>::with_capacity(hashes.len());

    maybe_parallelize!(nthreads, hashes, result, |hash| scalar::vertex_indices(
        depth, hash
    ));

    result
}
