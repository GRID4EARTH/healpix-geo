from typing import Any, Literal, TypeVar, final

import numpy as np
from numpy.typing import NDArray
from shapely import LineString, Point, Polygon

from healpix_geo._healpix_geo_python.geometry import Bbox
from healpix_geo.typing import Array1D, Array2D, DepthType, EllipsoidLike

_S = TypeVar("_S", bound=tuple[int, ...])

@final
class RangeMOCIndex:
    """
    range-based index of healpix cell ids

    The idea is to compress cell ids at depth 29 based on run-length encoding (RLE).

    Only works with cell ids following the "nested" scheme.
    """

    def __getstate__(self, /) -> bytes: ...
    def __reduce__(self, /) -> tuple[Any, Any, Any]: ...
    def __setstate__(self, /, state: bytes) -> None: ...
    def cell_ids(self, /) -> Array1D[np.uint64]:
        """
        Retrieve the cell ids from the index.

        Returns
        -------
        cell_ids : numpy.ndarray
            The cell ids contained by the index.
        """
        ...

    def compacted_cell_ids(self, /) -> Array1D[np.uint64]:
        """
        Retrieve the compacted cell ids

        For now, the cell ids will always be in the ``zuniq`` scheme.

        Returns
        -------
        compacted: numpy.ndarray
            The compacted cell ids as array of dtype ``uint64``.
        """
        ...

    @property
    def depth(self, /) -> int:
        """
        The depth of the index.
        """
        ...

    def difference(self, /, other: RangeMOCIndex) -> RangeMOCIndex:
        r"""
        Compute the set difference of two indexes

        The set difference contains all elements that are in `self` but not in `other`.

        .. math::

           A - B = { x | x \in A \land x \notin B }

        Parameters
        ----------
        other : RangeMOCIndex
            The index to subtract. May have a different depth, in which case the
            result will use the maximum depth between both indexes.

        Returns
        -------
        result : RangeMOCIndex
            The set difference of the two indexes.
        """
        ...

    @property
    def ellipsoid(self, /) -> dict[str, float]: ...
    @classmethod
    def empty(cls, /, depth: int, ellipsoid: EllipsoidLike = "sphere") -> RangeMOCIndex:
        """
        Create an empty index

        Parameters
        ----------
        depth : int
            The cell depth.
        """
        ...

    @classmethod
    def from_cell_ids(
        cls,
        /,
        depth: int,
        cell_ids: Array1D[np.uint64],
        ellipsoid: EllipsoidLike = "sphere",
    ) -> RangeMOCIndex:
        """
        Create an index from given cell ids.

        Parameters
        ----------
        depth : int
            The cell depth.
        cell_ids : numpy.ndarray
            The cells to construct the the index from.
        """
        ...

    @classmethod
    def from_compacted(
        cls,
        /,
        depth: int,
        cell_ids: Array1D[np.uint64],
        ellipsoid: EllipsoidLike = "sphere",
    ) -> RangeMOCIndex:
        """
        Create an index from the given compacted cells

        Parameters
        ----------
        depth : int
            The maximum depth of the index
        cell_ids : numpy.ndarray
            The cell ids in ``zuniq`` indexing scheme as a :math:`N` ``uint64`` array
        """
        ...

    @classmethod
    def from_ranges(
        cls,
        /,
        depth: int,
        ranges: Array2D[np.uint64],
        ellipsoid: EllipsoidLike = "sphere",
    ) -> RangeMOCIndex:
        """
        Create an index from the given ranges at level 29

        Parameters
        ----------
        depth : int
            The maximum depth of the index.
        ranges : numpy.ndarray
            The ranges to construct the index from as a :math:`N` x :math:`2` array.
        """
        ...

    @classmethod
    def full_domain(
        cls, /, depth: int, ellipsoid: EllipsoidLike = "sphere"
    ) -> RangeMOCIndex:
        """
        Create a full domain index

        This is a short-cut for creating an index for the entire sphere.

        Parameters
        ----------
        depth : int
            The cell depth.
        """
        ...

    def intersection(self, /, other: RangeMOCIndex) -> RangeMOCIndex:
        """
        Compute the set intersection of two indexes

        Parameters
        ----------
        other : RangeMOCIndex
            The other index. May have a different depth, in which case the
            result will use the maximum depth between both indexes.

        Returns
        -------
        result : RangeMOCIndex
            The intersection of the two indexes.
        """
        ...

    def isel(self, /, indexer: slice | NDArray[np.integer]) -> RangeMOCIndex:
        """
        Subset the index using positions

        Parameters
        ----------
        indexer : slice of int
            The integer positions. Currently only supports slices.

        Returns
        -------
        subset : RangeMOCIndex
            The resulting subset.
        """
        ...

    @property
    def nbytes(self, /) -> int:
        """
        The size of the ranges in bytes, minus any overhead.
        """
        ...

    def query(
        self, /, geometry: Bbox | Point | LineString | Polygon
    ) -> tuple[list[slice], RangeMOCIndex]:
        """
        Query by geometry

        Parameters
        ----------
        geometry : healpix_geo.geometry.Bbox or geometry-like
            The geometry to query by. Supported are:
            - Bbox for true bounding box queries (planar geometry)
            - shapely objects for spherical geometry queries

        Returns
        -------
        slices : list of slice
            The slices necessary for extracting the subdomain.
        moc : RangeMOCIndex
            The index for the queried cell ids.
        """
        ...

    def ranges(self, /) -> Array2D[np.uint64]:
        """
        Extract the underlying connected ranges

        Returns
        -------
        ranges: numpy.ndarray
            The ranges as a :math:`N` x :math:`2` array of dtype ``uint64``.
        """
        ...

    def refine(self, /, depth: int) -> RangeMOCIndex:
        """
        Change the internal depth of the index

        This may change the region covered by the index if the new depth is smaller than
        the current depth.

        Parameters
        ----------
        depth : int
            The new depth

        Returns
        -------
        refined : RangeMOCIndex
            The refined index with a different level
        """
        ...

    def sel(
        self, /, indexer: slice | NDArray[np.integer]
    ) -> tuple[slice | NDArray[np.integer], RangeMOCIndex]:
        """
        Subset the index using positions

        Parameters
        ----------
        indexer : slice of int or array-like
            The cell ids or ranges of cell ids to find. If an array, must be of dtype
            uint64.

        Returns
        -------
        subset : RangeMOCIndex
            The resulting subset.
        indexer : slice of int or array-like
            The integer positions of the selected cells as a uint64 array.
        """
        ...

    @property
    def size(self, /) -> int:
        """
        The number of items in the index.
        """
        ...

    def symmetric_difference(self, /, other: RangeMOCIndex) -> RangeMOCIndex:
        r"""
        Compute the symmetric set difference of two indexes

        The symmetric set difference contains all elements that are in `self` or `other` but not both:

        .. math::

           A \Delta B = (A - B) \cup (B - A)

        Parameters
        ----------
        other : RangeMOCIndex
            The index to subtract. May have a different depth, in which case the
            result will use the maximum depth between both indexes.

        Returns
        -------
        result : RangeMOCIndex
            The symmetric set difference of the two indexes.
        """
        ...

    def union(self, /, other: RangeMOCIndex) -> RangeMOCIndex:
        """
        Compute the set union of two indexes

        Parameters
        ----------
        other : RangeMOCIndex
            The other index. May have a different depth, in which case the
            result will use the maximum depth between both indexes.

        Returns
        -------
        result : RangeMOCIndex
            The union of the two indexes.
        """
        ...

def angular_distances(
    depth: int, from_: Array1D[np.uint64], to_: Array2D[np.uint64], /, nthreads: int
) -> Array2D[np.float64]:
    """
    Wrapper of `UnitVect3.ang_dist`
    The given array must be of the same size as `ipix`.
    """
    ...

def base_cell_coordinates_to_healpix(
    base_cell: np.ndarray[_S, np.dtype[np.uint8]],
    i: NDArray[np.uint32],
    j: NDArray[np.uint32],
    depth: DepthType,
    nthreads: int,
) -> np.ndarray[_S, np.dtype[np.uint64]]: ...
def bilinear_interpolation(
    depth: int,
    longitude: NDArray[np.float64],
    latitude: NDArray[np.float64],
    ellipsoid_like: EllipsoidLike,
    nthreads: int,
) -> tuple[NDArray[np.uint64], NDArray[np.uint64]]: ...
def box_coverage(
    depth: int,
    center: tuple[float, float],
    size: tuple[float, float],
    angle: float,
    *,
    ellipsoid: EllipsoidLike,
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]: ...
def cartesian_to_healpix(
    depth: int,
    x: np.ndarray[_S, np.dtype[np.float64]],
    y: np.ndarray[_S, np.dtype[np.float64]],
    z: np.ndarray[_S, np.dtype[np.float64]],
    ellipsoid_like: EllipsoidLike,
    nthreads: int,
) -> np.ndarray[_S, np.dtype[np.uint64]]: ...
def cone_coverage(
    depth: int,
    center: tuple[float, float],
    radius: float,
    *,
    ellipsoid: EllipsoidLike,
    delta_depth: int = 0,
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]: ...
def elliptical_cone_coverage(
    depth: int,
    center: tuple[float, float],
    ellipse_geometry: tuple[float, float],
    position_angle: float,
    *,
    ellipsoid: EllipsoidLike,
    delta_depth: int = 0,
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]: ...
def healpix_to_base_cell_coordinates(
    nested: np.ndarray[_S, np.dtype[np.uint64]], depth: DepthType, nthreads: int
) -> tuple[
    np.ndarray[_S, np.dtype[np.uint8]],
    np.ndarray[_S, np.dtype[np.uint32]],
    np.ndarray[_S, np.dtype[np.uint32]],
]: ...
def healpix_to_cartesian(
    depth: int,
    ipix: np.ndarray[_S, np.dtype[np.uint64]],
    ellipsoid_like: EllipsoidLike,
    nthreads: int,
) -> tuple[
    np.ndarray[_S, np.dtype[np.float64]],
    np.ndarray[_S, np.dtype[np.float64]],
    np.ndarray[_S, np.dtype[np.float64]],
]: ...
def healpix_to_lonlat(
    depth: int,
    ipix: np.ndarray[_S, np.dtype[np.uint64]],
    ellipsoid_like: EllipsoidLike,
    nthreads: int,
) -> tuple[
    np.ndarray[_S, np.dtype[np.float64]], np.ndarray[_S, np.dtype[np.float64]]
]: ...
def internal_boundary(depth: int, ipix: Array1D[np.uint64]) -> Array1D[np.uint64]:
    """
    Extract the internal boundary from a list of cells

    Parameters
    ----------
    depth : int
        The cell depth
    cell_ids : numpy.ndarray
        The cells describing a connected area

    Returns
    -------
    internal_boundary : numpy.ndarray
        The boundary cells
    """

def kth_neighbourhood(
    depth: int, ipix: NDArray[np.uint64], ring: int, nthreads: int
) -> NDArray[np.int64]:
    """
    Wrapper of `kth_neighbourhood`
    """

def kth_neighbours(
    depth: int, ipix: NDArray[np.uint64], ring: int, nthreads: int
) -> NDArray[np.int64]:
    """
    Wrapper of `kth_neighbours`
    """

def lonlat_to_healpix(
    depth: int,
    longitude: np.ndarray[_S, np.dtype[np.float64]],
    latitude: np.ndarray[_S, np.dtype[np.float64]],
    ellipsoid_like: EllipsoidLike,
    nthreads: int,
) -> np.ndarray[_S, np.dtype[np.uint64]]: ...
def neighbours(
    depth: int,
    ipix: NDArray[np.uint64],
    connectivity: Literal["edge", "vertex", "all"],
    nthreads: int,
) -> NDArray[np.int64]:
    """
    Return immediate neighbours in fixed directional positions.
    """

def polygon_coverage(
    depth: int,
    vertices: Array2D[np.float64],
    *,
    ellipsoid: EllipsoidLike,
    exact: bool = False,
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]: ...
def siblings(
    depth: int, ipix: NDArray[np.uint64], nthreads: int
) -> NDArray[np.uint64]: ...
def to_ring(
    nested: np.ndarray[_S, np.dtype[np.uint64]], depth: DepthType, nthreads: int
) -> np.ndarray[_S, np.dtype[np.uint64]]: ...
def to_zuniq(
    nested: np.ndarray[_S, np.dtype[np.uint64]], depth: DepthType, nthreads: int
) -> np.ndarray[_S, np.dtype[np.uint64]]: ...
def vertex_indices(
    depth: int, ipix: NDArray[np.uint64], nthreads: int
) -> NDArray[np.uint64]: ...
def vertices(
    depth: int,
    ipix: NDArray[np.uint64],
    ellipsoid_like: EllipsoidLike,
    step: int = 1,
    nthreads: int = 0,
) -> tuple[NDArray[np.float64], NDArray[np.float64]]: ...
def zone_coverage(
    depth: int,
    bbox: tuple[float, float, float, float],
    *,
    ellipsoid: EllipsoidLike,
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]: ...
def zoom_to(
    depth: int, ipix: NDArray[np.uint64], new_depth: int, nthreads: int
) -> NDArray[np.uint64]: ...
