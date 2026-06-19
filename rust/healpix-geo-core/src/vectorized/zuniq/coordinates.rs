#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use cdshealpix::nested::Layer;

use crate::ellipsoid::Ellipsoid;
use crate::maybe_parallelize;
use crate::scalar::zuniq::coordinates as scalar;

pub fn healpix_to_lonlat(ipix: &[u64], ellipsoid: &Ellipsoid, nthreads: usize) -> Vec<(f64, f64)> {
    let mut result = Vec::<(f64, f64)>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| scalar::healpix_to_lonlat(
        hash, ellipsoid
    ));

    result
}

pub fn lonlat_to_healpix(
    coords: &[(f64, f64)],
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(coords.len());

    maybe_parallelize!(nthreads, coords, result, |(lon, lat)| {
        scalar::lonlat_to_healpix(lon, lat, layer, ellipsoid)
    });

    result
}

pub fn vertices(
    ipix: &[u64],
    ellipsoid: &Ellipsoid,
    step: usize,
    nthreads: usize,
) -> Vec<Vec<(f64, f64)>> {
    let mut result = Vec::<Vec<(f64, f64)>>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| scalar::vertices(
        hash, ellipsoid, &step
    ));

    result
}
