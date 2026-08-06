use numpy::dtype;
use numpy::{
    Ix1, PyArrayDescrMethods, PyArrayDyn, PyArrayMethods, PyReadonlyArray, PyUntypedArray,
    PyUntypedArrayMethods,
};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyInt;

use healpix_geo_core::vectorized::depth::Depth;

pub(crate) enum DepthLike<'py> {
    Scalar(u8),
    Array(PyReadonlyArray<'py, u8, Ix1>),
}

impl<'py> FromPyObject<'_, 'py> for DepthLike<'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(value) = obj.cast::<PyInt>() {
            let scalar: u8 = value.extract()?;
            Ok(Self::Scalar(scalar))
        } else if let Ok(value) = obj.cast::<PyUntypedArray>() {
            let owned = value.to_owned();
            let array: &Bound<'py, PyUntypedArray> = owned.cast::<PyUntypedArray>()?;

            let element_type = array.dtype();
            if !element_type.is_equiv_to(&dtype::<u8>(obj.py())) {
                Err(PyTypeError::new_err("Only uint8 is supported."))
            } else {
                let array = array.cast::<PyArrayDyn<u8>>()?.reshape([array.len()])?;

                let readonly = array.readonly();

                Ok(Self::Array(readonly))
            }
        } else {
            Err(PyTypeError::new_err(
                "`depth` must be either a scalar int or an array of unsigned integers",
            ))
        }
    }
}

impl<'a> DepthLike<'a> {
    pub fn as_depth(&'a self) -> PyResult<Depth<'a>> {
        match self {
            Self::Scalar(depth) => Ok(Depth::Scalar(depth)),
            Self::Array(depths) => {
                let slice = depths.as_slice()?;
                Ok(Depth::Array(slice))
            }
        }
    }
}
