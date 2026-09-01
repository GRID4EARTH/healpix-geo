use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyFunction, PyModule, PyString, PyTuple};

use numpy::{
    PyArray1, PyArrayDescrMethods, PyArrayMethods, PyUntypedArray, PyUntypedArrayMethods, dtype,
};

/// very basic implementation of a ragged array
#[pyclass(frozen)]
#[derive(Debug)]
pub(crate) struct RaggedArray {
    #[pyo3(get)]
    offsets: Py<PyArray1<u64>>,
    #[pyo3(get)]
    data: Py<PyUntypedArray>,

    shape: [usize; 2],
}

#[pymethods]
impl RaggedArray {
    /// construct the ragged array from offsets and data
    #[new]
    fn create<'py>(
        py: Python<'py>,
        offsets: &Bound<'py, PyUntypedArray>,
        data: &Bound<'py, PyUntypedArray>,
    ) -> PyResult<Self> {
        let numpy = PyModule::import(py, "numpy")?;
        let isdtype = numpy.getattr("isdtype")?;
        let integer_category = PyString::new(py, "integral");

        let py_dtype = offsets.getattr("dtype")?;
        let rs_dtype = offsets.dtype();

        let shape = offsets.shape();
        if shape.len() != 1 {
            Err(PyValueError::new_err(format!(
                "offsets must be 1-dimensional, got shape {:?}",
                shape
            )))?;
        }

        let offsets_ = (if isdtype
            .call1((py_dtype, integer_category))?
            .extract::<bool>()?
        {
            if !rs_dtype.is_equiv_to(&dtype::<u64>(py)) {
                let array = numpy
                    .getattr("astype")?
                    .call1((offsets, numpy.getattr("uint64")?))?;

                Ok(array.cast::<PyArray1<u64>>()?.clone())
            } else {
                Ok(offsets.cast::<PyArray1<u64>>()?.clone())
            }
        } else {
            Err(PyValueError::new_err(format!(
                "offsets must be of integer dtype, got {rs_dtype}"
            )))
        })?;

        let readonly_offsets = offsets_.readonly();

        let array_shape = [
            offsets_.len() - 1,
            readonly_offsets
                .as_slice()?
                .windows(2)
                .map(|window| window[1] - window[0])
                .max()
                .unwrap_or(0) as usize,
        ];

        Ok(Self {
            offsets: offsets_.unbind(),
            data: data.clone().unbind(),
            shape: array_shape,
        })
    }

    #[getter]
    fn shape<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.shape)
    }

    #[getter]
    fn dtype<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.data.bind(py).getattr("dtype")
    }

    #[getter]
    fn ndim(&self) -> usize {
        self.shape.len()
    }

    /// apply a element-wise function
    fn apply_elementwise<'py>(
        &self,
        py: Python<'py>,
        func: &Bound<'py, PyFunction>,
    ) -> PyResult<Self> {
        let result = func.call1((self.data.bind(py),))?;
        let new_data = result.cast::<PyUntypedArray>()?;

        Ok(Self {
            offsets: self.offsets.clone_ref(py),
            data: new_data.clone().unbind(),
            shape: self.shape,
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

        let offsets = self.offsets.bind(py);
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
