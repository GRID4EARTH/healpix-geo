use crate::ellipsoid::{Ellipsoid, ReferenceBody};

pub fn lonlat_to_cartesian(lon: &f64, lat: &f64, ellipsoid: &Ellipsoid) -> (f64, f64, f64) {
    let p = (lon.to_radians(), lat.to_radians());

    ellipsoid.geographic_to_cartesian(&p)
}

pub fn cartesian_to_lonlat(x: &f64, y: &f64, z: &f64, ellipsoid: &Ellipsoid) -> (f64, f64) {
    let p = (*x, *y, *z);
    let (lon, lat) = ellipsoid.cartesian_to_geographic(&p);

    (lon.to_degrees(), lat.to_degrees())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ellipsoid::ReferenceEllipsoid;
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;

    #[test]
    fn test_lonlat_to_cartesian() {
        let base_ellipsoid = GeodesyEllipsoid::named("WGS84").unwrap();
        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(base_ellipsoid));

        let lon: f64 = -180.0;
        let lat: f64 = 75.0;

        let (x, y, z) = lonlat_to_cartesian(&lon, &lat, &ellipsoid);

        assert_eq!(x, -1655962.9523645048);
        assert_eq!(y, -2.0279697291197919e-10);
        assert_eq!(z, 6138765.682358242);
    }

    #[test]
    fn test_cartesian_to_lonlat() {
        let base_ellipsoid = GeodesyEllipsoid::named("WGS84").unwrap();
        let ellipsoid = Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(base_ellipsoid));

        let (x, y, z) = (
            -1655962.9523645048,
            -2.0279697291197919e-10,
            6138765.682358242,
        );
        let (lon, lat) = cartesian_to_lonlat(&x, &y, &z, &ellipsoid);

        assert_eq!(lon, -180.0);
        assert_eq!(lat, 75.0);
    }
}
