use crate::maybe_parallelize;

use crate::scalar::zuniq::conversion as scalar;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

pub fn to_nested(ipix: &[u64], nthreads: usize) -> (Vec<u64>, Vec<u8>) {
    let mut result = Vec::<(u64, u8)>::with_capacity(ipix.len());
    maybe_parallelize!(nthreads, ipix, result, scalar::to_nested);

    result.into_iter().unzip()
}

pub fn to_ring(ipix: &[u64], nthreads: usize) -> (Vec<u64>, Vec<u8>) {
    let mut result = Vec::<(u64, u8)>::with_capacity(ipix.len());
    maybe_parallelize!(nthreads, ipix, result, scalar::to_ring);

    let (nested, depths): (Vec<u64>, Vec<u8>) = result.into_iter().unzip();

    (nested, depths)
}
