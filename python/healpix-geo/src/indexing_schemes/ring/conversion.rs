use numpy::{PyArray1, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::prelude::*;

use healpix_geo_core::vectorized::ring::conversion as vectorized;

use crate::indexing_schemes::depth::DepthLike;

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn to_zuniq<'py>(
    py: Python<'py>,
    ring: &Bound<'py, PyArrayDyn<u64>>,
    depth: DepthLike,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    let input_shape = ring.shape();

    let flattened = ring.reshape([ring.len()])?;
    let flattened_ = flattened.readonly();

    let depth_ = depth.as_depth()?;
    let zuniq = vectorized::to_zuniq(flattened_.as_slice()?, depth_, nthreads as usize);

    Ok(PyArray1::from_vec(py, zuniq)
        .reshape(input_shape)?
        .to_dyn()
        .clone())
}

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn to_nested<'py>(
    py: Python<'py>,
    ring: &Bound<'py, PyArrayDyn<u64>>,
    depth: DepthLike,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    let input_shape = ring.shape();

    let flattened = ring.reshape([ring.len()])?;
    let flattened_ = flattened.readonly();

    let depth_ = depth.as_depth()?;
    let nested = vectorized::to_nested(flattened_.as_slice()?, depth_, nthreads as usize);

    Ok(PyArray1::from_vec(py, nested)
        .reshape(input_shape)?
        .to_dyn()
        .clone())
}
