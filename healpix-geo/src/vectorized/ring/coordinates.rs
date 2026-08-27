use cdshealpix as healpix;
#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use crate::ellipsoid::Ellipsoid;
use crate::maybe_parallelize;
use crate::scalar::ring::coordinates as scalar;

pub fn healpix_to_lonlat(
    ipix: &[u64],
    nside: &u32,
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::<(f64, f64)>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| scalar::healpix_to_lonlat(
        hash, nside, ellipsoid
    ));

    result
}

pub fn lonlat_to_healpix(
    coords: &[(f64, f64)],
    nside: &u32,
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(coords.len());

    maybe_parallelize!(nthreads, coords, result, |(lon, lat)| {
        scalar::lonlat_to_healpix(lon, lat, nside, ellipsoid)
    });

    result
}

pub fn vertices(
    ipix: &[u64],
    nside: &u32,
    ellipsoid: &Ellipsoid,
    step: usize,
    nthreads: usize,
) -> Vec<Vec<(f64, f64)>> {
    let mut result = Vec::<Vec<(f64, f64)>>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| scalar::vertices(
        hash, nside, ellipsoid, &step
    ));

    result
}

pub fn healpix_to_cartesian(
    ipix: &[u64],
    nside: &u32,
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<(f64, f64, f64)> {
    let mut result = Vec::<(f64, f64, f64)>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| {
        scalar::healpix_to_cartesian(hash, nside, ellipsoid)
    });

    result
}

pub fn cartesian_to_healpix(
    coords: &[(f64, f64, f64)],
    nside: &u32,
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<u64> {
    let mut result = Vec::<u64>::with_capacity(coords.len());

    maybe_parallelize!(nthreads, coords, result, |(x, y, z)| {
        scalar::cartesian_to_healpix(x, y, z, nside, ellipsoid)
    });

    result
}

pub fn bilinear_interpolation(
    coords: &[(f64, f64)],
    nside: &u32,
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<Vec<(u64, f64)>> {
    let layer = healpix::nested::get(healpix::depth(*nside));
    let mut result = Vec::<Vec<(u64, f64)>>::with_capacity(coords.len());

    maybe_parallelize!(nthreads, coords, result, |(lon, lat)| {
        scalar::bilinear_interpolation(lon, lat, layer, ellipsoid)
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ellipsoid::ReferenceEllipsoid;
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;

    #[test]
    fn test_bilinear_interpolation() {
        let nside = 8;
        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("WGS84").unwrap(),
        ));

        let coords = vec![(10.0, 30.0), (45.0, 50.0), (80.0, 70.0)];
        let nthreads: usize = 0;

        let actual = bilinear_interpolation(&coords, &nside, &ellipsoid, nthreads);
        let expected: Vec<Vec<(u64, f64)>> = vec![
            vec![
                (209, 0.24782825502968173),
                (177, 0.15113311306390578),
                (176, 0.37335533528612724),
                (145, 0.22768329662028527),
            ],
            vec![
                (87, 0.7417555426415379),
                (63, 0.11949676803259777),
                (62, 0.11949676803259777),
                (42, 0.01925092129326657),
            ],
            vec![
                (27, 0.47242437987207875),
                (28, 0.06464507274912022),
                (14, 0.4072093017055717),
                (15, 0.05572124567322935),
            ],
        ];
        assert_eq!(actual, expected);
    }
}
