use crate::maybe_parallelize;
use cdshealpix::sph_geom::coo3d::{UnitVec3, UnitVect3, vec3_of};

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

fn to_vec3(nside: &u32, hash: &u64) -> UnitVect3 {
    let (lon, lat) = cdshealpix::ring::center(*nside, *hash);

    vec3_of(lon, lat)
}

pub fn angular_distances(
    from: &[u64],
    to: &[u64],
    chunks: usize,
    nside: &u32,
    nthreads: usize,
) -> Vec<Vec<f64>> {
    let data: Vec<(&u64, &[u64])> = from.iter().zip(to.chunks(chunks)).collect();

    let mut result = Vec::<Vec<f64>>::with_capacity(from.len());
    maybe_parallelize!(nthreads, data, result, |(from_, to_)| {
        let first = to_vec3(nside, from_);
        to_.iter()
            .map(|h| to_vec3(nside, h))
            .map(|vec| first.ang_dist(&vec))
            .collect::<Vec<f64>>()
    });

    result
}
