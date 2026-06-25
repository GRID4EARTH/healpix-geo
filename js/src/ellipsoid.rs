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
