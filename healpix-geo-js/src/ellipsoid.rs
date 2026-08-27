use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;
use geodesy::prelude::EllipsoidBase;
use healpix_geo::ellipsoid::{
    Ellipsoid as RustEllipsoid, ReferenceBody, ReferenceEllipsoid, ReferenceSphere,
};
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

/// The plain-object shapes accepted wherever an ellipsoid is expected.
///
/// Mirrors [`EllipsoidLike`], which is an input (deserialization) type only
/// and is therefore not exported as a class.
///
/// The `{ [key: string]: unknown }` intersection is what makes the documented
/// "extra keys are ignored" behavior type-check: without it TypeScript's
/// excess-property check rejects a fresh object literal carrying the `name` /
/// `epsg` keys that ellipsoid catalogues ship with.
#[wasm_bindgen(typescript_custom_section)]
const ELLIPSOID_INPUT: &'static str = r#"
export type EllipsoidInput = { [key: string]: unknown } & (
    | { radius: number }
    | { semi_major_axis: number; inverse_flattening: number }
    | { semi_major_axis: number; semi_minor_axis: number }
);
"#;

/// Plain-object shapes accepted as ellipsoid definitions.
///
/// This is an input (deserialization) type only; the parsed, reusable state
/// lives in [`Ellipsoid`]. The TypeScript view of it is `EllipsoidInput`
/// (see the custom section above).
#[derive(Deserialize, Debug, PartialEq)]
pub enum EllipsoidLike {
    #[serde(untagged)]
    EllipsoidInverseFlattening(EllipsoidInverseFlattening),
    #[serde(untagged)]
    EllipsoidSemiMinorAxis(EllipsoidSemiMinorAxis),
    #[serde(untagged)]
    Sphere(Sphere),
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct EllipsoidInverseFlattening {
    pub semi_major_axis: f64,
    pub inverse_flattening: f64,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct EllipsoidSemiMinorAxis {
    pub semi_major_axis: f64,
    pub semi_minor_axis: f64,
}

impl From<EllipsoidSemiMinorAxis> for EllipsoidInverseFlattening {
    fn from(val: EllipsoidSemiMinorAxis) -> EllipsoidInverseFlattening {
        let a = val.semi_major_axis;
        let b = val.semi_minor_axis;

        EllipsoidInverseFlattening {
            semi_major_axis: a,
            inverse_flattening: a / (a - b),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Sphere {
    pub radius: f64,
}

impl Default for Sphere {
    fn default() -> Sphere {
        Sphere { radius: 6370997.0 }
    }
}

/// Reject an axis that is not a positive, finite length.
fn check_axis(value: f64, name: &str) -> Result<(), String> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(format!(
            "`{}` must be a positive, finite number, got {}",
            name, value
        ))
    }
}

impl EllipsoidLike {
    /// Reject reference bodies that are not physically meaningful.
    ///
    /// `geodesy` builds an ellipsoid out of whatever numbers it is handed: a
    /// zero or negative axis goes straight through, and
    /// `inverse_flattening: 0` yields `flattening: Infinity` rather than the
    /// sphere the caller almost certainly meant. Neither is caught downstream,
    /// so the check belongs here, at the parse boundary.
    fn validate(&self) -> Result<(), String> {
        match self {
            Self::EllipsoidInverseFlattening(ell) => {
                check_axis(ell.semi_major_axis, "semi_major_axis")?;

                // `<= 1` also covers NaN by way of the explicit test: a
                // flattening of 1 or more collapses the body onto its axis
                if ell.inverse_flattening.is_nan() || ell.inverse_flattening <= 1.0 {
                    return Err(format!(
                        "`inverse_flattening` must be greater than 1, got {} (use `{{ radius }}` for a sphere)",
                        ell.inverse_flattening
                    ));
                }
            }
            Self::EllipsoidSemiMinorAxis(ell) => {
                check_axis(ell.semi_major_axis, "semi_major_axis")?;
                check_axis(ell.semi_minor_axis, "semi_minor_axis")?;

                if ell.semi_minor_axis > ell.semi_major_axis {
                    return Err(format!(
                        "`semi_minor_axis` ({}) must not exceed `semi_major_axis` ({})",
                        ell.semi_minor_axis, ell.semi_major_axis
                    ));
                }
            }
            Self::Sphere(sphere) => check_axis(sphere.radius, "radius")?,
        }

        Ok(())
    }

    pub fn into_ellipsoid(self) -> Result<RustEllipsoid, String> {
        self.validate()?;

        Ok(match self {
            Self::EllipsoidInverseFlattening(ell) => {
                let ellipsoid =
                    GeodesyEllipsoid::new(ell.semi_major_axis, 1.0f64 / ell.inverse_flattening);

                RustEllipsoid::Ellipsoid(ReferenceEllipsoid::new(ellipsoid))
            }
            Self::EllipsoidSemiMinorAxis(ell) => {
                let a = ell.semi_major_axis;
                let b = ell.semi_minor_axis;
                let flattening = (a - b) / a;
                let ellipsoid = GeodesyEllipsoid::new(a, flattening);

                RustEllipsoid::Ellipsoid(ReferenceEllipsoid::new(ellipsoid))
            }
            Self::Sphere(sphere) => {
                let ellipsoid = GeodesyEllipsoid::new(sphere.radius, 0.0f64);

                RustEllipsoid::Sphere(ReferenceSphere::new(ellipsoid))
            }
        })
    }
}

/// A parsed, reusable reference body.
///
/// The handle is passed to functions by reference and can be reused for any
/// number of calls. Parsing and the authalic-latitude Fourier coefficient
/// computation happen once, at construction.
#[wasm_bindgen]
pub struct Ellipsoid {
    pub(crate) inner: RustEllipsoid,
}

impl Default for Ellipsoid {
    /// The default sphere (radius 6370997 m), equivalent to
    /// `Ellipsoid.from(null)`.
    fn default() -> Ellipsoid {
        Ellipsoid {
            inner: RustEllipsoid::default(),
        }
    }
}

#[wasm_bindgen]
impl Ellipsoid {
    /// Parse a plain object (or `null` for the default sphere) into a
    /// reusable ellipsoid handle.
    ///
    /// Accepted shapes:
    /// - `null` or `undefined` (including a missing argument) — the default
    ///   sphere (radius 6370997 m)
    /// - `{ radius }` — a sphere
    /// - `{ semi_major_axis, inverse_flattening }`
    /// - `{ semi_major_axis, semi_minor_axis }`
    ///
    /// Note that `undefined` is accepted as well as `null`. Extra keys
    /// (e.g. `name`) are ignored.
    ///
    /// The axes have to be positive, finite lengths, `semi_minor_axis` must
    /// not exceed `semi_major_axis`, and `inverse_flattening` must be greater
    /// than 1 — `inverse_flattening: 0` means an infinite flattening, not a
    /// sphere, and is rejected; use `{ radius }` for a sphere. Anything else
    /// throws a JS `Error`.
    #[wasm_bindgen(js_name = from)]
    pub fn from_value(
        #[wasm_bindgen(unchecked_param_type = "EllipsoidInput | null | undefined")] obj: JsValue,
    ) -> Result<Ellipsoid, JsValue> {
        let inner = if obj.is_null() || obj.is_undefined() {
            RustEllipsoid::default()
        } else {
            let parsed: EllipsoidLike = from_value(obj)?;
            parsed
                .into_ellipsoid()
                .map_err(|message| JsError::new(&message))?
        };

        Ok(Ellipsoid { inner })
    }

    /// Semi-major axis (the radius, for a sphere) in meters
    #[wasm_bindgen(getter, js_name = semiMajorAxis)]
    pub fn semi_major_axis(&self) -> f64 {
        self.inner.ellipsoid().semimajor_axis()
    }

    /// Flattening (0 for a sphere)
    #[wasm_bindgen(getter)]
    pub fn flattening(&self) -> f64 {
        self.inner.ellipsoid().flattening()
    }

    /// Whether this reference body is a sphere
    #[wasm_bindgen(getter, js_name = isSphere)]
    pub fn is_sphere(&self) -> bool {
        matches!(self.inner, RustEllipsoid::Sphere(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipsoidlike_to_ellipsoid() {
        let a: f64 = 6378137.0;
        let if_: f64 = 298.257223563;
        let f: f64 = 1.0 / if_;

        let obj = EllipsoidLike::EllipsoidInverseFlattening(EllipsoidInverseFlattening {
            semi_major_axis: a,
            inverse_flattening: if_,
        });

        let actual = obj.into_ellipsoid().unwrap();
        match actual {
            RustEllipsoid::Ellipsoid(ell) => {
                let unpacked = ell.ellipsoid();

                assert_eq!(unpacked.semimajor_axis(), a);
                assert_eq!(unpacked.flattening(), f);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_deserialize_ellipsoid() {
        let a = 6378137.0;
        let if_ = 298.257223563;

        let json = r#"{"name": "WGS84", "semi_major_axis": 6378137.0, "inverse_flattening": 298.257223563}"#;

        let actual: EllipsoidLike = serde_json::from_str(json).unwrap();
        let expected = EllipsoidLike::EllipsoidInverseFlattening(EllipsoidInverseFlattening {
            semi_major_axis: a,
            inverse_flattening: if_,
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_ellipsoid_semiminor() {
        let a = 6378137.0;
        let b = 6356752.314245179;

        let json = r#"{"name": "WGS84", "semi_major_axis": 6378137.0, "semi_minor_axis": 6356752.314245179}"#;

        let actual: EllipsoidLike = serde_json::from_str(json).unwrap();
        let expected = EllipsoidLike::EllipsoidSemiMinorAxis(EllipsoidSemiMinorAxis {
            semi_major_axis: a,
            semi_minor_axis: b,
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_sphere() {
        let json = r#"{"name": "sphere", "radius": 6370997.0}"#;

        let actual: EllipsoidLike = serde_json::from_str(json).unwrap();
        let expected = EllipsoidLike::Sphere(Sphere { radius: 6370997.0 });

        assert_eq!(actual, expected);
    }

    fn inverse_flattening(semi_major_axis: f64, inverse_flattening: f64) -> EllipsoidLike {
        EllipsoidLike::EllipsoidInverseFlattening(EllipsoidInverseFlattening {
            semi_major_axis,
            inverse_flattening,
        })
    }

    fn semi_minor(semi_major_axis: f64, semi_minor_axis: f64) -> EllipsoidLike {
        EllipsoidLike::EllipsoidSemiMinorAxis(EllipsoidSemiMinorAxis {
            semi_major_axis,
            semi_minor_axis,
        })
    }

    #[test]
    fn test_rejects_degenerate_spheres() {
        for radius in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(
                EllipsoidLike::Sphere(Sphere { radius })
                    .into_ellipsoid()
                    .is_err(),
                "radius {}",
                radius
            );
        }
    }

    #[test]
    fn test_default_sphere() {
        assert!(
            EllipsoidLike::Sphere(Sphere::default())
                .into_ellipsoid()
                .is_ok()
        );
    }

    #[test]
    fn test_rejects_degenerate_inverse_flattening() {
        // `0` is the plausible-looking way to *mean* a sphere; it produces
        // `flattening: Infinity` instead, so it is rejected outright
        for value in [0.0, 1.0, -298.257223563, f64::NAN] {
            assert!(
                inverse_flattening(6378137.0, value)
                    .into_ellipsoid()
                    .is_err(),
                "inverse_flattening {}",
                value
            );
        }

        assert!(
            inverse_flattening(0.0, 298.257223563)
                .into_ellipsoid()
                .is_err()
        );
        assert!(
            inverse_flattening(-6378137.0, 298.257223563)
                .into_ellipsoid()
                .is_err()
        );
        assert!(
            inverse_flattening(6378137.0, 298.257223563)
                .into_ellipsoid()
                .is_ok()
        );
    }

    #[test]
    fn test_rejects_degenerate_semi_minor_axis() {
        assert!(semi_minor(6378137.0, 0.0).into_ellipsoid().is_err());
        assert!(semi_minor(6378137.0, -1.0).into_ellipsoid().is_err());
        // an oblate spheroid is what the shape describes; b > a is not one
        assert!(
            semi_minor(6356752.314245179, 6378137.0)
                .into_ellipsoid()
                .is_err()
        );

        assert!(
            semi_minor(6378137.0, 6356752.314245179)
                .into_ellipsoid()
                .is_ok()
        );
        // a == b is a sphere expressed the long way, and stays legal
        assert!(semi_minor(6378137.0, 6378137.0).into_ellipsoid().is_ok());
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
pub mod tests_wasm32 {
    use super::*;
    use serde_json::json;
    use serde_wasm_bindgen::to_value;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_ellipsoid_from_ellipsoid() {
        let a: f64 = 6378137.0;
        let if_: f64 = 298.257223563;

        let data = json!({"name": "WGS84", "semi_major_axis": a, "inverse_flattening": if_});
        let obj = to_value(&data).map_err(JsValue::from).unwrap();

        let actual = Ellipsoid::from_value(obj).map_err(JsValue::from).unwrap();
        assert_eq!(actual.semi_major_axis(), a);
        assert!(!actual.is_sphere());
    }
}
