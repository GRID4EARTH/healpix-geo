use crate::ellipsoid::Ellipsoid;
use crate::maybe_parallelize;
use crate::scalar::distances::GeodesicSolver;
use crate::scalar::zuniq::coordinates;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

/// Geodesic distances in metres between the centres of `from` and `to` cells.
///
/// `to` holds `chunks` cells per entry of `from`. Cell centres are converted
/// to geographic coordinates on `ellipsoid` before solving the inverse
/// geodesic problem, see [`GeodesicSolver`]. Each cell carries its own depth,
/// so `from` and `to` may mix depths.
pub fn geodesic_distances(
    from: &[u64],
    to: &[u64],
    chunks: usize,
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
            let first = coordinates::healpix_to_lonlat(from_, ellipsoid);
            to_.iter()
                .map(|h| solver.distance(&first, &coordinates::healpix_to_lonlat(h, ellipsoid)))
                .collect::<Vec<f64>>()
        },
        min_len = 16
    );

    result
}
