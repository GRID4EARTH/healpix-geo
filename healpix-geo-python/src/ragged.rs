use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyFunction, PyModule, PyString, PyTuple};

use numpy::{
    PyArray1, PyArray2, PyArrayDescrMethods, PyArrayMethods, PyUntypedArray, PyUntypedArrayMethods,
    dtype,
};

/// very basic implementation of a ragged array
///
/// Attributes
/// ----------
/// offsets : numpy.ndarray of numpy.uint64
///     The offsets of each row, such that ``zip(offsets[:-1], offsets[1:])`` contains the start and stop
///     values that can be used to index ``data``.
/// data : numpy.ndarray
///     The data of the array.
#[pyclass(frozen)]
#[derive(Debug)]
pub(crate) struct RaggedArray {
    #[pyo3(get)]
    offsets: Py<PyArray1<u64>>,
    #[pyo3(get)]
    data: Py<PyUntypedArray>,

    shape: [usize; 2],
}

macro_rules! generate_copy_for_all_dtypes {
    ($offsets:expr, $data_in:expr, $shape:expr; $($T:ty),+ $(,)?) => {
        {
            let py = $offsets.py();

            let buffer_size = $shape[0] * $shape[1];
            let mut mask_buffer = vec![false; buffer_size];

            match $data_in.dtype() {
                $(
                    data_dtype if data_dtype.is_equiv_to(&dtype::<$T>(py)) => {
                        let cast = $data_in.cast::<PyArray1<$T>>()?;

                        let mut buffer: Vec<$T> = vec![Default::default(); buffer_size];

                        copy_into_rectangular(
                            $offsets.readonly().as_slice()?,
                            cast.readonly().as_slice()?,
                            &mut buffer,
                            &mut mask_buffer,
                            $shape,
                        );

                        create_masked_array(
                            PyArray1::from_vec(py, buffer).reshape($shape)?.as_untyped(),
                            &PyArray1::from_vec(py, mask_buffer).reshape($shape)?,
                        )
                    }
                )+
                _ => Err(PyTypeError::new_err(format!("Unsupported data dtype: {}", $data_in.dtype()))),
            }
        }
    }
}

fn copy_into_rectangular<T: Copy>(
    offsets: &[u64],
    data_in: &[T],
    data_out: &mut [T],
    mask_out: &mut [bool],
    shape: [usize; 2],
) {
    for (row, (in_start, in_stop)) in offsets[..offsets.len() - 1]
        .iter()
        .zip(offsets[1..].iter())
        .enumerate()
    {
        let in_start: usize = *in_start as usize;
        let in_stop: usize = *in_stop as usize;

        if in_stop < in_start {
            continue;
        }

        let row_elements = in_stop - in_start;

        let out_start = row * shape[0];
        let out_stop = out_start + row_elements;

        let row_stop = out_start + shape[1];

        data_out[out_start..out_stop].copy_from_slice(&data_in[in_start..in_stop]);

        if out_stop < row_stop {
            mask_out[out_stop..row_stop].fill(true);
        }
    }
}

fn create_masked_array<'py>(
    array: &Bound<'py, PyUntypedArray>,
    mask: &Bound<'py, PyArray2<bool>>,
) -> PyResult<Bound<'py, PyAny>> {
    let py = array.py();

    let marray = PyModule::import(py, "marray")?;
    let numpy = PyModule::import(py, "numpy")?;

    let xp = marray.getattr("masked_namespace")?.call1((&numpy,))?;

    let kwargs = PyDict::new(py);
    kwargs.set_item("mask", mask)?;

    xp.getattr("asarray")?.call((array,), Some(&kwargs))
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

    /// The rectangular shape of the array
    #[getter]
    fn shape<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.shape)
    }

    /// The numpy dtype of the array
    #[getter]
    fn dtype<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.data.bind(py).getattr("dtype")
    }

    /// The number of dimensions of the array
    ///
    /// This will always be ``2``.
    #[getter]
    fn ndim(&self) -> usize {
        self.shape.len()
    }

    /// apply a element-wise function
    ///
    /// For anything more elaborate, see the conversion functions or extract the offsets and data.
    ///
    /// Parameters
    /// ----------
    /// func : callable
    ///     A function that performs elementwise calculations.
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

    /// convert to a rectangular masked array
    fn as_masked_array<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let data = self.data.bind(py);
        let offsets = self.offsets.bind(py);
        if offsets.len() == 2 {
            let numpy = PyModule::import(py, "numpy")?;

            create_masked_array(
                numpy
                    .getattr("reshape")?
                    .call1((data, self.shape))?
                    .cast::<PyUntypedArray>()?,
                &PyArray2::<bool>::zeros(py, self.shape, false),
            )
        } else {
            generate_copy_for_all_dtypes!(
                offsets,
                data,
                self.shape;
                bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64
            )
        }
    }

    /// convert to a awkward array
    ///
    /// Notes
    /// -----
    /// Requires `awkward` to be installed.
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
    ///
    /// Notes
    /// -----
    /// Requires `ragged` to be installed.
    fn as_ragged<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let ragged = PyModule::import(py, "ragged")?;
        let array_cls = ragged.getattr("array")?;

        let awkward_array = self.as_awkward(py)?;

        array_cls.call1((awkward_array,))
    }
}
