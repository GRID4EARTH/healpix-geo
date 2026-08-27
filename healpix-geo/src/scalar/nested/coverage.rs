use crate::ellipsoid::{Ellipsoid, ReferenceBody};
use cdshealpix::nested::Layer;
use cdshealpix::nested::bmoc::BMOC;

fn get_cells(bmoc: BMOC) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    let len = bmoc.entries.len();

    let mut ipix = Vec::<u64>::with_capacity(len);
    let mut depth = Vec::<u8>::with_capacity(len);
    let mut fully_covered = Vec::<bool>::with_capacity(len);

    for c in bmoc.into_iter() {
        ipix.push(c.hash);
        depth.push(c.depth);
        fully_covered.push(c.is_full);
    }

    depth.shrink_to_fit();
    ipix.shrink_to_fit();
    fully_covered.shrink_to_fit();

    (ipix, depth, fully_covered)
}

fn get_flat_cells(bmoc: BMOC) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    let len = bmoc.deep_size();
    let mut ipix = Vec::<u64>::with_capacity(len);
    let mut depth = Vec::<u8>::with_capacity(len);
    let mut fully_covered = Vec::<bool>::with_capacity(len);

    for c in bmoc.flat_iter_cell() {
        ipix.push(c.hash);
        depth.push(c.depth);
        fully_covered.push(c.is_full);
    }

    depth.shrink_to_fit();
    ipix.shrink_to_fit();
    fully_covered.shrink_to_fit();

    (ipix, depth, fully_covered)
}

pub fn zone_coverage(
    bbox: (f64, f64, f64, f64),
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    flat: bool,
) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    let (lon_min, lat_min, lon_max, lat_max) = bbox;

    let bmoc = layer.zone_coverage(
        lon_min.rem_euclid(360.0).to_radians(),
        ellipsoid.latitude_geographic_to_authalic(lat_min.to_radians()),
        lon_max.rem_euclid(360.0).to_radians(),
        ellipsoid.latitude_geographic_to_authalic(lat_max.to_radians()),
    );

    if flat {
        get_flat_cells(bmoc)
    } else {
        get_cells(bmoc)
    }
}

pub fn box_coverage(
    center: (f64, f64),
    size: (f64, f64),
    angle: f64,
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    flat: bool,
) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    let (lon, lat) = center;
    let (size_lon, size_lat) = size;

    let bmoc = layer.box_coverage(
        lon.rem_euclid(360.0).to_radians(),
        ellipsoid.latitude_geographic_to_authalic(lat.to_radians()),
        size_lon.rem_euclid(360.0).to_radians(),
        size_lat.to_radians(),
        angle.to_radians(),
    );

    if flat {
        get_flat_cells(bmoc)
    } else {
        get_cells(bmoc)
    }
}

pub fn polygon_coverage(
    vertices: &[(f64, f64)],
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    exact: bool,
    flat: bool,
) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    let converted_vertices: Vec<(f64, f64)> = vertices
        .iter()
        .map(|v| {
            let (lon, lat) = v;

            (
                lon.rem_euclid(360.0).to_radians(),
                ellipsoid.latitude_geographic_to_authalic(lat.to_radians()),
            )
        })
        .collect();

    let bmoc = layer.polygon_coverage(&converted_vertices, exact);

    if flat {
        get_flat_cells(bmoc)
    } else {
        get_cells(bmoc)
    }
}

pub fn cone_coverage(
    center: (f64, f64),
    radius: f64,
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    delta_depth: u8,
    flat: bool,
) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    if layer.depth() + delta_depth > 29 {
        // TODO: return a Result object
        panic!("delta_depth must be chosen such that layer.depth() + delta_depth <= 29");
    }

    let (lon, lat) = center;

    let bmoc = layer.cone_coverage_approx_custom(
        delta_depth,
        lon.rem_euclid(360.0).to_radians(),
        ellipsoid.latitude_geographic_to_authalic(lat.to_radians()),
        radius.to_radians(),
    );

    if flat {
        get_flat_cells(bmoc)
    } else {
        get_cells(bmoc)
    }
}

pub fn elliptical_cone_coverage(
    center: (f64, f64),
    ellipse_geometry: (f64, f64),
    position_angle: f64,
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    delta_depth: u8,
    flat: bool,
) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    if layer.depth() + delta_depth > 29 {
        // TODO: return a Result object
        panic!("delta_depth must be chosen such that layer.depth() + delta_depth <= 29");
    }

    let (lon, lat) = center;

    let (a, b) = ellipse_geometry;

    let bmoc = layer.elliptical_cone_coverage_custom(
        delta_depth,
        lon.rem_euclid(360.0).to_radians(),
        ellipsoid.latitude_geographic_to_authalic(lat.to_radians()),
        a.to_radians(),
        b.to_radians(),
        position_angle.to_radians(),
    );

    if flat {
        get_flat_cells(bmoc)
    } else {
        get_cells(bmoc)
    }
}
