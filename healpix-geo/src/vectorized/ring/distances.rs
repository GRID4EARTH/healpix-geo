use crate::ellipsoid::Ellipsoid;
use crate::maybe_parallelize;
use crate::scalar::distances::GeodesicSolver;
use crate::scalar::ring::coordinates;
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
    maybe_parallelize!(
        nthreads,
        data,
        result,
        |(from_, to_)| {
            let first = to_vec3(nside, from_);
            to_.iter()
                .map(|h| to_vec3(nside, h))
                .map(|vec| first.ang_dist(&vec))
                .collect::<Vec<f64>>()
        },
        min_len = 16
    );

    result
}

/// Geodesic distances in metres between the centres of `from` and `to` cells.
///
/// `to` holds `chunks` cells per entry of `from`. Cell centres are converted
/// to geographic coordinates on `ellipsoid` before solving the inverse
/// geodesic problem, see [`GeodesicSolver`].
pub fn geodesic_distances(
    from: &[u64],
    to: &[u64],
    chunks: usize,
    nside: &u32,
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<Vec<f64>> {
    if chunks == 0 {
        return vec![Vec::new(); from.len()];
    }

    let solver = GeodesicSolver::new(ellipsoid);
    let data: Vec<(&u64, &[u64])> = from.iter().zip(to.chunks(chunks)).collect();

    let mut result = Vec::<Vec<f64>>::with_capacity(from.len());
    maybe_parallelize!(
        nthreads,
        data,
        result,
        |(from_, to_)| {
            let first = coordinates::healpix_to_lonlat(from_, nside, ellipsoid);
            to_.iter()
                .map(|h| {
                    solver.distance(&first, &coordinates::healpix_to_lonlat(h, nside, ellipsoid))
                })
                .collect::<Vec<f64>>()
        },
        min_len = 16
    );

    result
}
