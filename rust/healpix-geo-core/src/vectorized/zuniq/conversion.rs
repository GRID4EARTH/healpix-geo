use crate::maybe_parallelize;
use crate::vectorized::depth::Depth;

use crate::scalar::zuniq::conversion as scalar;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

pub fn from_nested(ipix: &[u64], depth: Depth, nthreads: usize) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(ipix.len());

    match depth {
        Depth::Scalar(depth) => {
            maybe_parallelize!(nthreads, ipix, result, |hash| scalar::from_nested(
                hash, &depth
            ));
        }
        Depth::Array(depths) => {
            let joined: Vec<(&u64, &u8)> = ipix.iter().zip(depths.iter()).collect();
            maybe_parallelize!(nthreads, &joined, result, |(hash, depth)| {
                scalar::from_nested(hash, depth)
            });
        }
    }

    result
}

pub fn from_ring(ipix: &[u64], depth: Depth, nthreads: usize) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(ipix.len());

    match depth {
        Depth::Scalar(depth) => {
            maybe_parallelize!(nthreads, ipix, result, |hash| scalar::from_ring(
                hash, &depth
            ));
        }
        Depth::Array(depths) => {
            let joined: Vec<(&u64, &u8)> = ipix.iter().zip(depths.iter()).collect();
            maybe_parallelize!(
                nthreads,
                &joined,
                result,
                |(hash, depth)| scalar::from_ring(hash, depth),
            );
        }
    }

    result
}

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
