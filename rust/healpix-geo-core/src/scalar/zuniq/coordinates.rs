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

    healpix::nested::to_zuniq(layer.depth(), hash_nested)
}

pub fn vertices(hash: &u64, ellipsoid: &Ellipsoid, step: &usize) -> Vec<(f64, f64)> {
    let (depth, hash_nested) = healpix::nested::from_zuniq(*hash);
    let layer = healpix::nested::get(depth);

    crate::scalar::nested::coordinates::vertices(&hash_nested, layer, ellipsoid, step)
}
