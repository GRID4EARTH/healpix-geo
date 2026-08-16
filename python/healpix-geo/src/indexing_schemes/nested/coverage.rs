use crate::ellipsoid::EllipsoidLike;
use cdshealpix as healpix;
use numpy::{PyArray1, PyArray2, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use healpix_geo_core::scalar::nested::coverage as scalar;
use healpix_geo_core::vectorized::nested::coverage as vectorized;

#[allow(clippy::type_complexity)]
#[pyfunction]
#[pyo3(signature = (depth, bbox, *, ellipsoid, flat = true))]
pub(crate) fn zone_coverage<'py>(
    py: Python<'py>,
    depth: u8,
    bbox: (f64, f64, f64, f64),
    ellipsoid: EllipsoidLike,
    flat: bool,
) -> PyResult<(
    Bound<'py, PyArray1<u64>>,
    Bound<'py, PyArray1<u8>>,
    Bound<'py, PyArray1<bool>>,
)> {
    let ellipsoid_ = ellipsoid.into_ellipsoid()?;
    let layer = healpix::nested::get(depth);

    let (ipix, depths, fully_covered) = scalar::zone_coverage(bbox, layer, &ellipsoid_, flat);

    Ok((
        PyArray1::from_vec(py, ipix),
        PyArray1::from_vec(py, depths),
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
) -> PyResult<(
    Bound<'py, PyArray1<u64>>,
    Bound<'py, PyArray1<u8>>,
    Bound<'py, PyArray1<bool>>,
)> {
    let ellipsoid_ = ellipsoid.into_ellipsoid()?;
    let layer = healpix::nested::get(depth);

    let (ipix, depths, fully_covered) =
        scalar::box_coverage(center, size, angle, layer, &ellipsoid_, flat);

    Ok((
        PyArray1::from_vec(py, ipix),
        PyArray1::from_vec(py, depths),
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
) -> PyResult<(
    Bound<'py, PyArray1<u64>>,
    Bound<'py, PyArray1<u8>>,
    Bound<'py, PyArray1<bool>>,
)> {
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

    let (ipix, depths, fully_covered) =
        scalar::polygon_coverage(&vertices_, layer, &ellipsoid_, exact, flat);

    Ok((
        PyArray1::from_vec(py, ipix),
        PyArray1::from_vec(py, depths),
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
) -> PyResult<(
    Bound<'py, PyArray1<u64>>,
    Bound<'py, PyArray1<u8>>,
    Bound<'py, PyArray1<bool>>,
)> {
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

    let (ipix, depths, fully_covered) =
        scalar::cone_coverage(center, radius, layer, &ellipsoid_, delta_depth, flat);

    Ok((
        PyArray1::from_vec(py, ipix),
        PyArray1::from_vec(py, depths),
        PyArray1::from_vec(py, fully_covered),
    ))
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (depth, centers, radius, *, ellipsoid, delta_depth = 0, flat = true, nthreads = 0))]
pub(crate) fn cone_coverage_many<'py>(
    py: Python<'py>,
    depth: u8,
    centers: &Bound<'py, PyArray2<f64>>,
    radius: f64,
    ellipsoid: EllipsoidLike,
    delta_depth: u8,
    flat: bool,
    nthreads: u16,
) -> PyResult<(
    Bound<'py, PyArray1<u64>>,
    Bound<'py, PyArray1<u64>>,
    Bound<'py, PyArray1<u8>>,
    Bound<'py, PyArray1<bool>>,
)> {
    if depth > 29 {
        return Err(PyValueError::new_err(
            "depth must be between 0 and 29, inclusive.",
        ));
    } else if depth + delta_depth > 29 {
        return Err(PyValueError::new_err(
            "delta_depth must chosen such that depth + delta_depth <= 29",
        ));
    }

    let shape = centers.shape();
    if shape[1] != 2 {
        return Err(PyValueError::new_err(format!(
            "The last dimension of the centers array must have a size of 2, got shape ({}, {})",
            shape[0], shape[1]
        )));
    }

    let centers_: Vec<(f64, f64)> = centers
        .to_vec()?
        .chunks_exact(2)
        .map(|row| (row[0], row[1]))
        .collect();
    let ellipsoid_ = ellipsoid.into_ellipsoid()?;
    let layer = healpix::nested::get(depth);

    let result = py.detach(move || {
        let rows = vectorized::cone_coverage_many(
            &centers_,
            radius,
            layer,
            &ellipsoid_,
            delta_depth,
            flat,
            nthreads as usize,
        );

        let total_len = rows
            .iter()
            .try_fold(0usize, |total, row| total.checked_add(row.0.len()))?;
        let mut offsets = Vec::<u64>::with_capacity(rows.len() + 1);
        let mut ipix = Vec::<u64>::with_capacity(total_len);
        let mut depths = Vec::<u8>::with_capacity(total_len);
        let mut fully_covered = Vec::<bool>::with_capacity(total_len);
        offsets.push(0);

        for (row_ipix, row_depths, row_fully_covered) in rows {
            ipix.extend(row_ipix);
            depths.extend(row_depths);
            fully_covered.extend(row_fully_covered);
            offsets.push(u64::try_from(ipix.len()).ok()?);
        }

        Some((offsets, ipix, depths, fully_covered))
    });

    let (offsets, ipix, depths, fully_covered) = result
        .ok_or_else(|| PyValueError::new_err("cone coverage result is too large to represent"))?;

    Ok((
        PyArray1::from_vec(py, offsets),
        PyArray1::from_vec(py, ipix),
        PyArray1::from_vec(py, depths),
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
) -> PyResult<(
    Bound<'py, PyArray1<u64>>,
    Bound<'py, PyArray1<u8>>,
    Bound<'py, PyArray1<bool>>,
)> {
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

    let (ipix, depths, fully_covered) = scalar::elliptical_cone_coverage(
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
        PyArray1::from_vec(py, depths),
        PyArray1::from_vec(py, fully_covered),
    ))
}
