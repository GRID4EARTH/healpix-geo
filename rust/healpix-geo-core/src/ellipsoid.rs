use geodesy::authoring::FourierCoefficients;
use geodesy::ellps::{Ellipsoid as GeodesyEllipsoid, Latitudes};

pub trait ReferenceBody {
    fn latitude_authalic_to_geographic(&self, latitude: f64) -> f64;
    fn latitude_geographic_to_authalic(&self, latitude: f64) -> f64;
}

#[derive(Clone, Debug)]
pub struct ReferenceSphere {
    ellipsoid: GeodesyEllipsoid,
}

impl ReferenceSphere {
    pub fn new(ellipsoid: GeodesyEllipsoid) -> Self {
        Self { ellipsoid }
    }
}

impl ReferenceBody for ReferenceSphere {
    fn latitude_authalic_to_geographic(&self, latitude: f64) -> f64 {
        latitude
    }

    fn latitude_geographic_to_authalic(&self, latitude: f64) -> f64 {
        latitude
    }
}

impl PartialEq for ReferenceSphere {
    fn eq(&self, other: &Self) -> bool {
        self.ellipsoid == other.ellipsoid
    }
}

#[derive(Clone, Debug)]
pub struct ReferenceEllipsoid {
    ellipsoid: GeodesyEllipsoid,
    coefficients: FourierCoefficients,
}

impl ReferenceEllipsoid {
    pub fn new(ellipsoid: GeodesyEllipsoid) -> Self {
        let coefficients = ellipsoid.coefficients_for_authalic_latitude_computations();

        Self {
            ellipsoid,
            coefficients,
        }
    }
}

impl ReferenceBody for ReferenceEllipsoid {
    fn latitude_authalic_to_geographic(&self, latitude: f64) -> f64 {
        self.ellipsoid
            .latitude_authalic_to_geographic(latitude, &self.coefficients)
    }

    fn latitude_geographic_to_authalic(&self, latitude: f64) -> f64 {
        self.ellipsoid
            .latitude_geographic_to_authalic(latitude, &self.coefficients)
    }
}

impl PartialEq for ReferenceEllipsoid {
    fn eq(&self, other: &Self) -> bool {
        self.ellipsoid == other.ellipsoid
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Ellipsoid {
    Ellipsoid(ReferenceEllipsoid),
    Sphere(ReferenceSphere),
}

impl ReferenceBody for Ellipsoid {
    fn latitude_authalic_to_geographic(&self, latitude: f64) -> f64 {
        match self {
            Self::Ellipsoid(wrapped) => wrapped.latitude_authalic_to_geographic(latitude),
            Self::Sphere(wrapped) => wrapped.latitude_authalic_to_geographic(latitude),
        }
    }

    fn latitude_geographic_to_authalic(&self, latitude: f64) -> f64 {
        match self {
            Self::Ellipsoid(wrapped) => wrapped.latitude_geographic_to_authalic(latitude),
            Self::Sphere(wrapped) => wrapped.latitude_geographic_to_authalic(latitude),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_ellipsoid_new() {
        let ellipsoid = GeodesyEllipsoid::named("WGS84").unwrap();

        let obj = ReferenceEllipsoid::new(ellipsoid);

        assert_eq!(obj.ellipsoid, ellipsoid);
    }

    #[test]
    fn test_reference_sphere_new() {
        let ellipsoid = GeodesyEllipsoid::named("unitsphere").unwrap();

        let obj = ReferenceSphere::new(ellipsoid);

        assert_eq!(obj.ellipsoid, ellipsoid);
    }

    #[test]
    fn test_reference_sphere_conversions() {
        let lat: f64 = 88.0;

        let ellipsoid = ReferenceSphere::new(GeodesyEllipsoid::named("sphere").unwrap());

        let actual1 = ellipsoid.latitude_geographic_to_authalic(lat);
        assert_eq!(actual1, lat);

        let actual2 = ellipsoid.latitude_authalic_to_geographic(lat);
        assert_eq!(actual2, lat);
    }

    #[test]
    fn test_reference_ellipsoid_conversions_wgs84() {
        let ellipsoid = ReferenceEllipsoid::new(GeodesyEllipsoid::named("WGS84").unwrap());

        let lat: f64 = 45.0;
        let authalic = ellipsoid.latitude_geographic_to_authalic(lat);
        let geographic = ellipsoid.latitude_authalic_to_geographic(authalic);

        assert_eq!(geographic, lat);
    }

    #[test]
    fn test_reference_ellipsoid_conversions_bessel() {
        let ellipsoid = ReferenceEllipsoid::new(GeodesyEllipsoid::named("bessel").unwrap());

        let lat: f64 = 45.0;
        let authalic = ellipsoid.latitude_geographic_to_authalic(lat);
        let geographic = ellipsoid.latitude_authalic_to_geographic(authalic);

        assert_eq!(geographic, lat);
    }
}
