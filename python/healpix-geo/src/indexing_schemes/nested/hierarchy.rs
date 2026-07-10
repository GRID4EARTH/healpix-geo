use cdshealpix as healpix;
use numpy::{PyArray1, PyArray2, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::prelude::*;

use healpix_geo_core::vectorized::nested::hierarchy as vectorized;

/// Wrapper of `kth_neighbours`
#[pyfunction]
pub(crate) fn kth_neighbours<'py>(
    py: Python<'py>,
    depth: u8,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    ring: u32,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<i64>>> {
    let input_shape = ipix.shape();
    let ipix_ = ipix.readonly();

    let layer = healpix::nested::get(depth);
    let result = vectorized::kth_neighbours(ipix_.as_slice()?, layer, &ring, nthreads as usize);

    let n_neighbours = 8 * ring as usize;
    let output_shape: Vec<usize> = input_shape.iter().copied().chain([n_neighbours]).collect();

    PyArray2::from_vec2(py, &result)?.reshape(output_shape.as_slice())
}

/// Wrapper of `kth_neighbourhood`
#[pyfunction]
pub(crate) fn kth_neighbourhood<'py>(
    py: Python<'py>,
    depth: u8,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    ring: u32,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<i64>>> {
    let input_shape = ipix.shape();
    let ipix_ = ipix.readonly();

    let layer = healpix::nested::get(depth);
    let result = vectorized::kth_neighbourhood(ipix_.as_slice()?, layer, &ring, nthreads as usize);

    let n_neighbours = usize::pow(2 * ring as usize + 1, 2);
    let output_shape: Vec<usize> = input_shape.iter().copied().chain([n_neighbours]).collect();

    PyArray2::from_vec2(py, &result)?.reshape(output_shape.as_slice())
}

#[pyfunction]
pub(crate) fn zoom_to<'py>(
    py: Python<'py>,
    depth: u8,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    new_depth: u8,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    use std::cmp::Ordering;

    let input_shape = ipix.shape();

    let ipix_ = ipix.readonly();
    let delta_depth = (depth as i8 - new_depth as i8).unsigned_abs();

    let result = match depth.cmp(&new_depth) {
        Ordering::Equal => ipix.to_dyn().clone(),
        Ordering::Less => {
            let result = vectorized::children(ipix_.as_slice()?, delta_depth, nthreads as usize);

            let output_shape: Vec<usize> = if ipix.len() == 0 {
                input_shape.to_vec()
            } else {
                input_shape
                    .iter()
                    .copied()
                    .chain([result[0].len()])
                    .collect()
            };

            PyArray2::from_vec2(py, &result)?.reshape(output_shape.as_slice())?
        }
        Ordering::Greater => {
            let result = vectorized::parents(ipix_.as_slice()?, delta_depth, nthreads as usize);

            PyArray1::from_vec(py, result).to_dyn().clone()
        }
    };

    Ok(result)
}

#[pyfunction]
pub(crate) fn siblings<'py>(
    py: Python<'py>,
    depth: u8,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    let ipix_ = ipix.readonly();
    let input_shape = ipix.shape();
    let layer = healpix::nested::get(depth);

    let siblings = vectorized::siblings(ipix_.as_slice()?, layer, nthreads as usize);

    let output_shape: Vec<usize> = if ipix.len() == 0 {
        input_shape.to_vec()
    } else {
        input_shape
            .iter()
            .copied()
            .chain([siblings[0].len()])
            .collect()
    };

    PyArray2::from_vec2(py, &siblings)?.reshape(output_shape.as_slice())
}
