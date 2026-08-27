use numpy::{PyArray1, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::indexing_schemes::wind_rose::WindRose;
use healpix_geo_core::scalar::nested::topology as scalar;
use healpix_geo_core::vectorized::nested::topology as vectorized;

use crate::indexing_schemes::depth::DepthLike;

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn base_cell_relationship<'py>(
    py: Python<'py>,
    face: u8,
    direction: WindRose,
) -> PyResult<Option<(Bound<'py, PyArray1<i32>>, Bound<'py, PyArray1<i32>>)>> {
    if !(0..=11).contains(&face) {
        Err(PyValueError::new_err(
            "face must be in the [0, 11] closed range",
        ))
    } else {
        match scalar::base_cell_relationship(face, direction.into_mainwind()) {
            None => Ok(None),
            Some(((x1, y1), (x2, y2))) => {
                let array1 = PyArray1::from_vec(py, vec![x1, y1]);
                let array2 = PyArray1::from_vec(py, vec![x2, y2]);

                Ok(Some((array1, array2)))
            }
        }
    }
}

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn healpix_to_base_cell_coordinates<'py>(
    py: Python<'py>,
    nested: &Bound<'py, PyArrayDyn<u64>>,
    depth: DepthLike,
    nthreads: u16,
) -> PyResult<(
    Bound<'py, PyArrayDyn<u8>>,
    Bound<'py, PyArrayDyn<u32>>,
    Bound<'py, PyArrayDyn<u32>>,
)> {
    let input_shape = nested.shape();
    let flattened = nested.reshape([nested.len()])?;
    let flattened = flattened.readonly();
    let depth = depth.as_depth()?;

    let (face, x, y) = vectorized::healpix_to_base_cell_coordinates(
        flattened.as_slice()?,
        depth,
        nthreads as usize,
    );

    Ok((
        PyArray1::from_vec(py, face)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
        PyArray1::from_vec(py, x)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
        PyArray1::from_vec(py, y)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
    ))
}

#[pyfunction]
pub(crate) fn base_cell_coordinates_to_healpix<'py>(
    py: Python<'py>,
    face: &Bound<'py, PyArrayDyn<u8>>,
    i: &Bound<'py, PyArrayDyn<u32>>,
    j: &Bound<'py, PyArrayDyn<u32>>,
    depth: DepthLike,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    let input_shape = face.shape();

    let face = face.reshape([face.len()])?;
    let i = i.reshape([i.len()])?;
    let j = j.reshape([j.len()])?;

    let face = face.readonly();
    let x = x.readonly();
    let y = y.readonly();

    let depth = depth.as_depth()?;

    let nested = vectorized::base_cell_coordinates_to_healpix(
        face.as_slice()?,
        x.as_slice()?,
        y.as_slice()?,
        depth,
        nthreads as usize,
    );

    Ok(PyArray1::from_vec(py, nested)
        .reshape(input_shape)?
        .to_dyn()
        .clone())
}
