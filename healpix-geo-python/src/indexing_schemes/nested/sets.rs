use moc::moc::range::RangeMOC;
use numpy::{PyArray1, PyArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Extract the internal boundary from a list of cells
///
/// Parameters
/// ----------
/// depth : int
///     The cell depth
/// cell_ids : numpy.ndarray
///     The cells describing a connected area
///
/// Returns
/// -------
/// internal_boundary : numpy.ndarray
///     The boundary cells
#[pyfunction]
pub(crate) fn internal_boundary<'py>(
    py: Python<'py>,
    depth: u8,
    ipix: &Bound<'py, PyArray1<u64>>,
) -> PyResult<Bound<'py, PyArray1<u64>>> {
    if depth > 29 {
        return Err(PyValueError::new_err(format!(
            "depth must be between 0 and 29, inclusive (got {})",
            depth
        )));
    }

    let ipix_ = ipix.readonly();
    let moc = RangeMOC::from_fixed_depth_cells(depth, ipix_.as_slice()?.iter().copied(), None);

    let border = moc.internal_border();

    Ok(PyArray1::from_vec(
        py,
        border.flatten_to_fixed_depth_cells().collect::<Vec<u64>>(),
    ))
}
