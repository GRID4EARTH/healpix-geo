use geodesy::ellps::Ellipsoid as GeoEllipsoid;
use healpix_geo::ellipsoid::{Ellipsoid, ReferenceBody, ReferenceEllipsoid, ReferenceSphere};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyDictMethods};

#[derive(FromPyObject)]
pub(crate) enum EllipsoidLike {
    Named(String),
    EllipsoidParameters {
        #[pyo3(item("semimajor_axis"))]
        semimajor_axis: f64,
        #[pyo3(item("inverse_flattening"))]
        inverse_flattening: f64,
    },
    SphereParameters {
        #[pyo3(item("radius"))]
        radius: f64,
    },
    EllipsoidObject {
        #[pyo3(attribute("semimajor_axis"))]
        semimajor_axis: f64,
        #[pyo3(attribute("inverse_flattening"))]
        inverse_flattening: f64,
    },
    SphereObject {
        #[pyo3(attribute("radius"))]
        radius: f64,
    },
}

impl EllipsoidLike {
    pub fn into_ellipsoid(self) -> PyResult<Ellipsoid> {
        match self {
            Self::Named(name) => {
                let ellipsoid =
                    GeoEllipsoid::named(&name).map_err(|e| PyValueError::new_err(e.to_string()))?;

                if name.contains("sphere") {
                    Ok(Ellipsoid::Sphere(ReferenceSphere::new(ellipsoid)))
                } else {
                    Ok(Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(ellipsoid)))
                }
            }
            Self::EllipsoidParameters {
                semimajor_axis,
                inverse_flattening,
            }
            | Self::EllipsoidObject {
                semimajor_axis,
                inverse_flattening,
            } => {
                if inverse_flattening >= 2.0 && semimajor_axis > 0.0 {
                    let ellipsoid = GeoEllipsoid::new(semimajor_axis, 1.0f64 / inverse_flattening);

                    Ok(Ellipsoid::Ellipsoid(ReferenceEllipsoid::new(ellipsoid)))
                } else if inverse_flattening < 2.0 {
                    Err(PyValueError::new_err(format!(
                        "The inverse flattening must be greater than or equal to 2, but got {:?}.",
                        inverse_flattening,
                    )))
                } else {
                    Err(PyValueError::new_err(format!(
                        "The semimajor axis must be greater than 0, but got {:?}.",
                        semimajor_axis
                    )))
                }
            }
            Self::SphereParameters { radius } | EllipsoidLike::SphereObject { radius } => {
                if radius > 0.0 {
                    let ellipsoid = GeoEllipsoid::new(radius, 0.0f64);
                    Ok(Ellipsoid::Sphere(ReferenceSphere::new(ellipsoid)))
                } else {
                    Err(PyValueError::new_err(format!(
                        "The radius must be greater than 0, but got {:?}.",
                        radius
                    )))
                }
            }
        }
    }
}

#[pyfunction]
pub(crate) fn resolve_ellipsoid<'py>(
    py: Python<'py>,
    name: String,
) -> PyResult<Bound<'py, PyDict>> {
    let mapping = PyDict::new(py);
    mapping.set_item("name", &name)?;

    let ellipsoid = EllipsoidLike::Named(name).into_ellipsoid()?;

    let mut parameters = ellipsoid.to_mapping();
    if let Some(v) = parameters.remove("flattening") {
        parameters.insert("inverse_flattening".to_string(), 1.0 / v);
    }
    mapping.update(parameters.into_py_dict(py)?.as_mapping())?;

    Ok(mapping)
}
