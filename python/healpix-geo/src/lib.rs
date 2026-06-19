use pyo3::prelude::*;

mod ellipsoid;
mod execution;
mod geometry;
mod index;
mod indexing_schemes;

#[pymodule]
mod nested {
    #[pymodule_export]
    use super::index::RangeMOCIndex;

    #[pymodule_export]
    use crate::indexing_schemes::nested::{
        angular_distances, bilinear_interpolation, box_coverage, cone_coverage,
        elliptical_cone_coverage, healpix_to_lonlat, internal_boundary, kth_neighbourhood,
        lonlat_to_healpix, polygon_coverage, siblings, vertices, zone_coverage, zoom_to,
    };
}

#[pymodule]
mod ring {
    #[pymodule_export]
    use crate::indexing_schemes::ring::{
        angular_distances, bilinear_interpolation, box_coverage, cone_coverage,
        elliptical_cone_coverage, healpix_to_lonlat, kth_neighbourhood, lonlat_to_healpix,
        polygon_coverage, vertices, zone_coverage,
    };
}

#[pymodule]
mod zuniq {
    #[pymodule_export]
    use crate::indexing_schemes::zuniq::{
        bilinear_interpolation, box_coverage, cone_coverage, elliptical_cone_coverage, from_nested,
        healpix_to_lonlat, kth_neighbourhood, lonlat_to_healpix, polygon_coverage, to_nested,
        vertices, zone_coverage,
    };
}

#[pymodule(name = "geometry")]
mod geometry_ {
    #[pymodule_export]
    use crate::geometry::Bbox;
}

#[pymodule]
#[pyo3(name = "healpix_geo")]
mod healpix_geo {
    #[pymodule_export]
    use super::nested;

    #[pymodule_export]
    use super::ring;

    #[pymodule_export]
    use super::zuniq;

    #[pymodule_export]
    use crate::geometry_;
}
