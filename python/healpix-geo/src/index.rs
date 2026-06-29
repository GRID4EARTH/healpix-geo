use numpy::{PyArray1, PyArrayDyn, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PySlice, PyType};
use std::collections::HashMap;

use std::cmp::PartialEq;

use crate::ellipsoid::EllipsoidLike;
use crate::geometry::GeometryTypes;

use healpix_geo_core::ellipsoid::ReferenceBody;
use healpix_geo_core::index::{
    Array, CellRegion, ConcreteSlice, LabelIndexer, PositionalIndexer, Slice,
};
use healpix_geo_core::index::{GeometryQuery, Indexing, SetOperations};

trait IntoPySlice {
    fn into_pyslice<'py>(self, py: Python<'py>) -> Bound<'py, PySlice>;
}

impl IntoPySlice for ConcreteSlice<isize> {
    fn into_pyslice<'py>(self, py: Python<'py>) -> Bound<'py, PySlice> {
        PySlice::new(py, self.start, self.stop, self.step)
    }
}

#[derive(FromPyObject, IntoPyObject)]
enum IndexKind<'py> {
    #[pyo3(transparent, annotation = "slice")]
    Slice(Bound<'py, PySlice>),
    #[pyo3(transparent, annotation = "numpy.ndarray")]
    Array(Bound<'py, PyArrayDyn<i64>>),
}

impl<'py> IndexKind<'py> {
    fn into_positional_indexer(self) -> PyResult<PositionalIndexer> {
        let positional_indexer = match self {
            Self::Slice(pyslice) => {
                let start = pyslice.getattr("start")?.extract::<Option<isize>>()?;
                let stop = pyslice.getattr("stop")?.extract::<Option<isize>>()?;
                let step = pyslice.getattr("step")?.extract::<Option<isize>>()?;

                PositionalIndexer::Slice(Slice::create(start, stop, step))
            }
            Self::Array(pyarray) => {
                let values: Vec<isize> =
                    pyarray.to_vec()?.into_iter().map(|x| x as isize).collect();

                PositionalIndexer::Array(Array::create(values))
            }
        };

        Ok(positional_indexer)
    }

    fn from_positional_indexer(
        py: Python<'py>,
        positional_indexer: PositionalIndexer,
    ) -> PyResult<Self> {
        let indexer = match positional_indexer {
            PositionalIndexer::Slice(slice) => {
                let pyslice = py
                    .import("builtins")?
                    .getattr("slice")?
                    .call1((slice.start, slice.stop, slice.step))?
                    .cast::<PySlice>()?
                    .clone();

                Self::Slice(pyslice)
            }
            PositionalIndexer::Array(array) => {
                let pyarray = PyArray1::from_iter(py, array.data.into_iter().map(|x| x as i64));
                IndexKind::Array(pyarray.to_dyn().clone())
            }
        };

        Ok(indexer)
    }

    fn into_label_indexer(self) -> PyResult<LabelIndexer> {
        let label_indexer = match self {
            Self::Slice(pyslice) => {
                let start = pyslice.getattr("start")?.extract::<Option<u64>>()?;
                let stop = pyslice.getattr("stop")?.extract::<Option<u64>>()?;
                let step = pyslice.getattr("step")?.extract::<Option<u64>>()?;

                LabelIndexer::Slice(Slice::create(start, stop, step))
            }
            Self::Array(pyarray) => {
                let values: Vec<u64> = pyarray.to_vec()?.into_iter().map(|x| x as u64).collect();

                LabelIndexer::Array(Array::create(values))
            }
        };

        Ok(label_indexer)
    }
}

/// range-based index of healpix cell ids
///
/// The idea is to compress cell ids at depth 29 based on run-length encoding (RLE).
///
/// Only works with cell ids following the "nested" scheme.
#[derive(PartialEq, Debug, Clone)]
#[pyclass(from_py_object)]
#[pyo3(module = "healpix_geo.nested")]
pub struct RangeMOCIndex {
    region: CellRegion,
}

#[pymethods]
impl RangeMOCIndex {
    /// Create a full domain index
    ///
    /// This is a short-cut for creating an index for the entire sphere.
    ///
    /// Parameters
    /// ----------
    /// depth : int
    ///     The cell depth.
    #[pyo3(signature = (depth, ellipsoid=EllipsoidLike::Named("sphere".to_string())))]
    #[classmethod]
    fn full_domain(
        _cls: &Bound<'_, PyType>,
        depth: u8,
        ellipsoid: EllipsoidLike,
    ) -> PyResult<Self> {
        let index = RangeMOCIndex {
            region: CellRegion::full_domain(depth, ellipsoid.into_ellipsoid()?),
        };

        Ok(index)
    }

    /// Create an empty index
    ///
    /// Parameters
    /// ----------
    /// depth : int
    ///     The cell depth.
    #[pyo3(signature = (depth, ellipsoid=EllipsoidLike::Named("sphere".to_string())))]
    #[classmethod]
    fn empty(_cls: &Bound<'_, PyType>, depth: u8, ellipsoid: EllipsoidLike) -> PyResult<Self> {
        let index = RangeMOCIndex {
            region: CellRegion::empty(depth, ellipsoid.into_ellipsoid()?),
        };

        Ok(index)
    }

    /// Create an index from given cell ids.
    ///
    /// Parameters
    /// ----------
    /// depth : int
    ///     The cell depth.
    /// cell_ids : numpy.ndarray
    ///     The cells to construct the the index from.
    #[pyo3(signature = (depth, cell_ids, ellipsoid=EllipsoidLike::Named("sphere".to_string())))]
    #[classmethod]
    fn from_cell_ids<'a>(
        _cls: &Bound<'a, PyType>,
        _py: Python,
        depth: u8,
        cell_ids: &Bound<'a, PyArray1<u64>>,
        ellipsoid: EllipsoidLike,
    ) -> PyResult<Self> {
        let index = RangeMOCIndex {
            region: CellRegion::from_cell_ids(
                depth,
                cell_ids.to_vec()?,
                ellipsoid.into_ellipsoid()?,
            ),
        };

        Ok(index)
    }

    /// Compute the set union of two indexes
    ///
    /// Parameters
    /// ----------
    /// other : RangeMOCIndex
    ///     The other index. May have a different depth, in which case the
    ///     result will use the maximum depth between both indexes.
    ///
    /// Returns
    /// -------
    /// result : RangeMOCIndex
    ///     The union of the two indexes.
    fn union(&self, other: &RangeMOCIndex) -> Self {
        RangeMOCIndex {
            region: self.region.union(&other.region),
        }
    }

    /// Compute the set intersection of two indexes
    ///
    /// Parameters
    /// ----------
    /// other : RangeMOCIndex
    ///     The other index. May have a different depth, in which case the
    ///     result will use the maximum depth between both indexes.
    ///
    /// Returns
    /// -------
    /// result : RangeMOCIndex
    ///     The intersection of the two indexes.
    fn intersection(&self, other: &RangeMOCIndex) -> Self {
        RangeMOCIndex {
            region: self.region.intersection(&other.region),
        }
    }

    /// Compute the set difference of two indexes
    ///
    /// The set difference contains all elements that are in `self` but not in `other`.
    ///
    /// .. math::
    ///
    ///    A - B = { x | x \in A \land x \notin B }
    ///
    /// Parameters
    /// ----------
    /// other : RangeMOCIndex
    ///     The index to subtract. May have a different depth, in which case the
    ///     result will use the maximum depth between both indexes.
    ///
    /// Returns
    /// -------
    /// result : RangeMOCIndex
    ///     The set difference of the two indexes.
    fn difference(&self, other: &RangeMOCIndex) -> Self {
        RangeMOCIndex {
            region: self.region.difference(&other.region),
        }
    }

    /// Compute the symmetric set difference of two indexes
    ///
    /// The symmetric set difference contains all elements that are in `self` or `other` but not both:
    ///
    /// .. math::
    ///
    ///    A \Delta B = (A - B) \cup (B - A)
    ///
    /// Parameters
    /// ----------
    /// other : RangeMOCIndex
    ///     The index to subtract. May have a different depth, in which case the
    ///     result will use the maximum depth between both indexes.
    ///
    /// Returns
    /// -------
    /// result : RangeMOCIndex
    ///     The symmetric set difference of the two indexes.
    fn symmetric_difference(&self, other: &RangeMOCIndex) -> Self {
        RangeMOCIndex {
            region: self.region.symmetric_difference(&other.region),
        }
    }

    /// The size of the ranges in bytes, minus any overhead.
    #[getter]
    fn nbytes(&self) -> u64 {
        self.region.nbytes() as u64
    }

    /// The number of items in the index.
    #[getter]
    fn size(&self) -> u64 {
        self.region.size() as u64
    }

    /// The depth of the index.
    #[getter]
    fn depth(&self) -> u8 {
        self.region.depth()
    }

    #[getter]
    fn ellipsoid(&self) -> HashMap<String, f64> {
        self.region.ellipsoid().to_mapping()
    }

    pub fn __setstate__(&mut self, state: &[u8]) -> PyResult<()> {
        // Deserialize the data contained in the PyBytes object
        // and update the struct with the deserialized values.
        // serde+bincode version:
        // *self = deserialize(state).map_err(|err| PyRuntimeError::new_err(err.to_string()))?;

        let reconstructed_region = CellRegion::from_bytes(state);

        *self = RangeMOCIndex {
            region: reconstructed_region,
        };

        Ok(())
    }

    pub fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        // Serialize the struct and return a PyBytes object
        // containing the serialized data.
        let serialized = self.region.to_bytes();
        let bytes = PyBytes::new(py, &serialized);
        Ok(bytes)
    }

    pub fn __reduce__(&self, py: Python) -> PyResult<(Py<PyAny>, Py<PyAny>, Py<PyAny>)> {
        let create = py
            .import("healpix_geo")?
            .getattr("nested")?
            .getattr("create_empty")?;
        let args = (self.region.depth(),);
        let state = self.__getstate__(py)?;

        Ok((
            create.into_pyobject(py)?.unbind().into_any(),
            args.into_pyobject(py)?.unbind().into_any(),
            state.into_pyobject(py)?.unbind().into_any(),
        ))
    }

    /// Retrieve the cell ids from the index.
    ///
    /// Returns
    /// -------
    /// cell_ids : numpy.ndarray
    ///     The cell ids contained by the index.
    fn cell_ids<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyArray1<u64>>> {
        let cell_ids: Vec<u64> = self.region.cell_ids();

        Ok(PyArray1::from_vec(py, cell_ids))
    }

    /// Subset the index using positions
    ///
    /// Parameters
    /// ----------
    /// indexer : slice of int
    ///     The integer positions. Currently only supports slices.
    ///
    /// Returns
    /// -------
    /// subset : RangeMOCIndex
    ///     The resulting subset.
    fn isel<'a>(&self, _py: Python<'a>, indexer: IndexKind<'a>) -> PyResult<Self> {
        let positional_indexer = indexer.into_positional_indexer()?;

        let region = self.region.isel(&positional_indexer);
        let new_index = Self { region };

        Ok(new_index)
    }

    /// Subset the index using positions
    ///
    /// Parameters
    /// ----------
    /// indexer : slice of int or array-like
    ///     The cell ids or ranges of cell ids to find. If an array, must be of dtype uint64.
    ///
    /// Returns
    /// -------
    /// subset : RangeMOCIndex
    ///     The resulting subset.
    /// indexer : slice of int or array-like
    ///     The integer positions of the selected cells as a uint64 array.
    fn sel<'a>(&self, py: Python<'a>, indexer: IndexKind<'a>) -> PyResult<(IndexKind<'a>, Self)> {
        let label_indexer = indexer.into_label_indexer()?;
        let (region, positional_indexer) = self.region.sel(&label_indexer);

        let pyindexer = IndexKind::from_positional_indexer(py, positional_indexer)?;
        let new_index = Self { region };

        Ok((pyindexer, new_index))
    }

    /// Query by geometry
    ///
    /// Parameters
    /// ----------
    /// geometry : healpix_geo.geometry.Bbox or geometry-like
    ///     The geometry to query by. Supported are:
    ///     - Bbox for true bounding box queries (planar geometry)
    ///     - shapely objects for spherical geometry queries
    ///
    /// Returns
    /// -------
    /// slices : list of slice
    ///     The slices necessary for extracting the subdomain.
    /// moc : RangeMOCIndex
    ///     The index for the queried cell ids.
    fn query<'py>(
        &self,
        py: Python<'py>,
        geometry: &Bound<'py, PyAny>,
    ) -> PyResult<(Vec<Bound<'py, PySlice>>, Self)> {
        let geom = GeometryTypes::from_pyobject(py, geometry)?.into_geometry()?;
        let (positional_slices, new_region) = self.region.query(&geom);

        Ok((
            positional_slices
                .into_iter()
                .map(|x| x.into_pyslice(py))
                .collect(),
            RangeMOCIndex { region: new_region },
        ))
    }
}
