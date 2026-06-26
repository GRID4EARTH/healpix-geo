mod coordinates;
mod geometry;

pub mod ellipsoid;
pub mod nested;
pub mod ring;
pub mod zuniq;

use wasm_bindgen::prelude::*;

pub use crate::coordinates::Coordinate;
pub use crate::ellipsoid::EllipsoidLike;

pub use crate::nested::{
    bit_combine as bit_combine_nested, healpix_to_lonlat as healpix_to_lonlat_nested,
    lonlat_to_healpix as lonlat_to_healpix_nested, vertex as vertex_nested,
};
pub use crate::ring::{
    bit_combine as bit_combine_ring, healpix_to_lonlat as healpix_to_lonlat_ring,
    lonlat_to_healpix as lonlat_to_healpix_ring, vertex as vertex_ring,
};
pub use crate::zuniq::{
    bit_combine as bit_combine_zuniq, healpix_to_lonlat as healpix_to_lonlat_zuniq,
    lonlat_to_healpix as lonlat_to_healpix_zuniq, vertex as vertex_zuniq,
};

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
