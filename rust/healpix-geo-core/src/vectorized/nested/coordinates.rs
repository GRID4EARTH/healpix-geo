#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use cdshealpix::nested::Layer;

use crate::ellipsoid::Ellipsoid;
use crate::maybe_parallelize;
use crate::scalar::nested::coordinates as scalar;

pub fn healpix_to_lonlat(
    ipix: &[u64],
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::<(f64, f64)>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| scalar::healpix_to_lonlat(
        hash, layer, ellipsoid
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
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    step: usize,
    nthreads: usize,
) -> Vec<Vec<(f64, f64)>> {
    let mut result = Vec::<Vec<(f64, f64)>>::with_capacity(ipix.len());

    maybe_parallelize!(nthreads, ipix, result, |hash| scalar::vertices(
        hash, layer, ellipsoid, &step
    ));

    result
}

pub fn bilinear_interpolation(
    coords: &[(f64, f64)],
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    nthreads: usize,
) -> Vec<Vec<(u64, f64)>> {
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
    use cdshealpix as healpix;
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;

    #[test]
    fn test_bilinear_interpolation() {
        let layer = healpix::nested::get(3);
        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("WGS84").unwrap(),
        ));

        let coords = vec![(10.0, 30.0), (45.0, 50.0), (80.0, 70.0)];
        let nthreads: usize = 0;

        let actual = bilinear_interpolation(&coords, layer, &ellipsoid, nthreads);
        let expected: Vec<Vec<(u64, f64)>> = vec![
            vec![
                (311, 0.24782825502968173),
                (34, 0.15113311306390578),
                (317, 0.37335533528612724),
                (40, 0.22768329662028527),
            ],
            vec![
                (48, 0.7417555426415379),
                (49, 0.11949676803259777),
                (50, 0.11949676803259777),
                (51, 0.01925092129326657),
            ],
            vec![
                (53, 0.47242437987207875),
                (122, 0.06464507274912022),
                (55, 0.4072093017055717),
                (123, 0.05572124567322935),
            ],
        ];
        assert_eq!(actual, expected);
    }
}
