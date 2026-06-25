use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;
use healpix_geo_core::ellipsoid::{
    Ellipsoid as RustEllipsoid, ReferenceEllipsoid, ReferenceSphere,
};
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Debug)]
#[wasm_bindgen]
pub enum EllipsoidLike {
    Ellipsoid(Ellipsoid),
    Sphere(Sphere),
}

#[derive(Deserialize, Debug)]
#[wasm_bindgen]
pub struct Ellipsoid {
    pub semi_major_axis: f64,
    pub inverse_flattening: f64,
}

#[derive(Deserialize, Debug)]
struct EllipsoidSemiMinor {
    pub semi_major_axis: f64,
    pub semi_minor_axis: f64,
}

impl From<EllipsoidSemiMinor> for Ellipsoid {
    fn from(val: EllipsoidSemiMinor) -> Ellipsoid {
        let a = val.semi_major_axis;
        let b = val.semi_minor_axis;

        Ellipsoid {
            semi_major_axis: a,
            inverse_flattening: a / (a - b),
        }
    }
}

#[derive(Deserialize, Debug)]
#[wasm_bindgen]
pub struct Sphere {
    pub radius: f64,
}

#[wasm_bindgen(js_name = parseEllipsoid)]
pub fn parse_ellipsoid(obj: JsValue) -> Result<EllipsoidLike, JsValue> {
    println!("received: {obj:?}");

    let parsed = from_value(obj)?;

    Ok(parsed)
}

impl EllipsoidLike {
    pub fn into_ellipsoid(self) -> RustEllipsoid {
        match self {
            Self::Ellipsoid(ell) => {
                let ellipsoid =
                    GeodesyEllipsoid::new(ell.semi_major_axis, 1.0f64 / ell.inverse_flattening);

                RustEllipsoid::Ellipsoid(ReferenceEllipsoid::new(ellipsoid))
            }
            Self::Sphere(sphere) => {
                let ellipsoid = GeodesyEllipsoid::new(sphere.radius, 0.0f64);

                RustEllipsoid::Sphere(ReferenceSphere::new(ellipsoid))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geodesy::prelude::EllipsoidBase;
    use healpix_geo_core::ellipsoid::ReferenceBody;

    #[test]
    fn test_ellipsoidlike_to_ellipsoid() {
        let a: f64 = 6378137.0;
        let if_: f64 = 298.257223563;
        let f: f64 = 1.0 / if_;

        let obj = EllipsoidLike::Ellipsoid(Ellipsoid {
            semi_major_axis: a,
            inverse_flattening: if_,
        });

        let actual = obj.into_ellipsoid();
        match actual {
            RustEllipsoid::Ellipsoid(ell) => {
                let unpacked = ell.ellipsoid();

                assert_eq!(unpacked.semimajor_axis(), a);
                assert_eq!(unpacked.flattening(), f);
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests_wasm32 {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::to_value;
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    enum Value {
        String(String),
        Float(f64),
    }

    #[test]
    fn test_parse_ellipsoid_ellipsoid() {
        let mut map = HashMap::new();
        let a: f64 = 6378137.0;
        let if_: f64 = 298.257223563;
        let name = "WGS84".to_string();

        map.insert("name".to_string(), Value::String(name));
        map.insert("semi_major_axis".to_string(), Value::Float(a));
        map.insert("inverse_flattening".to_string(), Value::Float(if_));

        let obj = to_value(&map);

        let actual: EllipsoidLike = parse_ellipsoid(obj).unwrap();
        match actual {
            EllipsoidLike::Ellipsoid(ell) => {
                assert_eq!(ell.semi_major_axis, map["semi_major_axis"]);
                assert_eq!(ell.inverse_flattening, map["inverse_flattening"]);
            }
            _ => unreachable!(),
        }
    }
}
