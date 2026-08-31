use crate::indexing_schemes::wind_rose::WindRose;
use pyo3::prelude::*;

use healpix_geo::topology as implementation;

use numpy::PyArray1;
use pyo3::exceptions::PyValueError;

#[allow(clippy::type_complexity)]
#[pyfunction]
pub(crate) fn base_cell_relationship<'py>(
    py: Python<'py>,
    base_cell: u8,
    direction: WindRose,
) -> PyResult<(
    Option<u8>,
    Bound<'py, PyArray1<i8>>,
    Bound<'py, PyArray1<i8>>,
)> {
    if !(0..=11).contains(&base_cell) {
        Err(PyValueError::new_err(
            "base_cell must be in the [0, 11] closed range",
        ))
    } else {
        match implementation::base_cell_relationship(base_cell, direction.into_mainwind()) {
            None => Ok((
                None,
                PyArray1::<i8>::zeros(py, [2], false),
                PyArray1::<i8>::zeros(py, [2], false),
            )),
            Some((neighbour, (x1, y1), (x2, y2))) => {
                let array1 = PyArray1::from_vec(py, vec![x1, y1]);
                let array2 = PyArray1::from_vec(py, vec![x2, y2]);

                Ok((Some(neighbour), array1, array2))
            }
        }
    }
}
