#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use cdshealpix::nested::Layer;

use crate::ellipsoid::Ellipsoid;
use crate::maybe_parallelize;
use crate::scalar::nested::coverage as scalar;

pub type Coverage = (Vec<u64>, Vec<u8>, Vec<bool>);

/// Evaluate multiple cone coverage queries while preserving the result and
/// ordering of [`scalar::cone_coverage`] for every input center.
pub fn cone_coverage(
    centers: &[(f64, f64)],
    radius: f64,
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    delta_depth: u8,
    flat: bool,
    nthreads: usize,
) -> Vec<Coverage> {
    if centers.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::<Coverage>::with_capacity(centers.len());

    maybe_parallelize!(nthreads, centers, result, |&center| {
        scalar::cone_coverage(center, radius, layer, ellipsoid, delta_depth, flat)
    });

    result
}

// Re-export the scalar functions which do not benefit from vectorization.
#[allow(unused)]
use crate::scalar::nested::coverage::{
    box_coverage, elliptical_cone_coverage, polygon_coverage, zone_coverage,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ellipsoid::{Ellipsoid, ReferenceSphere};
    use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;

    #[test]
    fn cone_coverage_many_matches_scalar() {
        let centers = vec![(45.0, 45.0), (179.999, 0.0), (12.0, 89.9)];
        let layer = cdshealpix::nested::get(8);
        let ellipsoid = Ellipsoid::Sphere(ReferenceSphere::new(GeodesyEllipsoid::new(1.0, 0.0)));

        let expected: Vec<Coverage> = centers
            .iter()
            .map(|&center| scalar::cone_coverage(center, 0.5, layer, &ellipsoid, 1, false))
            .collect();

        assert_eq!(
            cone_coverage(&centers, 0.5, layer, &ellipsoid, 1, false, 4),
            expected
        );
    }

    #[test]
    fn cone_coverage_many_accepts_empty_input() {
        let layer = cdshealpix::nested::get(8);
        let ellipsoid = Ellipsoid::Sphere(ReferenceSphere::new(GeodesyEllipsoid::new(1.0, 0.0)));

        assert!(cone_coverage(&[], 0.5, layer, &ellipsoid, 0, true, 0).is_empty());
    }
}
