use crate::ellipsoid::EllipsoidLike;
use cdshealpix as healpix;
use numpy::{PyArray1, PyArray2, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::indexing_schemes::coverage_types::CenterLike;
use crate::ragged::RaggedArray;
use healpix_geo::scalar::nested::coverage as scalar;
use healpix_geo::vectorized::nested::coverage as vectorized;

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

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (depth, center, radius, *, ellipsoid, delta_depth = 0, flat = true, nthreads = 0))]
pub(crate) fn cone_coverage<'py>(
    py: Python<'py>,
    depth: u8,
    center: CenterLike,
    radius: f64,
    ellipsoid: EllipsoidLike,
    delta_depth: u8,
    flat: bool,
    nthreads: u16,
) -> PyResult<(RaggedArray, RaggedArray, RaggedArray)> {
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

    let center_coords = match center {
        CenterLike::Array(array) => {
            let shape = array.shape();

            if shape[shape.len() - 1] != 2 {
                Err(PyValueError::new_err(
                    "`center` must have a last dimension of size 2",
                ))
            } else {
                let reshaped = array.reshape([shape[..shape.len() - 1].iter().product(), 2])?;
                let readonly = reshaped.readonly();

                Ok(readonly
                    .as_slice()?
                    .chunks_exact(2)
                    .map(|chunk| {
                        if let [lon, lat] = chunk {
                            (*lon, *lat)
                        } else {
                            unreachable!()
                        }
                    })
                    .collect::<Vec<(f64, f64)>>())
            }
        }
        CenterLike::Scalar((lon, lat)) => Ok(vec![(lon, lat)]),
    }?;

    let (offsets, ipix, depths, fully_covered) = py.detach(move || {
        let result = vectorized::cone_coverage(
            &center_coords,
            radius,
            layer,
            &ellipsoid_,
            delta_depth,
            flat,
            nthreads as usize,
        );

        let n_elements = result.iter().fold(0, |total, entry| total + entry.0.len());

        let mut offsets = Vec::<u64>::with_capacity(result.len() + 1);
        let mut ipix = Vec::<u64>::with_capacity(n_elements);
        let mut depths = Vec::<u8>::with_capacity(n_elements);
        let mut fully_covered = Vec::<bool>::with_capacity(n_elements);

        offsets.push(0);
        for (index, (entry_ipix, entry_depths, entry_fully_covered)) in
            result.into_iter().enumerate()
        {
            offsets.push(offsets[index] + entry_ipix.len() as u64);

            ipix.extend(entry_ipix);
            depths.extend(entry_depths);
            fully_covered.extend(entry_fully_covered);
        }

        (offsets, ipix, depths, fully_covered)
    });

    let offset_array = PyArray1::from_vec(py, offsets);

    Ok((
        RaggedArray::new(&offset_array, PyArray1::from_vec(py, ipix).as_untyped())?,
        RaggedArray::new(&offset_array, PyArray1::from_vec(py, depths).as_untyped())?,
        RaggedArray::new(
            &offset_array,
            PyArray1::from_vec(py, fully_covered).as_untyped(),
        )?,
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
