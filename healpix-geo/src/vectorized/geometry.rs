#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use crate::ellipsoid::Ellipsoid;
use crate::maybe_parallelize;
use crate::scalar::geometry as scalar;

pub fn lonlat_to_cartesian(
    coords: &[(f64, f64)],
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<(f64, f64, f64)> {
    let mut result = Vec::<(f64, f64, f64)>::with_capacity(coords.len());

    maybe_parallelize!(nthreads, coords, result, |(lon, lat)| {
        scalar::lonlat_to_cartesian(lon, lat, ellipsoid)
    });

    result
}

pub fn cartesian_to_lonlat(
    coords: &[(f64, f64, f64)],
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::<(f64, f64)>::with_capacity(coords.len());

    maybe_parallelize!(nthreads, coords, result, |(x, y, z)| {
        scalar::cartesian_to_lonlat(x, y, z, ellipsoid)
    });

    result
}
