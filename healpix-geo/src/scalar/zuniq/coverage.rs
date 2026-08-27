use crate::ellipsoid::Ellipsoid;
use cdshealpix as healpix;
use cdshealpix::nested::Layer;

pub fn box_coverage(
    center: (f64, f64),
    size: (f64, f64),
    angle: f64,
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    flat: bool,
) -> (Vec<u64>, Vec<bool>) {
    let (ipix_nested, depths, fully_covered) =
        crate::scalar::nested::coverage::box_coverage(center, size, angle, layer, ellipsoid, flat);

    let ipix: Vec<u64> = ipix_nested
        .into_iter()
        .zip(depths)
        .map(|(h, d)| healpix::nested::to_zuniq(d, h))
        .collect();

    (ipix, fully_covered)
}

pub fn zone_coverage(
    bbox: (f64, f64, f64, f64),
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    flat: bool,
) -> (Vec<u64>, Vec<bool>) {
    let (ipix_nested, depths, fully_covered) =
        crate::scalar::nested::coverage::zone_coverage(bbox, layer, ellipsoid, flat);

    let ipix: Vec<u64> = ipix_nested
        .into_iter()
        .zip(depths)
        .map(|(h, d)| healpix::nested::to_zuniq(d, h))
        .collect();

    (ipix, fully_covered)
}

pub fn polygon_coverage(
    vertices: &[(f64, f64)],
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    exact: bool,
    flat: bool,
) -> (Vec<u64>, Vec<bool>) {
    let (ipix_nested, depths, fully_covered) =
        crate::scalar::nested::coverage::polygon_coverage(vertices, layer, ellipsoid, exact, flat);

    let ipix: Vec<u64> = ipix_nested
        .into_iter()
        .zip(depths)
        .map(|(h, d)| healpix::nested::to_zuniq(d, h))
        .collect();

    (ipix, fully_covered)
}

pub fn cone_coverage(
    center: (f64, f64),
    radius: f64,
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    delta_depth: u8,
    flat: bool,
) -> (Vec<u64>, Vec<bool>) {
    let (ipix_nested, depths, fully_covered) = crate::scalar::nested::coverage::cone_coverage(
        center,
        radius,
        layer,
        ellipsoid,
        delta_depth,
        flat,
    );

    let ipix: Vec<u64> = ipix_nested
        .into_iter()
        .zip(depths)
        .map(|(h, d)| healpix::nested::to_zuniq(d, h))
        .collect();

    (ipix, fully_covered)
}

pub fn elliptical_cone_coverage(
    center: (f64, f64),
    ellipse_geometry: (f64, f64),
    position_angle: f64,
    layer: &Layer,
    ellipsoid: &Ellipsoid,
    delta_depth: u8,
    flat: bool,
) -> (Vec<u64>, Vec<bool>) {
    let (ipix_nested, depths, fully_covered) =
        crate::scalar::nested::coverage::elliptical_cone_coverage(
            center,
            ellipse_geometry,
            position_angle,
            layer,
            ellipsoid,
            delta_depth,
            flat,
        );

    let ipix: Vec<u64> = ipix_nested
        .into_iter()
        .zip(depths)
        .map(|(h, d)| healpix::nested::to_zuniq(d, h))
        .collect();

    (ipix, fully_covered)
}
