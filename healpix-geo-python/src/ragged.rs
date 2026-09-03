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

fn copy_into_rectangular<T: Copy>(
    offsets: &[u64],
    data_in: &[T],
    data_out: &mut [T],
    shape: [usize; 2],
) {
    for (row, (in_start, in_stop)) in offsets[..offsets.len() - 1]
        .iter()
        .zip(offsets[1..].iter())
        .enumerate()
    {
        let in_start: usize = *in_start as usize;
        let in_stop: usize = *in_stop as usize;

        let row_size: isize = (in_stop as isize) - (in_start as isize);
        if row_size <= 0 {
            continue;
        }
        let row_size = row_size as usize;
        let out_start = row * shape[0];
        let out_stop = out_start + row_size;

        data_out[out_start..out_stop].copy_from_slice(&data_in[in_start..in_stop]);
    }
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

    /// convert to a rectangular numpy array
    fn as_numpy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyUntypedArray>> {
        let bound_data = self.data.bind(py);
        let offsets_ = self.offsets.bind(py);
        if offsets_.len() == 2 {
            return Ok(bound_data.into());
        }

        // steps:
        // 1. create an array like self.data, pre-filled with a fill value
        // 2. convert the array to readwrite, then to a mutable slice
        // 3. call copy_into_rectangular
        // 4. return

        let offsets = offsets_.readonly();

        let data_dtype = bound_data.dtype();
        let buffer_size = self.shape[0] * self.shape[1];
        if data_dtype.is_equiv_to(&dtype::<bool>(py))
            || data_dtype.is_equiv_to(&dtype::<i8>(py))
            || data_dtype.is_equiv_to(&dtype::<u8>(py))
        {
            let mut buffer = vec![-1i8; buffer_size];

            let cast = bound_data.cast::<PyArray1<i8>>()?;
            let source = cast.readonly();

            copy_into_rectangular(
                offsets.as_slice()?,
                source.as_slice()?,
                &mut buffer,
                self.shape,
            );

            Ok(PyArray1::from_vec(py, buffer)
                .reshape(self.shape)?
                .as_untyped()
                .clone())
        } else if data_dtype.is_equiv_to(&dtype::<i16>(py))
            || data_dtype.is_equiv_to(&dtype::<u16>(py))
        {
            let mut buffer = vec![-1i16; buffer_size];

            let cast = bound_data.cast::<PyArray1<i16>>()?;
            let source = cast.readonly();

            copy_into_rectangular(
                offsets.as_slice()?,
                source.as_slice()?,
                &mut buffer,
                self.shape,
            );

            Ok(PyArray1::from_vec(py, buffer)
                .reshape(self.shape)?
                .as_untyped()
                .clone())
        } else if data_dtype.is_equiv_to(&dtype::<i32>(py))
            || data_dtype.is_equiv_to(&dtype::<u32>(py))
        {
            let mut buffer = vec![-1i32; buffer_size];

            let cast = bound_data.cast::<PyArray1<i32>>()?;
            let source = cast.readonly();

            copy_into_rectangular(
                offsets.as_slice()?,
                source.as_slice()?,
                &mut buffer,
                self.shape,
            );

            Ok(PyArray1::from_vec(py, buffer)
                .reshape(self.shape)?
                .as_untyped()
                .clone())
        } else if data_dtype.is_equiv_to(&dtype::<i64>(py))
            || data_dtype.is_equiv_to(&dtype::<u64>(py))
        {
            let mut buffer = vec![-1i64; buffer_size];

            let cast = bound_data.cast::<PyArray1<i64>>()?;
            let source = cast.readonly();

            copy_into_rectangular(
                offsets.as_slice()?,
                source.as_slice()?,
                &mut buffer,
                self.shape,
            );

            Ok(PyArray1::from_vec(py, buffer)
                .reshape(self.shape)?
                .as_untyped()
                .clone())
        } else {
            Err(PyValueError::new_err(
                "unsupported data dtype: only boolean and integer dtypes are supported right now",
            ))
        }
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
