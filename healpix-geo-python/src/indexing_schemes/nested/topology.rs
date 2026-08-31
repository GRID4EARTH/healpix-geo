use numpy::{PyArray1, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::prelude::*;

use healpix_geo::vectorized::nested::topology as vectorized;

use crate::indexing_schemes::depth::DepthLike;
use crate::traits::Unzip3;

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

    let (base_cell, i, j) = vectorized::healpix_to_base_cell_coordinates(
        flattened.as_slice()?,
        depth,
        nthreads as usize,
    )
    .unzip3();

    Ok((
        PyArray1::from_vec(py, base_cell)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
        PyArray1::from_vec(py, i)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
        PyArray1::from_vec(py, j)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
    ))
}

#[pyfunction]
pub(crate) fn base_cell_coordinates_to_healpix<'py>(
    py: Python<'py>,
    base_cell: &Bound<'py, PyArrayDyn<u8>>,
    i: &Bound<'py, PyArrayDyn<u32>>,
    j: &Bound<'py, PyArrayDyn<u32>>,
    depth: DepthLike,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    let input_shape = base_cell.shape();

    let base_cell_ = base_cell.reshape([base_cell.len()])?;
    let i_ = i.reshape([i.len()])?;
    let j_ = j.reshape([j.len()])?;

    let base_cell = base_cell_.readonly();
    let i = i_.readonly();
    let j = j_.readonly();

    let coords: Vec<_> = base_cell
        .as_slice()?
        .iter()
        .zip(i.as_slice()?)
        .zip(j.as_slice()?)
        .map(|((base_cell, i), j)| (*base_cell, *i, *j))
        .collect();

    let depth = depth.as_depth()?;

    let nested = vectorized::base_cell_coordinates_to_healpix(&coords, depth, nthreads as usize);

    Ok(PyArray1::from_vec(py, nested)
        .reshape(input_shape)?
        .to_dyn()
        .clone())
}
