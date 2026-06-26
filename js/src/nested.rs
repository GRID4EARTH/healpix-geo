use cdshealpix as healpix;
use healpix_geo_core::ellipsoid::ReferenceBody;
use healpix_geo_core::scalar::nested::coordinates as scalar;
use wasm_bindgen::prelude::*;

use crate::coordinates::Coordinate;
use crate::ellipsoid::EllipsoidLike;
use crate::geometry::spherical_vertex;

/// Nested index of the cell at the given z-order coordinates
///
/// Interleaves the bits of `i` and `j` — the two axes of the nested z-order
/// numbering within a base-resolution pixel — into the cell index at `depth`.
/// Note the parameter order: the function takes `(depth, j, i)` but interleaves
/// them as `ij2h(i, j)`.
#[wasm_bindgen(js_name = bitCombineNested)]
pub fn bit_combine(depth: u8, j: u32, i: u32) -> u64 {
    let zoc = healpix::nested::zordercurve::get_zoc(depth);

    zoc.ij2h(i, j)
}

/// Center coordinates for the given cell
#[wasm_bindgen(js_name = healpixToLonLatNested)]
pub fn healpix_to_lonlat(ipix: u64, depth: u8, ellipsoid: Option<EllipsoidLike>) -> Coordinate {
    let layer = healpix::nested::get(depth);

    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    let (lon, lat) = scalar::healpix_to_lonlat(&ipix, layer, &ellipsoid_);

    Coordinate { lon, lat }
}

/// Project the given coordinate to the healpix grid
#[wasm_bindgen(js_name = lonLatToHealpixNested)]
pub fn lonlat_to_healpix(lon: f64, lat: f64, depth: u8, ellipsoid: Option<EllipsoidLike>) -> u64 {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    scalar::lonlat_to_healpix(&lon, &lat, layer, &ellipsoid_)
}

/// Single vertex of the given cell
///
/// The parameters `u` and `v` represent offsets from the southern vertex of the given cell.
#[wasm_bindgen(js_name = vertexNested)]
pub fn vertex(
    hash: u64,
    depth: u8,
    u: f64,
    v: f64,
    ellipsoid: Option<EllipsoidLike>,
) -> Coordinate {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    let center = layer.center_of_projected_cell(hash);
    let (lon, lat) = spherical_vertex(center, depth, (u, v));

    Coordinate {
        lon: lon.to_degrees().rem_euclid(360.0),
        lat: ellipsoid_.latitude_authalic_to_geographic(lat).to_degrees(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex() {
        let hash: u64 = 0;
        let depth: u8 = 0;

        let uv: Vec<(f64, f64)> = vec![
            (0.0, 0.0),
            (0.5, 0.0),
            (1.0, 0.0),
            (1.0, 0.5),
            (1.0, 1.0),
            (0.5, 1.0),
            (0.0, 1.0),
            (0.0, 0.5),
        ];

        let values = uv
            .into_iter()
            .map(|(u, v)| vertex(hash, depth, u, v, None))
            .collect::<Vec<_>>();
        let expected: Vec<Coordinate> = vec![
            (45.0, 0.0),
            (67.5, 19.47122063),
            (90.0, 41.8103149),
            (90.0, 66.44353569),
            (45.0, 90.0),
            (0.0, 66.44353569),
            (0.0, 41.8103149),
            (22.5, 19.47122063),
        ]
        .into_iter()
        .map(|(lon, lat)| Coordinate { lon, lat })
        .collect();

        for (a, b) in values.into_iter().zip(expected) {
            assert!((a.lon - b.lon).abs() < 1e-4);
            assert!((a.lat - b.lat).abs() < 1e-4);
        }
    }

    #[test]
    fn test_bit_combine() {
        let i = 0;
        let j = 1;
        let depth = 1;

        let hash = bit_combine(depth, j, i);

        assert_eq!(hash, 2);
    }

    #[test]
    fn test_healpix_to_lonlat_ellipsoid() {
        use crate::ellipsoid::{EllipsoidInverseFlattening, EllipsoidLike, Sphere};

        let depth: u8 = 0;
        // a base pixel whose center sits at a mid latitude (~41.8 deg), where the
        // authalic -> geographic conversion produces a measurable difference
        let ipix: u64 = 0;

        let sphere_default = healpix_to_lonlat(ipix, depth, None);

        // WGS84: the geographic latitude differs measurably from the authalic
        // (spherical) latitude, while the longitude is unaffected
        let wgs84 = EllipsoidLike::EllipsoidInverseFlattening(EllipsoidInverseFlattening {
            semi_major_axis: 6378137.0,
            inverse_flattening: 298.257223563,
        });
        let ellipsoidal = healpix_to_lonlat(ipix, depth, Some(wgs84));

        assert!((sphere_default.lon - ellipsoidal.lon).abs() < 1e-9);
        assert!((sphere_default.lat - ellipsoidal.lat).abs() > 1e-3);

        // an explicit sphere reproduces the default (radius does not affect lon/lat)
        let sphere = EllipsoidLike::Sphere(Sphere {
            radius: 6_371_000.0,
        });
        let explicit_sphere = healpix_to_lonlat(ipix, depth, Some(sphere));

        assert!((sphere_default.lon - explicit_sphere.lon).abs() < 1e-9);
        assert!((sphere_default.lat - explicit_sphere.lat).abs() < 1e-9);
    }
}
