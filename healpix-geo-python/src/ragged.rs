use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyFunction, PyModule};

use numpy::{
    PyArray1, PyArrayDescrMethods, PyArrayDyn, PyArrayMethods, PyUntypedArray,
    PyUntypedArrayMethods, dtype,
};

/// very basic implementation of a ragged array
#[pyclass(frozen)]
#[derive(Debug)]
struct RaggedArray {
    #[pyo3(get)]
    cuts: Py<PyArray1<u64>>,
    #[pyo3(get)]
    data: Py<PyUntypedArray>,
}

#[pymethods]
impl RaggedArray {
    /// construct the ragged array from offsets and data
    #[new]
    fn create<'py>(
        py: Python<'py>,
        cuts: &Bound<'py, PyUntypedArray>,
        data: &Bound<'py, PyUntypedArray>,
    ) -> PyResult<Self> {
        let cuts_dtype = cuts.dtype();
        let cuts_ = (if cuts_dtype.is_equiv_to(&dtype::<u64>(py)) {
            let array = cuts.cast::<PyArrayDyn<u64>>()?;

            array.reshape([array.len()])
        } else {
            Err(PyValueError::new_err(format!(
                "cuts must be of dtype uint64, got {cuts_dtype}"
            )))
        })?;

        Ok(Self {
            cuts: cuts_.unbind(),
            data: data.clone().unbind(),
        })
    }

    /// apply a element-wise function
    fn apply_element_wise<'py>(
        &self,
        py: Python<'py>,
        func: &Bound<'py, PyFunction>,
    ) -> PyResult<Self> {
        let result = func.call1((self.data.bind(py),))?;
        let new_data = result.cast::<PyUntypedArray>()?;

        Ok(Self {
            cuts: self.cuts.clone_ref(py),
            data: new_data.clone().unbind(),
        })
    }

    /// convert to a awkward array
    fn as_awkward<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let awkward = PyModule::import(py, "awkward")?;
        let contents = awkward.getattr("contents")?;
        let index = awkward.getattr("index")?;

        let array_cls = awkward.getattr("Array")?;
        let list_offset_array_cls = contents.getattr("ListOffsetArray")?;
        let index64_cls = index.getattr("Index64")?;
        let numpy_array_cls = contents.getattr("NumpyArray")?;

        let offsets = self.cuts.bind(py);
        let data = self.data.bind(py);

        let layout = list_offset_array_cls.call1((
            index64_cls.call1((offsets,))?,
            numpy_array_cls.call1((data,))?,
        ))?;

        array_cls.call1((layout,))
    }

    /// convert to a ragged array
    fn as_ragged<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let ragged = PyModule::import(py, "ragged")?;
        let array_cls = ragged.getattr("array")?;

        let awkward_array = self.as_awkward(py)?;

        array_cls.call1((awkward_array,))
    }
}
