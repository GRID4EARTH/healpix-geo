use crate::ellipsoid::Ellipsoid;

use cdshealpix as healpix;
use cdshealpix::nested::Layer;

pub fn healpix_to_lonlat(hash: &u64, ellipsoid: &Ellipsoid) -> (f64, f64) {
    let (depth, hash_nested) = healpix::nested::from_zuniq(*hash);
    let layer = healpix::nested::get(depth);

    crate::scalar::nested::coordinates::healpix_to_lonlat(&hash_nested, layer, ellipsoid)
}

pub fn lonlat_to_healpix(lon: &f64, lat: &f64, layer: &Layer, ellipsoid: &Ellipsoid) -> u64 {
    let hash_nested =
        crate::scalar::nested::coordinates::lonlat_to_healpix(lon, lat, layer, ellipsoid);

    healpix::nested::to_zuniq_unsafe(layer.depth(), hash_nested)
}

pub fn vertices(hash: &u64, ellipsoid: &Ellipsoid, step: &usize) -> Vec<(f64, f64)> {
    let (depth, hash_nested) = healpix::nested::from_zuniq(*hash);
    let layer = healpix::nested::get(depth);

    crate::scalar::nested::coordinates::vertices(&hash_nested, layer, ellipsoid, step)
}

pub fn healpix_to_cartesian(hash: &u64, ellipsoid: &Ellipsoid) -> (f64, f64, f64) {
    let (depth, hash_nested) = healpix::nested::from_zuniq(*hash);
    let layer = healpix::nested::get(depth);

    crate::scalar::nested::coordinates::healpix_to_cartesian(&hash_nested, layer, ellipsoid)
}

pub fn cartesian_to_healpix(
    x: &f64,
    y: &f64,
    z: &f64,
    layer: &Layer,
    ellipsoid: &Ellipsoid,
) -> u64 {
    let hash = crate::scalar::nested::coordinates::cartesian_to_healpix(x, y, z, layer, ellipsoid);

    healpix::nested::to_zuniq_unsafe(layer.depth(), hash)
}

pub fn bilinear_interpolation(
    lon: &f64,
    lat: &f64,
    layer: &Layer,
    ellipsoid: &Ellipsoid,
) -> Vec<(u64, f64)> {
    crate::scalar::nested::coordinates::bilinear_interpolation(lon, lat, layer, ellipsoid)
        .into_iter()
        .map(|(hash, weight)| {
            (
                healpix::nested::to_zuniq_unsafe(layer.depth(), hash),
                weight,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ellipsoid::ReferenceEllipsoid;
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;

    #[test]
    fn test_healpix_to_cartesian() {
        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("sphere").unwrap(),
        ));

        let hash = 6052837899185946624;

        let (x, y, z) = healpix_to_cartesian(&hash, &ellipsoid);

        let (expected_x, expected_y, expected_z) =
            (-3357810.2476832955, -3357810.2476832923, -4247331.333333333);

        assert!((x - expected_x).abs() < 1e-8);
        assert!((y - expected_y).abs() < 1e-8);
        assert!((z - expected_z).abs() < 1e-8);
    }

    #[test]
    fn test_cartesian_to_healpix() {
        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("sphere").unwrap(),
        ));
        let layer = healpix::nested::get(0);

        let (x, y, z) = (-3357810.2476832955, -3357810.2476832923, -4247331.333333333);
        let actual = cartesian_to_healpix(&x, &y, &z, layer, &ellipsoid);

        let expected = 6052837899185946624;

        assert_eq!(actual, expected);
    }
}
