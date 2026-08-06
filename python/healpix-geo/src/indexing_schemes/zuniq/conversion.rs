use numpy::{PyArray1, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::prelude::*;

use crate::indexing_schemes::depth::DepthLike;
use healpix_geo_core::vectorized::zuniq::conversion as vectorized;

#[pyfunction]
pub(crate) fn from_nested<'py>(
    py: Python<'py>,
    nested: &Bound<'py, PyArrayDyn<u64>>,
    depth: DepthLike,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    let input_shape = nested.shape();
    let depth_ = depth.as_depth()?;

    let flattened = nested.reshape([nested.len()])?;
    let flattened_ = flattened.readonly();
    let zuniq = vectorized::from_nested(flattened_.as_slice()?, depth_, nthreads as usize);

    Ok(PyArray1::from_vec(py, zuniq)
        .reshape(input_shape)?
        .to_dyn()
        .clone())
}

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

#[pyfunction]
pub(crate) fn from_ring<'py>(
    py: Python<'py>,
    ring: &Bound<'py, PyArrayDyn<u64>>,
    depth: DepthLike,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    let input_shape = ring.shape();
    let depth_ = depth.as_depth()?;

    let flattened = ring.reshape([ring.len()])?;
    let flattened_ = flattened.readonly();
    let zuniq = vectorized::from_ring(flattened_.as_slice()?, depth_, nthreads as usize);

    Ok(PyArray1::from_vec(py, zuniq)
        .reshape(input_shape)?
        .to_dyn()
        .clone())
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
