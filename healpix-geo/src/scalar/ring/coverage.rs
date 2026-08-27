use crate::ellipsoid::Ellipsoid;
use cdshealpix as healpix;
use itertools::{MultiUnzip, izip};

pub fn box_coverage(
    center: (f64, f64),
    size: (f64, f64),
    angle: f64,
    nside: &u32,
    ellipsoid: &Ellipsoid,
    flat: bool,
) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    let layer = healpix::nested::get(healpix::depth(*nside));

    let (ipix, depths, fully_covered) =
        crate::scalar::nested::coverage::box_coverage(center, size, angle, layer, ellipsoid, flat);

    let mut result: Vec<(u64, u8, bool)> = izip!(
        ipix.into_iter(),
        depths.into_iter(),
        fully_covered.into_iter()
    )
    .map(|(h, d, f)| (healpix::nested::get(d).to_ring(h), d, f))
    .collect::<Vec<_>>();
    result.sort_by_key(|it| it.0);

    result.into_iter().multiunzip()
}

pub fn zone_coverage(
    bbox: (f64, f64, f64, f64),
    nside: &u32,
    ellipsoid: &Ellipsoid,
    flat: bool,
) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    let layer = healpix::nested::get(healpix::depth(*nside));

    let (ipix, depths, fully_covered) =
        crate::scalar::nested::coverage::zone_coverage(bbox, layer, ellipsoid, flat);

    let mut result: Vec<(u64, u8, bool)> = izip!(
        ipix.into_iter(),
        depths.into_iter(),
        fully_covered.into_iter()
    )
    .map(|(h, d, f)| (healpix::nested::get(d).to_ring(h), d, f))
    .collect::<Vec<_>>();
    result.sort_by_key(|it| it.0);

    result.into_iter().multiunzip()
}

pub fn polygon_coverage(
    vertices: &[(f64, f64)],
    nside: &u32,
    ellipsoid: &Ellipsoid,
    exact: bool,
    flat: bool,
) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    let layer = healpix::nested::get(healpix::depth(*nside));

    let (ipix, depths, fully_covered) =
        crate::scalar::nested::coverage::polygon_coverage(vertices, layer, ellipsoid, exact, flat);

    let mut result: Vec<(u64, u8, bool)> = izip!(
        ipix.into_iter(),
        depths.into_iter(),
        fully_covered.into_iter()
    )
    .map(|(h, d, f)| (healpix::nested::get(d).to_ring(h), d, f))
    .collect::<Vec<_>>();
    result.sort_by_key(|it| it.0);

    result.into_iter().multiunzip()
}

pub fn cone_coverage(
    center: (f64, f64),
    radius: f64,
    nside: &u32,
    ellipsoid: &Ellipsoid,
    delta_depth: u8,
    flat: bool,
) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    let layer = healpix::nested::get(healpix::depth(*nside));

    let (ipix, depths, fully_covered) = crate::scalar::nested::coverage::cone_coverage(
        center,
        radius,
        layer,
        ellipsoid,
        delta_depth,
        flat,
    );

    let mut result: Vec<(u64, u8, bool)> = izip!(
        ipix.into_iter(),
        depths.into_iter(),
        fully_covered.into_iter()
    )
    .map(|(h, d, f)| (healpix::nested::get(d).to_ring(h), d, f))
    .collect::<Vec<_>>();
    result.sort_by_key(|it| it.0);

    result.into_iter().multiunzip()
}

pub fn elliptical_cone_coverage(
    center: (f64, f64),
    ellipse_geometry: (f64, f64),
    position_angle: f64,
    nside: &u32,
    ellipsoid: &Ellipsoid,
    delta_depth: u8,
    flat: bool,
) -> (Vec<u64>, Vec<u8>, Vec<bool>) {
    let layer = healpix::nested::get(healpix::depth(*nside));

    let (ipix, depths, fully_covered) = crate::scalar::nested::coverage::elliptical_cone_coverage(
        center,
        ellipse_geometry,
        position_angle,
        layer,
        ellipsoid,
        delta_depth,
        flat,
    );

    let mut result: Vec<(u64, u8, bool)> = izip!(
        ipix.into_iter(),
        depths.into_iter(),
        fully_covered.into_iter()
    )
    .map(|(h, d, f)| (healpix::nested::get(d).to_ring(h), d, f))
    .collect::<Vec<_>>();
    result.sort_by_key(|it| it.0);

    result.into_iter().multiunzip()
}
