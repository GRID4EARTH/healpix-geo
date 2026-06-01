use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyRange, PySlice, PySliceMethods, PyTuple, PyType};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// More powerful version of the built-in slice
///
/// For positional indexing.
#[derive(PartialEq, PartialOrd, Debug, Clone)]
#[pyclass]
#[pyo3(module = "healpix_geo.slices", name = "Slice", frozen)]
pub struct PositionalSlice {
    #[pyo3(get)]
    pub start: Option<isize>,
    #[pyo3(get)]
    pub stop: Option<isize>,
    #[pyo3(get)]
    pub step: Option<isize>,
}

/// Slice with concrete values
///
/// Note: no `None` values allowed.
#[derive(PartialEq, PartialOrd, Debug, Clone)]
#[pyclass]
#[pyo3(module = "healpix_geo.slices", name = "ConcreteSlice", frozen)]
pub struct ConcreteSlice {
    #[pyo3(get)]
    pub start: isize,
    #[pyo3(get)]
    pub stop: isize,
    #[pyo3(get)]
    pub step: isize,
}

/// Multiple concrete positional slices
#[derive(PartialEq, PartialOrd, Debug, Clone)]
#[pyclass]
#[pyo3(module = "healpix_geo.slices", name = "MultiConcreteSlice", frozen)]
pub struct MultiConcreteSlice {
    #[pyo3(get)]
    pub slices: Vec<ConcreteSlice>,
}


impl ConcreteSlice {
    pub fn join_slices(slices: Vec<Self>) -> Result<Self, String> {
        if slices.is_empty() {
            Err("Empty list".to_string())
        } else if slices.len() == 1 {
            Ok(slices[0].clone())
        } else if !slices
            .windows(2)
            .map(|window| {
                let a = window.first().unwrap();
                let b = window.last().unwrap();

                a.step == b.step
            })
            .reduce(|a, b| a & b)
            .unwrap()
        {
            Err("Step sizes are not equal".to_string())
        } else if !slices
            .windows(2)
            .map(|window| {
                let a = window.first().unwrap();
                let b = window.last().unwrap();
                a.stop == b.start
            })
            .reduce(|a, b| a & b)
            .unwrap()
        {
            Err("Slices are not contiguous".to_string())
        } else {
            let first = slices.first().unwrap();
            let last = slices.last().unwrap();

            Ok(ConcreteSlice {
                start: first.start,
                stop: last.stop,
                step: first.step,
            })
        }
    }
}

#[pymethods]
impl ConcreteSlice {
    fn __repr__(&self) -> String {
        format!(
            "ConcreteSlice({0}, {1}, {2})",
            self.start, self.stop, self.step
        )
    }

    /// Compute the size of the slice
    pub fn size(&self, py: Python<'_>) -> PyResult<usize> {
        let range = PyRange::new_with_step(py, self.start, self.stop, self.step)?;

        range.len()
    }

    /// Extract the elements of the slice
    pub fn indices<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyTuple>> {
        PyTuple::new(py, vec![self.start, self.stop, self.step])
    }

    /// Convert to a python slice
    pub fn as_pyslice<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PySlice>> {
        let builtins = py.import("builtins")?;

        let result = builtins
            .getattr("slice")?
            .call1((self.start, self.stop, self.step))?;

        Ok(result.extract::<Bound<'a, PySlice>>()?)
    }

    #[classmethod]
    pub fn join<'a>(
        _cls: &Bound<'a, PyType>,
        _py: Python<'a>,
        slices: Vec<Self>,
    ) -> PyResult<Self> {
        Self::join_slices(slices).map_err(PyValueError::new_err)
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.start.hash(&mut hasher);
        self.stop.hash(&mut hasher);
        self.step.hash(&mut hasher);

        hasher.finish()
    }
}

#[pyclass]
struct SliceIterator {
    inner: std::vec::IntoIter<ConcreteSlice>,
}

#[pymethods]
impl SliceIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<ConcreteSlice> {
        slf.inner.next()
    }
}

#[pymethods]
impl MultiConcreteSlice {
    fn __repr__(&self) -> String {
        format!(
            "MultiConcreteSlice([{0}])",
            self.slices
                .iter()
                .map(|s| s.__repr__())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<SliceIterator>> {
        let iter = SliceIterator {
            inner: slf.slices.clone().into_iter(),
        };
        Py::new(slf.py(), iter)
    }

    /// total size of the concrete multi-slice
    pub fn size(&self, py: Python<'_>) -> usize {
        self.slices.iter().map(|s| s.size(py).unwrap()).sum()
    }
}
