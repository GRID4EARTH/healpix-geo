#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use cdshealpix::nested::Layer;

use crate::maybe_parallelize;
use crate::scalar::depth::Depth;
use crate::scalar::nested::conversion as scalar;

pub fn from_zuniq(ipix: &[u64], nthreads: usize) -> Vec<(u64, u8)> {
    let mut result = Vec::<(u64, u8)>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, scalar::from_zuniq);

    result
}

pub fn from_ring(ipix: &[u64], depth: Depth, nthreads: usize) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(ipix.len());

    match depth {
        Depth::Scalar(d) => {
            maybe_parallelize!(nthreads, ipix, result, |hash| scalar::from_ring(hash, d));
        }
        Depth::Array(d) => {
            let zipped: Vec<_> = ipix.iter().zip(d.iter()).collect();
            maybe_parallelize!(nthreads, zipped, result, |(hash, depth)| scalar::from_ring(
                hash, depth
            ));
        }
    };

    result
}

pub fn to_zuniq(hash: &[u64], depth: Depth, nthreads: usize) -> Vec<u64> {
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

pub fn to_ring(hash: &[u64], depth: Depth, nthreads: usize) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(ipix.len());

    match depth {
        Depth::Scalar(d) => {
            maybe_parallelize!(nthreads, ipix, result, |hash| scalar::to_ring(hash, d));
        }
        Depth::Array(d) => {
            let zipped: Vec<_> = ipix.iter().zip(d.iter()).collect();
            maybe_parallelize!(nthreads, zipped, result, |(hash, depth)| scalar::to_ring(
                hash, depth
            ));
        }
    };

    result
}
