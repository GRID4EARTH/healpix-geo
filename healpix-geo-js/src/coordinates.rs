use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub lon: f64,
    pub lat: f64,
}
