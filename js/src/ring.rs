use cdshealpix as healpix;
use healpix_geo_core::ellipsoid::ReferenceBody;
use healpix_geo_core::scalar::nested::coordinates as scalar;
use wasm_bindgen::prelude::*;

use crate::coordinates::Coordinate;
use crate::ellipsoid::Ellipsoid;
use crate::geometry::spherical_vertex;

/// Ring index of the cell at the given z-order coordinates
///
/// Interleaves the bits of `i` and `j` — the two axes of the nested z-order
/// numbering within a base-resolution pixel — into the nested cell index at
/// `depth`, then converts it to the ring scheme. Note the parameter order: the
/// function takes `(depth, j, i)` but interleaves them as `ij2h(i, j)`.
#[wasm_bindgen(js_name = bitCombinedRing)]
pub fn bit_combine(depth: u8, j: u32, i: u32) -> u64 {
    let layer = healpix::nested::get(depth);
    let zoc = healpix::nested::zordercurve::get_zoc(depth);
    let hash = zoc.ij2h(i, j);

    layer.to_ring(hash)
}

/// Center coordinates for the given cell
#[wasm_bindgen(js_name = healpixToLonLatRing)]
pub fn healpix_to_lonlat(hash: u64, depth: u8, ellipsoid: Option<Ellipsoid>) -> Coordinate {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();
    let hash_ = layer.from_ring(hash);

    let (lon, lat) = scalar::healpix_to_lonlat(&hash_, layer, &ellipsoid_);

    Coordinate { lon, lat }
}

/// Project the given coordinate to the healpix grid
#[wasm_bindgen(js_name = lonLatToHealpixRing)]
pub fn lonlat_to_healpix(lon: f64, lat: f64, depth: u8, ellipsoid: Option<Ellipsoid>) -> u64 {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    layer.to_ring(scalar::lonlat_to_healpix(&lon, &lat, layer, &ellipsoid_))
}

/// Single vertex of the given cell
///
/// The parameters `u` and `v` represent offsets from the southern vertex of the given cell.
#[wasm_bindgen(js_name = vertexRing)]
pub fn vertex(hash: u64, depth: u8, u: f64, v: f64, ellipsoid: Option<Ellipsoid>) -> Coordinate {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    let center = layer.center_of_projected_cell(layer.from_ring(hash));
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

        assert_eq!(hash, 4);
    }
}
