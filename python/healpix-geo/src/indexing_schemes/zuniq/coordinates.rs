use crate::ellipsoid::EllipsoidLike;
use cdshealpix as healpix;

use numpy::{PyArray1, PyArray2, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;

use crate::indexing_schemes::depth::DepthLike;
use crate::traits::Unzip3;
use healpix_geo_core::vectorized::zuniq::coordinates as vectorized;

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn healpix_to_lonlat<'py>(
    py: Python<'py>,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    ellipsoid_like: EllipsoidLike,
    nthreads: u16,
) -> PyResult<(Bound<'py, PyArrayDyn<f64>>, Bound<'py, PyArrayDyn<f64>>)> {
    let ellipsoid = ellipsoid_like.into_ellipsoid()?;
    let input_shape = ipix.shape();

    let ipix_ = ipix.readonly();

    let (lon, lat): (Vec<f64>, Vec<f64>) =
        vectorized::healpix_to_lonlat(ipix_.as_slice()?, &ellipsoid, nthreads as usize)
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
    depth: DepthLike,
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

    let ipix = match depth {
        DepthLike::Constant(d) => {
            let layer = healpix::nested::get(d);

            vectorized::lonlat_to_healpix(&coords, layer, &ellipsoid, nthreads as usize)
        }
        DepthLike::Array(_depths) => {
            return Err(PyNotImplementedError::new_err("not implemented yet!"));

            // never reachable
            // let bound = depths.bind(py);
            // let depths_ = unsafe { bound.as_array() };
            // maybe_parallelize!(
            //     nthreads,
            //     Zip::from(&longitude)
            //         .and(&latitude)
            //         .and(&depths_)
            //         .and(&mut ipix),
            //     |lon, lat, &d, p| {
            //         let layer = cdshealpix::nested::get(d);

            //         let hash = lonlat_to_healpix_internal(
            //             lon,
            //             lat,
            //             layer,
            //             &ellipsoid_,
            //             &coefficients,
            //             &is_spherical,
            //         );

            //         *p = healpix::nested::to_zuniq(d, hash);
            //     }
            // );
        }
    };

    PyArray1::from_vec(py, ipix).reshape(input_shape)
}

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn healpix_to_cartesian<'py>(
    py: Python<'py>,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    ellipsoid_like: EllipsoidLike,
    nthreads: u16,
) -> PyResult<(
    Bound<'py, PyArrayDyn<f64>>,
    Bound<'py, PyArrayDyn<f64>>,
    Bound<'py, PyArrayDyn<f64>>,
)> {
    let ellipsoid = ellipsoid_like.into_ellipsoid()?;
    let input_shape = ipix.shape();

    let ipix_ = ipix.readonly();

    let (x, y, z): (Vec<f64>, Vec<f64>, Vec<f64>) =
        vectorized::healpix_to_cartesian(ipix_.as_slice()?, &ellipsoid, nthreads as usize).unzip3();

    Ok((
        PyArray1::from_vec(py, x).reshape(input_shape)?,
        PyArray1::from_vec(py, y).reshape(input_shape)?,
        PyArray1::from_vec(py, z).reshape(input_shape)?,
    ))
}

#[pyfunction]
pub(crate) fn cartesian_to_healpix<'py>(
    py: Python<'py>,
    depth: u8,
    x: &Bound<'py, PyArrayDyn<f64>>,
    y: &Bound<'py, PyArrayDyn<f64>>,
    z: &Bound<'py, PyArrayDyn<f64>>,
    ellipsoid_like: EllipsoidLike,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    let ellipsoid = ellipsoid_like.into_ellipsoid()?;
    let input_shape = x.shape();

    let x = x.readonly();
    let y = y.readonly();
    let z = z.readonly();
    let coords: Vec<(f64, f64, f64)> = x
        .as_slice()?
        .iter()
        .zip(y.as_slice()?)
        .zip(z.as_slice()?)
        .map(|((&x, &y), &z)| (x, y, z))
        .collect();

    let layer = healpix::nested::get(depth);

    let ipix = vectorized::cartesian_to_healpix(&coords, layer, &ellipsoid, nthreads as usize);

    PyArray1::from_vec(py, ipix).reshape(input_shape)
}

#[allow(clippy::type_complexity)]
#[pyfunction]
#[pyo3(signature = (ipix, ellipsoid_like, step=1, nthreads=0))]
pub(crate) fn vertices<'py>(
    py: Python<'py>,
    ipix: &Bound<'py, PyArrayDyn<u64>>,
    ellipsoid_like: EllipsoidLike,
    step: usize,
    nthreads: u16,
) -> PyResult<(Bound<'py, PyArrayDyn<f64>>, Bound<'py, PyArrayDyn<f64>>)> {
    let ellipsoid = ellipsoid_like.into_ellipsoid()?;
    let input_shape = ipix.shape();

    let ipix_ = ipix.readonly();

    let vertices: Vec<Vec<(f64, f64)>> =
        vectorized::vertices(ipix_.as_slice()?, &ellipsoid, step, nthreads as usize);

    let (lon, lat): (Vec<Vec<f64>>, Vec<Vec<f64>>) = vertices
        .into_iter()
        .map(|row: Vec<(f64, f64)>| -> (Vec<f64>, Vec<f64>) { row.into_iter().unzip() })
        .unzip();

    let output_shape: Vec<usize> = input_shape.iter().copied().chain([lon[0].len()]).collect();

    let longitude = PyArray2::from_vec2(py, &lon)?.reshape(output_shape.as_slice())?;
    let latitude = PyArray2::from_vec2(py, &lat)?.reshape(output_shape.as_slice())?;

    Ok((longitude, latitude))
}

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn bilinear_interpolation<'py>(
    py: Python<'py>,
    depth: u8,
    longitude: &Bound<'py, PyArrayDyn<f64>>,
    latitude: &Bound<'py, PyArrayDyn<f64>>,
    ellipsoid_like: EllipsoidLike,
    nthreads: u16,
) -> PyResult<(Bound<'py, PyArrayDyn<u64>>, Bound<'py, PyArrayDyn<f64>>)> {
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

    let layer = cdshealpix::nested::get(depth);

    let (ipix, weights): (Vec<Vec<u64>>, Vec<Vec<f64>>) =
        vectorized::bilinear_interpolation(&coords, layer, &ellipsoid, nthreads as usize)
            .into_iter()
            .map(|row: Vec<(u64, f64)>| -> (Vec<u64>, Vec<f64>) { row.into_iter().unzip() })
            .unzip();

    let output_shape: Vec<usize> = input_shape.iter().copied().chain([ipix[0].len()]).collect();

    let ipix_ = PyArray2::from_vec2(py, &ipix)?.reshape(output_shape.as_slice())?;
    let weights_ = PyArray2::from_vec2(py, &weights)?.reshape(output_shape.as_slice())?;

    Ok((ipix_, weights_))
}
