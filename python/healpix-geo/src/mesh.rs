use numpy::{PyArray1, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::prelude::*;

use crate::ellipsoid::EllipsoidLike;
use healpix_geo_core::vectorized::mesh as vectorized;

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn vertex_to_lonlat<'py>(
    py: Python<'py>,
    depth: u8,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    ellipsoid_like: EllipsoidLike,
    nthreads: u16,
) -> PyResult<(Bound<'py, PyArrayDyn<f64>>, Bound<'py, PyArrayDyn<f64>>)> {
    let ellipsoid = ellipsoid_like.into_ellipsoid()?;
    let input_shape = ipix.shape();

    let ipix_ = ipix.readonly();

    let (lon, lat): (Vec<f64>, Vec<f64>) =
        vectorized::vertex_to_lonlat(depth, ipix_.as_slice()?, &ellipsoid, nthreads as usize)
            .into_iter()
            .unzip();

    Ok((
        PyArray1::from_vec(py, lon).reshape(input_shape)?,
        PyArray1::from_vec(py, lat).reshape(input_shape)?,
    ))
}
