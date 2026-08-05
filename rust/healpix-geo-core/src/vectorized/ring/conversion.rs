#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use crate::maybe_parallelize;
use crate::scalar::ring::conversion as scalar;
use crate::vectorized::depth::Depth;

pub fn from_zuniq(ipix: &[u64], nthreads: usize) -> Vec<(u64, u8)> {
    let mut result = Vec::<(u64, u8)>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, scalar::from_zuniq);

    result
}

pub fn from_nested(ipix: &[u64], depth: Depth, nthreads: usize) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(ipix.len());

    match depth {
        Depth::Scalar(depth) => {
            maybe_parallelize!(nthreads, ipix, result, |hash| scalar::from_nested(
                hash, depth
            ));
        }
        Depth::Array(depths) => {
            let zipped: Vec<_> = ipix.iter().zip(depths.iter()).collect();
            maybe_parallelize!(nthreads, zipped, result, |(hash, depth)| {
                scalar::from_nested(hash, depth)
            });
        }
    };

    result
}

pub fn to_zuniq(ipix: &[u64], depth: Depth, nthreads: usize) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(ipix.len());

    match depth {
        Depth::Scalar(d) => {
            maybe_parallelize!(nthreads, ipix, result, |hash| scalar::to_zuniq(hash, d));
        }
        Depth::Array(d) => {
            let zipped: Vec<_> = ipix.iter().zip(d.iter()).collect();
            maybe_parallelize!(nthreads, zipped, result, |(hash, depth)| scalar::to_zuniq(
                hash, depth
            ));
        }
    };

    result
}

pub fn to_nested(ipix: &[u64], depth: Depth, nthreads: usize) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(ipix.len());

    match depth {
        Depth::Scalar(d) => {
            maybe_parallelize!(nthreads, ipix, result, |hash| scalar::to_nested(hash, d));
        }
        Depth::Array(d) => {
            let zipped: Vec<_> = ipix.iter().zip(d.iter()).collect();
            maybe_parallelize!(nthreads, zipped, result, |(hash, depth)| scalar::to_nested(
                hash, depth
            ));
        }
    };

    result
}
