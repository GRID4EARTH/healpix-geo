use numpy::{PyArray1, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::prelude::*;

use healpix_geo::vectorized::zuniq::conversion as vectorized;

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn to_nested<'py>(
    py: Python<'py>,
    zuniq: &Bound<'py, PyArrayDyn<u64>>,
    nthreads: u16,
) -> PyResult<(Bound<'py, PyArrayDyn<u64>>, Bound<'py, PyArrayDyn<u8>>)> {
    let input_shape = zuniq.shape();

    let flattened = zuniq.reshape([zuniq.len()])?;
    let flattened_ = flattened.readonly();

    let (nested, depths) = vectorized::to_nested(flattened_.as_slice()?, nthreads as usize);

    Ok((
        PyArray1::from_vec(py, nested)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
        PyArray1::from_vec(py, depths)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
    ))
}

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn to_ring<'py>(
    py: Python<'py>,
    zuniq: &Bound<'py, PyArrayDyn<u64>>,
    nthreads: u16,
) -> PyResult<(Bound<'py, PyArrayDyn<u64>>, Bound<'py, PyArrayDyn<u8>>)> {
    let input_shape = zuniq.shape();

    let flattened = zuniq.reshape([zuniq.len()])?;
    let flattened_ = flattened.readonly();

    let (ring, depths) = vectorized::to_ring(flattened_.as_slice()?, nthreads as usize);

    Ok((
        PyArray1::from_vec(py, ring)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
        PyArray1::from_vec(py, depths)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
    ))
}
