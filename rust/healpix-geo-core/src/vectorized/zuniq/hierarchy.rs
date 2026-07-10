#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use crate::maybe_parallelize;
use crate::scalar::zuniq::hierarchy as scalar;

pub fn kth_neighbours(ipix: &[u64], ring: &u32, nthreads: usize) -> Vec<Vec<i64>> {
    let mut result = Vec::<Vec<i64>>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| scalar::kth_neighbours(
        hash, ring
    ));

    result
}

pub fn kth_neighbourhood(ipix: &[u64], ring: &u32, nthreads: usize) -> Vec<Vec<i64>> {
    let mut result = Vec::<Vec<i64>>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| scalar::kth_neighbourhood(
        hash, ring
    ));

    result
}
