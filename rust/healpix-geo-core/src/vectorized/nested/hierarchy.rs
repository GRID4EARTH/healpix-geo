#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use cdshealpix as healpix;
use cdshealpix::nested::Layer;

use crate::maybe_parallelize;
use crate::scalar::nested::hierarchy as scalar;

pub fn kth_neighbours(ipix: &[u64], layer: &Layer, ring: &u32, nthreads: usize) -> Vec<Vec<i64>> {
    let mut result = Vec::<Vec<i64>>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| scalar::kth_neighbours(
        hash, layer, ring
    ));

    result
}

pub fn kth_neighbourhood(
    ipix: &[u64],
    layer: &Layer,
    ring: &u32,
    nthreads: usize,
) -> Vec<Vec<i64>> {
    let mut result = Vec::<Vec<i64>>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| scalar::kth_neighbourhood(
        hash, layer, ring
    ));

    result
}

pub fn parents(ipix: &[u64], delta_depth: u8, nthreads: usize) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(ipix.len());
    if delta_depth > 0 {
        maybe_parallelize!(nthreads, ipix, result, |hash| healpix::nested::parent(
            *hash,
            delta_depth
        ));
    } else {
        result[..].clone_from_slice(ipix);
    }

    result
}

pub fn children(ipix: &[u64], delta_depth: u8, nthreads: usize) -> Vec<Vec<u64>> {
    if delta_depth == 0 {
        panic!("cannot query children at the same depth as the input");
    }
    let mut result = Vec::<Vec<u64>>::with_capacity(ipix.len());
    maybe_parallelize!(nthreads, ipix, result, |hash| healpix::nested::children(
        *hash,
        delta_depth
    )
    .collect::<Vec<u64>>());

    result
}

pub fn siblings(ipix: &[u64], layer: &Layer, nthreads: usize) -> Vec<Vec<u64>> {
    let depth = layer.depth();
    let mut result = Vec::<Vec<u64>>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| healpix::nested::siblings(
        depth, *hash
    )
    .collect::<Vec<u64>>());

    result
}
