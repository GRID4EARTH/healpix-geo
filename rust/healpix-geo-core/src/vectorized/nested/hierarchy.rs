#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use cdshealpix as healpix;
use cdshealpix::compass_point::MainWind;
use cdshealpix::nested::Layer;

use crate::maybe_parallelize;
use crate::scalar::nested::hierarchy as scalar;

/// Below this size, creating or scheduling parallel work is generally more
/// expensive than the immediate-neighbour calculation itself.
const AUTO_PARALLEL_THRESHOLD: usize = 32_768;

/// Return immediate neighbours without losing their directional positions.
///
/// The result is a flat row-major buffer with `directions.len()` values per
/// input cell. Missing positions are represented by `-1`.
pub fn neighbours(
    ipix: &[u64],
    layer: &Layer,
    directions: &[MainWind],
    nthreads: usize,
) -> Vec<i64> {
    let width = directions.len();
    debug_assert!(width > 0);

    let mut result = vec![-1; ipix.len() * width];

    #[cfg(not(target_arch = "wasm32"))]
    {
        let write_parallel = |output: &mut [i64]| {
            output
                .par_chunks_mut(width)
                .zip(ipix.par_iter())
                .for_each(|(row, hash)| scalar::write_neighbours(hash, layer, directions, row));
        };

        match nthreads {
            1 => result
                .chunks_mut(width)
                .zip(ipix)
                .for_each(|(row, hash)| scalar::write_neighbours(hash, layer, directions, row)),
            0 if ipix.len() < AUTO_PARALLEL_THRESHOLD => result
                .chunks_mut(width)
                .zip(ipix)
                .for_each(|(row, hash)| scalar::write_neighbours(hash, layer, directions, row)),
            0 => write_parallel(&mut result),
            _ => {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(nthreads)
                    .build()
                    .unwrap();
                pool.install(|| write_parallel(&mut result));
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let _ = nthreads;
        result
            .chunks_mut(width)
            .zip(ipix)
            .for_each(|(row, hash)| scalar::write_neighbours(hash, layer, directions, row));
    }

    result
}

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
