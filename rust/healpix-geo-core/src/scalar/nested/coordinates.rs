use crate::ellipsoid::{Ellipsoid, ReferenceBody};

use cdshealpix::compass_point::Cardinal;
use cdshealpix::nested::Layer;

pub fn healpix_to_lonlat(hash: &u64, layer: &Layer, ellipsoid: &Ellipsoid) -> (f64, f64) {
    let center = layer.center(*hash);

    let lon = center.0.to_degrees().rem_euclid(360.0);
    let lat = ellipsoid
        .latitude_authalic_to_geographic(center.1)
        .to_degrees();

    (lon, lat)
}

pub fn lonlat_to_healpix(lon: &f64, lat: &f64, layer: &Layer, ellipsoid: &Ellipsoid) -> u64 {
    let lon_ = lon.rem_euclid(360.0).to_radians();
    let lat_ = ellipsoid.latitude_geographic_to_authalic(lat.to_radians());

    layer.hash(lon_, lat_)
}

pub fn vertices(hash: &u64, layer: &Layer, ellipsoid: &Ellipsoid, step: &usize) -> Vec<(f64, f64)> {
    let vertices: Vec<(f64, f64)> = if *step == 1 {
        layer.vertices(*hash).into()
    } else {
        layer
            .path_along_cell_edge(*hash, &Cardinal::S, false, *step as u32)
            .into()
    };

    vertices
        .into_iter()
        .map(|(lon, lat): (f64, f64)| {
            (
                lon.to_degrees().rem_euclid(360.0),
                ellipsoid.latitude_authalic_to_geographic(lat).to_degrees(),
            )
        })
        .collect()
}

pub fn bilinear_interpolation(
    lon: &f64,
    lat: &f64,
    layer: &Layer,
    ellipsoid: &Ellipsoid,
) -> Vec<(u64, f64)> {
    let lon_ = lon.rem_euclid(360.0).to_radians();
    let lat_ = ellipsoid.latitude_geographic_to_authalic(lat.to_radians());

    layer.bilinear_interpolation(lon_, lat_).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ellipsoid::ReferenceEllipsoid;
    use cdshealpix as healpix;
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;

    #[test]
    fn test_lonlat_to_healpix_edge_cases_lon() {
        let layer = healpix::nested::get(0);

        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("WGS84").unwrap(),
        ));

        let lon: f64 = -180.0;
        let lat: f64 = 75.0;

        let actual = lonlat_to_healpix(&lon, &lat, layer, &ellipsoid);
        assert_eq!(actual, 2);

        let lon: f64 = 180.0;
        let lat: f64 = 75.0;

        let actual = lonlat_to_healpix(&lon, &lat, layer, &ellipsoid);
        assert_eq!(actual, 2);

        let lon: f64 = 0.0;
        let lat: f64 = 75.0;

        let actual = lonlat_to_healpix(&lon, &lat, layer, &ellipsoid);
        assert_eq!(actual, 0);

        let lon: f64 = 360.0;
        let lat: f64 = 75.0;

        let actual = lonlat_to_healpix(&lon, &lat, layer, &ellipsoid);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_bilinear_interpolation() {
        let layer = healpix::nested::get(1);
        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("WGS84").unwrap(),
        ));

        let lon = 45.0;
        let lat = 60.0;

        let (ipix, weights): (Vec<u64>, Vec<f64>) =
            bilinear_interpolation(&lon, &lat, layer, &ellipsoid)
                .into_iter()
                .unzip();

        assert_eq!(ipix, vec![0, 1, 2, 3]);
        assert_eq!(
            weights,
            vec![
                0.018569674659181364,
                0.11770091886407795,
                0.11770091886407795,
                0.7460284876126627
            ]
        );
    }
}
