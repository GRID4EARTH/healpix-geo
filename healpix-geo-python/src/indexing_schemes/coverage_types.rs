use numpy::{
    IxDyn, PyArrayDescrMethods, PyArrayDyn, PyArrayMethods, PyReadonlyArray, PyUntypedArray,
    PyUntypedArrayMethods, dtype,
};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

pub(crate) enum CenterLike<'py> {
    Scalar((f64, f64)),
    Array(PyReadonlyArray<'py, f64, IxDyn>),
}

impl<'py> FromPyObject<'_, 'py> for CenterLike<'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(value) = obj.cast::<PyUntypedArray>() {
            let owned = value.to_owned();
            let array: &Bound<'py, PyUntypedArray> = owned.cast::<PyUntypedArray>()?;

            let element_type = array.dtype();
            if !element_type.is_equiv_to(&dtype::<f64>(obj.py())) {
                Err(PyTypeError::new_err(
                    "only float64 is supported for center coordinates",
                ))
            } else {
                let array = array.cast::<PyArrayDyn<f64>>()?;
                let readonly = array.readonly();

                Ok(Self::Array(readonly))
            }
        } else if let Ok(value) = obj.cast::<PyTuple>() {
            let owned = value.to_owned();

            Ok(Self::Scalar(owned.extract::<(f64, f64)>()?))
        } else {
            Err(PyTypeError::new_err(
                "`center` must be either a tuple of two floats or an array of 64-bit floats",
            ))
        }
    }
}
