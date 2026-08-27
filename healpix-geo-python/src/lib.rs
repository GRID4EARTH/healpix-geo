use pyo3::prelude::*;

mod ellipsoid;
mod execution;
mod geometry;
mod index;
mod indexing_schemes;
mod mesh;
mod traits;

#[pymodule]
mod nested {
    #[pymodule_export]
    use super::index::RangeMOCIndex;

    #[pymodule_export]
    use crate::indexing_schemes::nested::{
        angular_distances, bilinear_interpolation, box_coverage, cartesian_to_healpix,
        cone_coverage, elliptical_cone_coverage, healpix_to_cartesian, healpix_to_lonlat,
        internal_boundary, kth_neighbourhood, kth_neighbours, lonlat_to_healpix, neighbours,
        polygon_coverage, siblings, to_ring, to_zuniq, vertex_indices, vertices, zone_coverage,
        zoom_to,
    };
}

#[pymodule]
mod ring {
    #[pymodule_export]
    use crate::indexing_schemes::ring::{
        angular_distances, bilinear_interpolation, box_coverage, cartesian_to_healpix,
        cone_coverage, elliptical_cone_coverage, healpix_to_cartesian, healpix_to_lonlat,
        kth_neighbourhood, kth_neighbours, lonlat_to_healpix, neighbours, polygon_coverage,
        to_nested, to_zuniq, vertex_indices, vertices, zone_coverage,
    };
}

#[pymodule]
mod zuniq {
    #[pymodule_export]
    use crate::indexing_schemes::zuniq::{
        bilinear_interpolation, box_coverage, cartesian_to_healpix, cone_coverage,
        elliptical_cone_coverage, healpix_to_cartesian, healpix_to_lonlat, kth_neighbourhood,
        kth_neighbours, lonlat_to_healpix, neighbours, polygon_coverage, to_nested, to_ring,
        vertex_indices, vertices, zone_coverage,
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

    #[pymodule_export]
    use crate::geometry::{cartesian_to_lonlat, lonlat_to_cartesian};
    #[pymodule_export]
    use crate::mesh::vertex_to_lonlat;

    #[pymodule_export]
    use crate::ellipsoid::resolve_ellipsoid;
}
