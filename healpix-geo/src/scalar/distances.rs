use cdshealpix::sph_geom::coo3d::{UnitVec3, vec3_of};
use geodesy::ellps::EllipsoidBase;
use geographiclib_rs::{Geodesic, InverseGeodesic};

use crate::ellipsoid::{Ellipsoid, ReferenceBody};

/// Length of the shortest path between two points on a reference body.
///
/// On a sphere this is the great-circle distance. On an ellipsoid it is the
/// solution of the inverse geodesic problem by Karney's algorithm
/// (GeographicLib, the same algorithm as PROJ's `geod_inverse`), accurate to
/// a few nanometres for any pair of points, antipodal ones included.
#[derive(Clone, Debug)]
pub enum GeodesicSolver {
    Sphere { radius: f64 },
    Ellipsoid(Box<Geodesic>),
}

impl GeodesicSolver {
    pub fn new(ellipsoid: &Ellipsoid) -> Self {
        let body = ellipsoid.ellipsoid();

        match ellipsoid {
            Ellipsoid::Sphere(_) => Self::Sphere {
                radius: body.semimajor_axis(),
            },
            Ellipsoid::Ellipsoid(_) => Self::Ellipsoid(Box::new(Geodesic::new(
                body.semimajor_axis(),
                body.flattening(),
            ))),
        }
    }

    /// Distance in metres between two points.
    ///
    /// The points are given as `(longitude, latitude)` in degrees, where the
    /// latitude is geographic (geodetic) on the reference body.
    pub fn distance(&self, from: &(f64, f64), to: &(f64, f64)) -> f64 {
        match self {
            Self::Sphere { radius } => {
                let first = vec3_of(from.0.to_radians(), from.1.to_radians());
                let second = vec3_of(to.0.to_radians(), to.1.to_radians());

                radius * first.ang_dist(&second)
            }
            Self::Ellipsoid(geodesic) => geodesic.inverse(from.1, from.0, to.1, to.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;

    use super::*;
    use crate::ellipsoid::{ReferenceEllipsoid, ReferenceSphere};

    fn wgs84() -> Ellipsoid {
        Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(
            GeodesyEllipsoid::named("WGS84").unwrap(),
        ))
    }

    #[test]
    fn same_point_is_zero() {
        let solver = GeodesicSolver::new(&wgs84());

        assert_eq!(solver.distance(&(12.5, 41.9), &(12.5, 41.9)), 0.0);
    }

    #[test]
    fn wgs84_quarter_meridian() {
        // Equator to pole along a meridian: 10 001 965.729 m on WGS84.
        let solver = GeodesicSolver::new(&wgs84());
        let distance = solver.distance(&(0.0, 0.0), &(0.0, 90.0));

        assert!((distance - 10_001_965.729_312_724).abs() < 1e-6);
    }

    #[test]
    fn wgs84_matches_pyproj() {
        // JFK to SIN, reference value from pyproj.Geod(ellps="WGS84").inv
        let solver = GeodesicSolver::new(&wgs84());
        let distance = solver.distance(&(-73.8, 40.6), &(103.8, 1.4));

        assert!((distance - 15_346_667.397_302_179).abs() < 1e-6);
    }

    #[test]
    fn wgs84_antipodal_points() {
        // Nearly antipodal points, where Vincenty's method fails to converge.
        let solver = GeodesicSolver::new(&wgs84());
        let distance = solver.distance(&(0.0, 0.0), &(179.5, 0.5));

        // Reference value from pyproj.Geod(ellps="WGS84").inv.
        assert!((distance - 19_936_288.578_965_314).abs() < 1e-6);
    }

    #[test]
    fn sphere_is_great_circle() {
        let radius = 6_371_000.0;
        let sphere = Ellipsoid::Sphere(ReferenceSphere::new(GeodesyEllipsoid::new(radius, 0.0)));
        let solver = GeodesicSolver::new(&sphere);

        let distance = solver.distance(&(0.0, 0.0), &(90.0, 0.0));

        assert!((distance - radius * std::f64::consts::FRAC_PI_2).abs() < 1e-6);
    }
}
