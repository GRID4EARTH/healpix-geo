use crate::ellipsoid::EllipsoidLike;
use cdshealpix as healpix;
use numpy::{PyArray1, PyArray2, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::prelude::*;

use healpix_geo_core::vectorized::ring::coordinates as vectorized;

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn healpix_to_lonlat<'py>(
    py: Python<'py>,
    depth: u8,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    ellipsoid_like: EllipsoidLike,
    nthreads: u16,
) -> PyResult<(Bound<'py, PyArrayDyn<f64>>, Bound<'py, PyArrayDyn<f64>>)> {
    let ellipsoid = ellipsoid_like.into_ellipsoid()?;
    let input_shape = ipix.shape();

    let ipix_ = ipix.readonly();

    let nside = healpix::nside(depth);

    let (lon, lat): (Vec<f64>, Vec<f64>) =
        vectorized::healpix_to_lonlat(ipix_.as_slice()?, &nside, &ellipsoid, nthreads as usize)
            .into_iter()
            .unzip();

    Ok((
        PyArray1::from_vec(py, lon).reshape(input_shape)?,
        PyArray1::from_vec(py, lat).reshape(input_shape)?,
    ))
}

#[pyfunction]
pub(crate) fn lonlat_to_healpix<'py>(
    py: Python<'py>,
    depth: u8,
    longitude: &Bound<'py, PyArrayDyn<f64>>,
    latitude: &Bound<'py, PyArrayDyn<f64>>,
    ellipsoid_like: EllipsoidLike,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    let ellipsoid = ellipsoid_like.into_ellipsoid()?;
    let input_shape = longitude.shape();

    let lon = longitude.readonly();
    let lat = latitude.readonly();
    let coords: Vec<(f64, f64)> = lon
        .as_slice()?
        .iter()
        .zip(lat.as_slice()?)
        .map(|(&lon, &lat)| (lon, lat))
        .collect();

    let nside = healpix::nside(depth);

    let ipix = vectorized::lonlat_to_healpix(&coords, &nside, &ellipsoid, nthreads as usize);

    PyArray1::from_vec(py, ipix).reshape(input_shape)
}

#[allow(clippy::type_complexity)]
#[pyfunction]
#[pyo3(signature = (depth, ipix, ellipsoid_like, step=1, nthreads=0))]
pub(crate) fn vertices<'py>(
    py: Python<'py>,
    depth: u8,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    ellipsoid_like: EllipsoidLike,
    step: usize,
    nthreads: u16,
) -> PyResult<(Bound<'py, PyArrayDyn<f64>>, Bound<'py, PyArrayDyn<f64>>)> {
    let ellipsoid = ellipsoid_like.into_ellipsoid()?;
    let input_shape: &[usize] = ipix.shape();
    let ipix_ = ipix.readonly();

    let nside = healpix::nside(depth);

    let vertices: Vec<Vec<(f64, f64)>> = vectorized::vertices(
        ipix_.as_slice()?,
        &nside,
        &ellipsoid,
        step,
        nthreads as usize,
    );

    let (lon, lat): (Vec<Vec<f64>>, Vec<Vec<f64>>) = vertices
        .into_iter()
        .map(|row: Vec<(f64, f64)>| -> (Vec<f64>, Vec<f64>) { row.into_iter().unzip() })
        .unzip();

    let output_shape: Vec<usize> = input_shape.iter().copied().chain([lon[0].len()]).collect();

    let longitude = PyArray2::from_vec2(py, &lon)?.reshape(output_shape.as_slice())?;
    let latitude = PyArray2::from_vec2(py, &lat)?.reshape(output_shape.as_slice())?;

    Ok((longitude, latitude))
}

/// Wrapper of `UnitVect3.ang_dist`
/// The given array must be of the same size as `ipix`.
#[pyfunction]
pub(crate) fn angular_distances<'py>(
    py: Python<'py>,
    depth: u8,
    from: &Bound<'py, PyArray1<u64>>,
    to: &Bound<'py, PyArray2<u64>>,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    use healpix_geo_core::vectorized::ring::distances as vectorized;
    let from_ = from.readonly();
    let to_ = to.readonly();
    let cols = to.shape()[1];

    let nside = cdshealpix::nside(depth);
    let result = vectorized::angular_distances(
        from_.as_slice()?,
        to_.as_slice()?,
        cols,
        &nside,
        nthreads as usize,
    );

    Ok(PyArray2::from_vec2(py, &result)?)
}
