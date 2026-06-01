use pyo3::exceptions::{PyImportError, PyNotImplementedError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple, PyType};

use healpix_geo_core::geometry::{
    BoundingBox as HgBoundingBox, Geometry, Point as HgPoint, Polygon as HgPolygon,
};

/// bounding box
#[derive(PartialEq, PartialOrd, Debug, Clone)]
#[pyclass]
#[pyo3(module = "healpix_geo.geometry", frozen)]
pub struct Bbox {
    #[pyo3(get)]
    pub lon_min: f64,
    #[pyo3(get)]
    pub lat_min: f64,
    #[pyo3(get)]
    pub lon_max: f64,
    #[pyo3(get)]
    pub lat_max: f64,
}

#[pymethods]
impl Bbox {
    #[new]
    fn new(lon_min: f64, lat_min: f64, lon_max: f64, lat_max: f64) -> Self {
        Self {
            lon_min,
            lat_min,
            lon_max,
            lat_max,
        }
    }

    #[classmethod]
    fn from_tuple<'py>(
        _cls: &Bound<'py, PyType>,
        _py: Python<'py>,
        bbox: &Bound<'py, PyTuple>,
    ) -> PyResult<Self> {
        let bbox_ = bbox.extract::<(f64, f64, f64, f64)>()?;

        Ok(Self {
            lon_min: bbox_.0,
            lat_min: bbox_.1,
            lon_max: bbox_.2,
            lat_max: bbox_.3,
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "Bbox({0}, {1}, {2}, {3})",
            self.lon_min, self.lat_min, self.lon_max, self.lat_max
        )
    }
}

enum ShapelyGeometryTypes {
    Point,
    LineString,
    Polygon,
}

impl ShapelyGeometryTypes {
    fn from_string(kind: String) -> PyResult<Self> {
        if kind == "Point" {
            Ok(Self::Point)
        } else if kind == "LineString" {
            Ok(Self::LineString)
        } else if kind == "Polygon" {
            Ok(Self::Polygon)
        } else {
            Err(PyValueError::new_err("unsupported geometry type: {kind}"))
        }
    }
}

pub enum GeometryTypes {
    Point(f64, f64),
    #[allow(dead_code)]
    LineString(Vec<(f64, f64)>),
    Polygon(Vec<(f64, f64)>, Vec<Vec<(f64, f64)>>),
    Bbox(f64, f64, f64, f64),
}

impl GeometryTypes {
    pub fn from_pyobject<'py>(py: Python<'py>, obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if obj.is_instance_of::<Bbox>() {
            let bbox = obj.extract::<Bbox>()?;

            Ok(Self::Bbox(
                bbox.lon_min,
                bbox.lat_min,
                bbox.lon_max,
                bbox.lat_max,
            ))
        } else {
            let shapely = match py.import("shapely") {
                Ok(module) => Ok(module),
                Err(err) => {
                    if err.is_instance_of::<PyImportError>(py) {
                        return Err(PyTypeError::new_err(
                            "Object other than Bbox found, and cannot import shapely.",
                        ));
                    }

                    Err(err)
                }
            }?;

            let geometry_type = shapely.getattr("Geometry")?;

            if !obj.is_instance(&geometry_type)? {
                return Err(PyTypeError::new_err(
                    "need to pass a Bbox object or a shapely geometry",
                ));
            }

            let geom_type = ShapelyGeometryTypes::from_string(
                obj.getattr("geom_type")?
                    .extract::<Bound<'_, PyString>>()?
                    .to_string(),
            )?;
            match geom_type {
                ShapelyGeometryTypes::Point => {
                    let (lon, lat) = obj
                        .getattr("coords")?
                        .get_item(0)?
                        .extract::<(f64, f64)>()?;
                    Ok(GeometryTypes::Point(lon, lat))
                }
                ShapelyGeometryTypes::LineString => {
                    let coords = obj.getattr("coords")?.extract::<Vec<(_, _)>>()?;

                    Ok(GeometryTypes::LineString(coords))
                }
                ShapelyGeometryTypes::Polygon => {
                    let exterior = obj
                        .getattr("exterior")?
                        .getattr("coords")?
                        .extract::<Vec<(f64, f64)>>()?;

                    let interiors = obj
                        .getattr("interiors")?
                        .extract::<Vec<Vec<(f64, f64)>>>()?;

                    Ok(GeometryTypes::Polygon(exterior, interiors))
                }
            }
        }
    }

    pub fn into_geometry(self) -> PyResult<Geometry> {
        let geom: Geometry = match self {
            Self::Point(x, y) => {
                let p = HgPoint::from_tuple((x, y));

                Geometry::Point(p)
            }
            Self::LineString(_) => {
                return Err(PyNotImplementedError::new_err(
                    "line strings are not supported, yet",
                ));
            }
            Self::Polygon(exterior, interiors) => {
                if !interiors.is_empty() {
                    return Err(PyNotImplementedError::new_err(
                        "interior rings are not supported, yet",
                    ));
                }
                let p = HgPolygon::create(exterior);

                Geometry::Polygon(p)
            }
            Self::Bbox(lon_min, lat_min, lon_max, lat_max) => {
                let bbox = HgBoundingBox {
                    lon_min,
                    lat_min,
                    lon_max,
                    lat_max,
                };

                Geometry::BoundingBox(bbox)
            }
        };

        Ok(geom)
    }
}
