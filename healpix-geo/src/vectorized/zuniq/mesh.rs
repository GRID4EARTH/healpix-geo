use crate::maybe_parallelize;
use crate::scalar::zuniq::mesh as scalar;
#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

pub fn vertex_indices(hashes: &[u64], nthreads: usize) -> Vec<(u64, u64, u64, u64)> {
    let mut result = Vec::<(u64, u64, u64, u64)>::with_capacity(hashes.len());

    maybe_parallelize!(nthreads, hashes, result, scalar::vertex_indices);

    result
}
