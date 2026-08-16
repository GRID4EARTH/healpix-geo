use numpy::{PyArray1, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use cdshealpix::compass_point::MainWind;
use healpix_geo_core::vectorized::nested::topology as vectorized;

use crate::indexing_schemes::depth::DepthLike;

#[pyfunction]
pub(crate) fn face_neighbour_transform(
    face: u8,
    direction: &str,
) -> PyResult<Option<(u8, bool, bool, bool)>> {
    if face >= 12 {
        return Err(PyValueError::new_err(
            "face must be in the [0, 11] closed range",
        ));
    }
    let direction = match direction.to_ascii_uppercase().as_str() {
        "N" => MainWind::N,
        "NE" => MainWind::NE,
        "E" => MainWind::E,
        "SE" => MainWind::SE,
        "S" => MainWind::S,
        "SW" => MainWind::SW,
        "W" => MainWind::W,
        "NW" => MainWind::NW,
        _ => {
            return Err(PyValueError::new_err(
                "direction must be one of N, NE, E, SE, S, SW, W, or NW",
            ));
        }
    };

    Ok(
        vectorized::face_neighbour_transform(face, direction).map(|transform| {
            (
                transform.target_face,
                transform.swap_xy,
                transform.flip_x,
                transform.flip_y,
            )
        }),
    )
}

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn pix2xyf<'py>(
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

    let (face, x, y) = vectorized::pix2xyf(flattened.as_slice()?, depth, nthreads as usize);

    Ok((
        PyArray1::from_vec(py, face)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
        PyArray1::from_vec(py, x)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
        PyArray1::from_vec(py, y)
            .reshape(input_shape)?
            .to_dyn()
            .clone(),
    ))
}

#[pyfunction]
pub(crate) fn xyf2pix<'py>(
    py: Python<'py>,
    face: &Bound<'py, PyArrayDyn<u8>>,
    x: &Bound<'py, PyArrayDyn<u32>>,
    y: &Bound<'py, PyArrayDyn<u32>>,
    depth: DepthLike,
    nthreads: u16,
) -> PyResult<Bound<'py, PyArrayDyn<u64>>> {
    let input_shape = face.shape();
    let face = face.reshape([face.len()])?;
    let x = x.reshape([x.len()])?;
    let y = y.reshape([y.len()])?;
    let face = face.readonly();
    let x = x.readonly();
    let y = y.readonly();
    let depth = depth.as_depth()?;

    let nested = vectorized::xyf2pix(
        face.as_slice()?,
        x.as_slice()?,
        y.as_slice()?,
        depth,
        nthreads as usize,
    );

    Ok(PyArray1::from_vec(py, nested)
        .reshape(input_shape)?
        .to_dyn()
        .clone())
}
