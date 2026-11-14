use crate::ellipsoid::Ellipsoid;
use crate::maybe_parallelize;
use crate::scalar::distances::GeodesicSolver;
use crate::scalar::nested::coordinates;
use cdshealpix::nested::Layer;
use cdshealpix::sph_geom::coo3d::{UnitVec3, UnitVect3, vec3_of};

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

fn to_vec3(layer: &Layer, hash: &u64) -> UnitVect3 {
    let (lon, lat) = layer.center(*hash);

    vec3_of(lon, lat)
}

pub fn angular_distances(
    from: &[u64],
    to: &[u64],
    chunks: usize,
    layer: &Layer,
    nthreads: usize,
) -> Vec<Vec<f64>> {
    let data: Vec<(&u64, &[u64])> = from.iter().zip(to.chunks(chunks)).collect();

    let mut result = Vec::<Vec<f64>>::with_capacity(from.len());
    maybe_parallelize!(
        nthreads,
        data,
        result,
        |(from_, to_)| {
            let first = to_vec3(layer, from_);
            to_.iter()
                .map(|h| to_vec3(layer, h))
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
    layer: &Layer,
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
            let first = coordinates::healpix_to_lonlat(from_, layer, ellipsoid);
            to_.iter()
                .map(|h| {
                    solver.distance(&first, &coordinates::healpix_to_lonlat(h, layer, ellipsoid))
                })
                .collect::<Vec<f64>>()
        },
        min_len = 16
    );

    result
}

#[cfg(test)]
mod tests {
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;
    use rstest::rstest;

    use super::*;
    use crate::ellipsoid::{ReferenceEllipsoid, ReferenceSphere};

    fn wgs84() -> Ellipsoid {
        Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("WGS84").unwrap(),
        ))
    }

    #[test]
    fn geodesic_distances_match_pyproj() {
        // Reference values: pyproj.Geod(ellps="WGS84").inv on the WGS84 cell centres.
        let layer = cdshealpix::nested::get(1);
        let from = [0, 16, 25, 32, 46];
        let to = [2, 15, 27, 40, 41];
        let expected = [
            3265491.2781980103,
            10250304.064259019,
            3271797.130274477,
            5238406.494509201,
            3687722.904155331,
        ];

        let actual = geodesic_distances(&from, &to, 1, layer, &wgs84(), 1);

        assert_eq!(actual.len(), from.len());
        for (row, expected) in actual.iter().zip(expected) {
            assert_eq!(row.len(), 1);
            assert!(
                (row[0] - expected).abs() < 1e-6,
                "{} != {}",
                row[0],
                expected
            );
        }
    }

    #[test]
    fn geodesic_distances_on_sphere_scale_angular_distances() {
        let radius = 6_371_000.0;
        let sphere = Ellipsoid::Sphere(ReferenceSphere::new(GeodesyEllipsoid::new(radius, 0.0)));
        let layer = cdshealpix::nested::get(3);
        let from: Vec<u64> = (0..20).collect();
        let to: Vec<u64> = (0..60).map(|i| (i * 37) % 768).collect();

        let angular = angular_distances(&from, &to, 3, layer, 1);
        let geodesic = geodesic_distances(&from, &to, 3, layer, &sphere, 1);

        for (a_row, g_row) in angular.iter().zip(&geodesic) {
            for (a, g) in a_row.iter().zip(g_row) {
                assert!((a * radius - g).abs() < 1e-6);
            }
        }
    }

    #[rstest]
    fn geodesic_distances_do_not_depend_on_nthreads(#[values(0, 2, 4)] nthreads: usize) {
        let layer = cdshealpix::nested::get(6);
        let n_cells = layer.n_hash();
        let from: Vec<u64> = (0..500).map(|i| (i * 97) % n_cells).collect();
        let to: Vec<u64> = (0..4000).map(|i| (i * 131) % n_cells).collect();

        let serial = geodesic_distances(&from, &to, 8, layer, &wgs84(), 1);
        let parallel = geodesic_distances(&from, &to, 8, layer, &wgs84(), nthreads);

        assert_eq!(serial, parallel);
    }

    #[test]
    fn geodesic_distances_agree_across_indexing_schemes() {
        let depth = 4;
        let layer = cdshealpix::nested::get(depth);
        let from: Vec<u64> = (0..30).map(|i| (i * 101) % layer.n_hash()).collect();
        let to: Vec<u64> = (0..90).map(|i| (i * 53) % layer.n_hash()).collect();

        let nested = geodesic_distances(&from, &to, 3, layer, &wgs84(), 1);

        let to_ring =
            |ipix: &[u64]| -> Vec<u64> { ipix.iter().map(|h| layer.to_ring(*h)).collect() };
        let ring = crate::vectorized::ring::distances::geodesic_distances(
            &to_ring(&from),
            &to_ring(&to),
            3,
            &cdshealpix::nside(depth),
            &wgs84(),
            1,
        );

        let to_zuniq = |ipix: &[u64]| -> Vec<u64> {
            ipix.iter()
                .map(|h| cdshealpix::nested::to_zuniq(depth, *h))
                .collect()
        };
        let zuniq = crate::vectorized::zuniq::distances::geodesic_distances(
            &to_zuniq(&from),
            &to_zuniq(&to),
            3,
            &wgs84(),
            1,
        );

        assert_eq!(nested, ring);
        assert_eq!(nested, zuniq);
    }
}
