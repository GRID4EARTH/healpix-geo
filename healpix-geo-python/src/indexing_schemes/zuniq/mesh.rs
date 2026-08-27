use healpix_geo::vectorized::zuniq::mesh as vectorized;
use numpy::{PyArray2, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::prelude::*;

#[pyfunction]
pub(crate) fn vertex_indices<'py>(
    py: Python<'py>,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    let input_shape = ipix.shape();
    let output_shape: Vec<usize> = input_shape.iter().copied().chain([4]).collect();

    let ipix_ = ipix.readonly();

    let vertex_ids = vectorized::vertex_indices(ipix_.as_slice()?, nthreads as usize)
        .into_iter()
        .map(|(a, b, c, d)| vec![a, b, c, d])
        .collect::<Vec<Vec<u64>>>();

    PyArray2::from_vec2(py, &vertex_ids)?.reshape(output_shape)
}
