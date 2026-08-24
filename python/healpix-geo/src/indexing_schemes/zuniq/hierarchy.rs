use numpy::{PyArray2, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::prelude::*;

use healpix_geo_core::vectorized::zuniq::hierarchy as vectorized;

use crate::indexing_schemes::connectivity::Connectivity;

/// Return immediate neighbours in fixed directional positions.
#[pyfunction]
pub(crate) fn neighbours<'py>(
    py: Python<'py>,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    connectivity: Connectivity,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<i64>>> {
    let input_shape = ipix.shape();
    let ipix_ = ipix.readonly();
    let connectivity_ = connectivity.into_connectivity();

    let result = vectorized::neighbours(ipix_.as_slice()?, &connectivity_, nthreads as usize);

    let output_shape: Vec<usize> = input_shape
        .iter()
        .copied()
        .chain([connectivity_.size()])
        .collect();

    PyArray2::from_vec2(py, &result)?.reshape(output_shape.as_slice())
}

/// Wrapper of `kth_neighbourhood`
#[pyfunction]
pub(crate) fn kth_neighbours<'py>(
    py: Python<'py>,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    ring: u32,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<i64>>> {
    let ipix_ = ipix.readonly();
    let input_shape = ipix.shape();

    let result = vectorized::kth_neighbours(ipix_.as_slice()?, &ring, nthreads as usize);

    let output_shape: Vec<usize> = if ipix.len() == 0 {
        input_shape.to_vec()
    } else {
        input_shape
            .iter()
            .copied()
            .chain([result[0].len()])
            .collect()
    };

    PyArray2::from_vec2(py, &result)?.reshape(output_shape.as_slice())
}

/// Wrapper of `kth_neighbourhood`
#[pyfunction]
pub(crate) fn kth_neighbourhood<'py>(
    py: Python<'py>,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    ring: u32,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<i64>>> {
    let ipix_ = ipix.readonly();
    let input_shape = ipix.shape();

    let result = vectorized::kth_neighbourhood(ipix_.as_slice()?, &ring, nthreads as usize);

    let output_shape: Vec<usize> = if ipix.len() == 0 {
        input_shape.to_vec()
    } else {
        input_shape
            .iter()
            .copied()
            .chain([result[0].len()])
            .collect()
    };

    PyArray2::from_vec2(py, &result)?.reshape(output_shape.as_slice())
}
