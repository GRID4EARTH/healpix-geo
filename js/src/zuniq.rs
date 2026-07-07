use cdshealpix as healpix;
use healpix_geo_core::ellipsoid::ReferenceBody;
use healpix_geo_core::scalar::zuniq::coordinates as scalar;
use wasm_bindgen::prelude::*;

use crate::coordinates::Coordinate;
use crate::ellipsoid::EllipsoidLike;
use crate::geometry::spherical_vertex;

#[wasm_bindgen(js_name = zuniq)]
pub struct Zuniq;

#[wasm_bindgen(js_class = zuniq)]
impl Zuniq {
    /// `zuniq` index of the cell at the given z-order coordinates
    ///
    /// Interleaves the bits of `i` and `j` (the two axes of the nested z-order
    /// numbering within a base-resolution pixel) into the nested cell index at
    /// `depth`, then converts it to the `zuniq` scheme.
    #[wasm_bindgen(js_name = bitCombine)]
    pub fn bit_combine(depth: u8, i: u32, j: u32) -> u64 {
        let zoc = healpix::nested::zordercurve::get_zoc(depth);

        let hash = zoc.ij2h(i, j);

        healpix::nested::to_zuniq(depth, hash)
    }

    /// Center coordinates for the given cell
    #[wasm_bindgen(js_name = healpixToLonLat)]
    pub fn healpix_to_lonlat(hash: u64, ellipsoid: Option<EllipsoidLike>) -> Coordinate {
        let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

        let (lon, lat) = scalar::healpix_to_lonlat(&hash, &ellipsoid_);

        Coordinate { lon, lat }
    }

    /// Project the given coordinate to the healpix grid
    #[wasm_bindgen(js_name = lonLatToHealpix)]
    pub fn lonlat_to_healpix(
        lon: f64,
        lat: f64,
        depth: u8,
        ellipsoid: Option<EllipsoidLike>,
    ) -> u64 {
        let layer = healpix::nested::get(depth);
        let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

        scalar::lonlat_to_healpix(&lon, &lat, layer, &ellipsoid_)
    }

    /// Single vertex of the given cell
    ///
    /// The parameters `u` and `v` represent offsets from the southern vertex of the given cell.
    #[wasm_bindgen(js_name = vertex)]
    pub fn vertex(hash: u64, u: f64, v: f64, ellipsoid: Option<EllipsoidLike>) -> Coordinate {
        let (depth, nested) = healpix::nested::from_zuniq(hash);
        let layer = healpix::nested::get(depth);
        let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

        let center = layer.center_of_projected_cell(nested);
        let (lon, lat) = spherical_vertex(center, depth, (u, v));

        Coordinate {
            lon: lon.to_degrees().rem_euclid(360.0),
            lat: ellipsoid_.latitude_authalic_to_geographic(lat).to_degrees(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex() {
        let hash: u64 = 288230376151711744;

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
            .map(|(u, v)| Zuniq::vertex(hash, u, v, None))
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

        let hash = Zuniq::bit_combine(depth, i, j);

        assert_eq!(hash, 360287970189639680);
    }
}
