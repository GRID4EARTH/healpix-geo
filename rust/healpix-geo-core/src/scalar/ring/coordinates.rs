use crate::ellipsoid::{Ellipsoid, ReferenceBody};

use cdshealpix as healpix;
use cdshealpix::compass_point::Cardinal;
use cdshealpix::nested::Layer;

pub fn healpix_to_lonlat(hash: &u64, nside: &u32, ellipsoid: &Ellipsoid) -> (f64, f64) {
    let center = healpix::ring::center(*nside, *hash);

    let lon = center.0.to_degrees().rem_euclid(360.0);
    let lat = ellipsoid
        .latitude_authalic_to_geographic(center.1)
        .to_degrees();

    (lon, lat)
}

pub fn lonlat_to_healpix(lon: &f64, lat: &f64, nside: &u32, ellipsoid: &Ellipsoid) -> u64 {
    let lon_ = lon.rem_euclid(360.0).to_radians();
    let lat_ = ellipsoid.latitude_geographic_to_authalic(lat.to_radians());

    healpix::ring::hash(*nside, lon_, lat_)
}

pub fn vertices(hash: &u64, nside: &u32, ellipsoid: &Ellipsoid, step: &usize) -> Vec<(f64, f64)> {
    let vertices: Vec<(f64, f64)> = if *step == 1 {
        healpix::ring::vertices(*nside, *hash).into()
    } else {
        let layer = healpix::nested::get(healpix::depth(*nside));

        layer
            .path_along_cell_edge(layer.from_ring(*hash), &Cardinal::S, false, *step as u32)
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

pub fn healpix_to_cartesian(hash: &u64, nside: &u32, ellipsoid: &Ellipsoid) -> (f64, f64, f64) {
    let center = healpix::ring::center(*nside, *hash);
    let p = (
        center.0,
        ellipsoid.latitude_authalic_to_geographic(center.1),
    );

    ellipsoid.geographic_to_cartesian(&p)
}

pub fn cartesian_to_healpix(x: &f64, y: &f64, z: &f64, nside: &u32, ellipsoid: &Ellipsoid) -> u64 {
    let p = (*x, *y, *z);
    let (lon, lat) = ellipsoid.cartesian_to_geographic(&p);

    let lat_ = ellipsoid.latitude_geographic_to_authalic(lat);

    healpix::ring::hash(*nside, lon, lat_)
}

pub fn bilinear_interpolation(
    lon: &f64,
    lat: &f64,
    layer: &Layer,
    ellipsoid: &Ellipsoid,
) -> Vec<(u64, f64)> {
    let lon_ = lon.rem_euclid(360.0).to_radians();
    let lat_ = ellipsoid.latitude_geographic_to_authalic(lat.to_radians());

    layer
        .bilinear_interpolation(lon_, lat_)
        .into_iter()
        .map(|(hash, weight)| (layer.to_ring(hash), weight))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ellipsoid::ReferenceEllipsoid;
    use cdshealpix as healpix;
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;

    #[test]
    fn test_healpix_to_cartesian() {
        let nside = 1;

        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("sphere").unwrap(),
        ));

        let hash = 10;

        let (x, y, z) = healpix_to_cartesian(&hash, &nside, &ellipsoid);

        let (expected_x, expected_y, expected_z) =
            (-3357810.2476832955, -3357810.2476832923, -4247331.333333333);

        assert!((x - expected_x).abs() < 1e-8);
        assert!((y - expected_y).abs() < 1e-8);
        assert!((z - expected_z).abs() < 1e-8);
    }

    #[test]
    fn test_cartesian_to_healpix() {
        let nside = 1;

        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("sphere").unwrap(),
        ));

        let (x, y, z) = (-3357810.2476832955, -3357810.2476832923, -4247331.333333333);

        let actual = cartesian_to_healpix(&x, &y, &z, &nside, &ellipsoid);
        let expected = 10;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lonlat_to_healpix_edge_cases_lon() {
        let nside = healpix::nside(0);
        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("WGS84").unwrap(),
        ));

        let lon: f64 = -180.0;
        let lat: f64 = 75.0;

        let actual = lonlat_to_healpix(&lon, &lat, &nside, &ellipsoid);
        assert_eq!(actual, 2);

        let lon: f64 = 180.0;
        let lat: f64 = 75.0;

        let actual = lonlat_to_healpix(&lon, &lat, &nside, &ellipsoid);
        assert_eq!(actual, 2);

        let lon: f64 = 0.0;
        let lat: f64 = 75.0;

        let actual = lonlat_to_healpix(&lon, &lat, &nside, &ellipsoid);
        assert_eq!(actual, 0);

        let lon: f64 = 360.0;
        let lat: f64 = 75.0;

        let actual = lonlat_to_healpix(&lon, &lat, &nside, &ellipsoid);
        assert_eq!(actual, 0);
    }
}
