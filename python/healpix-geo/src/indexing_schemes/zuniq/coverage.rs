use crate::ellipsoid::EllipsoidLike;
use cdshealpix as healpix;
use numpy::{PyArray1, PyArray2, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use healpix_geo_core::scalar::zuniq::coverage as scalar;

#[allow(clippy::type_complexity)]
#[pyfunction]
#[pyo3(signature = (depth, bbox, *, ellipsoid, flat = true))]
pub(crate) fn zone_coverage<'py>(
    py: Python<'py>,
    depth: u8,
    bbox: (f64, f64, f64, f64),
    ellipsoid: EllipsoidLike,
    flat: bool,
) -> PyResult<(Bound<'py, PyArray1<u64>>, Bound<'py, PyArray1<bool>>)> {
    let ellipsoid_ = ellipsoid.into_ellipsoid()?;
    let layer = healpix::nested::get(depth);

    let (ipix, fully_covered) = scalar::zone_coverage(bbox, layer, &ellipsoid_, flat);

    Ok((
        PyArray1::from_vec(py, ipix),
        PyArray1::from_vec(py, fully_covered),
    ))
}

#[allow(clippy::type_complexity)]
#[pyfunction]
#[pyo3(signature = (depth, center, size, angle, *, ellipsoid, flat = true))]
pub(crate) fn box_coverage<'py>(
    py: Python<'py>,
    depth: u8,
    center: (f64, f64),
    size: (f64, f64),
    angle: f64,
    ellipsoid: EllipsoidLike,
    flat: bool,
) -> PyResult<(Bound<'py, PyArray1<u64>>, Bound<'py, PyArray1<bool>>)> {
    let ellipsoid_ = ellipsoid.into_ellipsoid()?;
    let layer = healpix::nested::get(depth);

    let (ipix, fully_covered) = scalar::box_coverage(center, size, angle, layer, &ellipsoid_, flat);

    Ok((
        PyArray1::from_vec(py, ipix),
        PyArray1::from_vec(py, fully_covered),
    ))
}

#[allow(clippy::type_complexity)]
#[pyfunction]
#[pyo3(signature = (depth, vertices, *, ellipsoid, exact = false, flat = true))]
pub(crate) fn polygon_coverage<'py>(
    py: Python<'py>,
    depth: u8,
    vertices: &Bound<PyArray2<f64>>,
    ellipsoid: EllipsoidLike,
    exact: bool,
    flat: bool,
) -> PyResult<(Bound<'py, PyArray1<u64>>, Bound<'py, PyArray1<bool>>)> {
    let ellipsoid_ = ellipsoid.into_ellipsoid()?;
    let layer = healpix::nested::get(depth);

    let shape = vertices.shape();
    if shape[1] != 2 {
        return Err(PyValueError::new_err(format!(
            "The last dimension of the vertices array must have a size of 2, got shape ({}, {})",
            shape[0], shape[1]
        )));
    }

    let vertices_: Vec<(f64, f64)> = vertices
        .to_vec()?
        .chunks(2)
        .map(|row| (row[0], row[1]))
        .collect();

    let (ipix, fully_covered) =
        scalar::polygon_coverage(&vertices_, layer, &ellipsoid_, exact, flat);

    Ok((
        PyArray1::from_vec(py, ipix),
        PyArray1::from_vec(py, fully_covered),
    ))
}

#[allow(clippy::type_complexity)]
#[pyfunction]
#[pyo3(signature = (depth, center, radius, *, ellipsoid, delta_depth = 0, flat = true))]
pub(crate) fn cone_coverage<'py>(
    py: Python<'py>,
    depth: u8,
    center: (f64, f64),
    radius: f64,
    ellipsoid: EllipsoidLike,
    delta_depth: u8,
    flat: bool,
) -> PyResult<(Bound<'py, PyArray1<u64>>, Bound<'py, PyArray1<bool>>)> {
    if depth > 29 {
        return Err(PyValueError::new_err(
            "depth must be between 0 and 29, inclusive.",
        ));
    } else if depth + delta_depth > 29 {
        return Err(PyValueError::new_err(
            "delta_depth must chosen such that depth + delta_depth <= 29",
        ));
    }

    let ellipsoid_ = ellipsoid.into_ellipsoid()?;
    let layer = healpix::nested::get(depth);

    let (ipix, fully_covered) =
        scalar::cone_coverage(center, radius, layer, &ellipsoid_, delta_depth, flat);

    Ok((
        PyArray1::from_vec(py, ipix),
        PyArray1::from_vec(py, fully_covered),
    ))
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (depth, center, ellipse_geometry, position_angle, *, ellipsoid, delta_depth = 0, flat = true))]
pub(crate) fn elliptical_cone_coverage<'py>(
    py: Python<'py>,
    depth: u8,
    center: (f64, f64),
    ellipse_geometry: (f64, f64),
    position_angle: f64,
    ellipsoid: EllipsoidLike,
    delta_depth: u8,
    flat: bool,
) -> PyResult<(Bound<'py, PyArray1<u64>>, Bound<'py, PyArray1<bool>>)> {
    if depth > 29 {
        return Err(PyValueError::new_err(
            "depth must be between 0 and 29, inclusive.",
        ));
    } else if depth + delta_depth > 29 {
        return Err(PyValueError::new_err(
            "delta_depth must chosen such that depth + delta_depth <= 29",
        ));
    }

    let ellipsoid_ = ellipsoid.into_ellipsoid()?;
    let layer = healpix::nested::get(depth);

    let (ipix, fully_covered) = scalar::elliptical_cone_coverage(
        center,
        ellipse_geometry,
        position_angle,
        layer,
        &ellipsoid_,
        delta_depth,
        flat,
    );

    Ok((
        PyArray1::from_vec(py, ipix),
        PyArray1::from_vec(py, fully_covered),
    ))
}
