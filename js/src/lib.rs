mod coordinates;
mod geometry;

pub mod ellipsoid;
pub mod nested;
pub mod ring;
pub mod zuniq;

use wasm_bindgen::prelude::*;

pub use crate::coordinates::Coordinate;
pub use crate::ellipsoid::EllipsoidLike;

pub use crate::nested::Nested;
pub use crate::ring::Ring;
pub use crate::zuniq::Zuniq;

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
