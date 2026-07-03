use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub lon: f64,
    pub lat: f64,
}

#[wasm_bindgen]
impl Coordinate {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Coordinate {{ lon: {}, lat: {} }}", self.lon, self.lat)
    }
}
