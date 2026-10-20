from collections.abc import Callable
from typing import Any, Literal, TypeVar, final

import awkward
import numpy as np
import ragged
from numpy.typing import NDArray

from healpix_geo._healpix_geo_python import geometry, nested, ring, zuniq
from healpix_geo.typing import Array1D, EllipsoidDict, EllipsoidLike, SphereDict

_S = TypeVar("_S", bound=tuple[int, ...])

@final
class RaggedArray:
    """
    very basic implementation of a ragged array

    Attributes
    ----------
    offsets : numpy.ndarray of numpy.uint64
        The offsets of each row, such that ``zip(offsets[:-1], offsets[1:])`` contains
        the start and stop values that can be used to index ``data``.
    data : numpy.ndarray
        The data of the array.
    """

    def __new__(
        cls, /, offsets: np.ndarray[tuple[int], np.dtype[np.integer]], data: np.ndarray
    ) -> RaggedArray: ...
    def apply_elementwise(self, /, func: Callable) -> RaggedArray:
        """
        apply a element-wise function

        For anything more elaborate, see the conversion functions or extract the offsets
        and data.

        Parameters
        ----------
        func : callable
            A function that performs elementwise calculations.
        """
        ...

    def as_awkward(self, /) -> awkward.Array:
        """
        convert to a awkward array

        Notes
        -----
        Requires `awkward` to be installed.
        """
        ...

    def as_masked_array(self, /) -> Any:  # noqa: ANN401
        """
        convert to a rectangular masked array
        """
        ...

    def as_ragged(self, /) -> ragged.array:
        """
        convert to a ragged array

        Notes
        -----
        Requires `ragged` to be installed.
        """
        ...

    @property
    def data(self, /) -> np.ndarray: ...
    @property
    def dtype(self, /) -> np.dtype:
        """
        The numpy dtype of the array
        """
        ...

    @property
    def ndim(self, /) -> Literal[2]:
        """
        The number of dimensions of the array

        This will always be ``2``.
        """
        ...

    @property
    def offsets(self, /) -> np.ndarray[tuple[int], np.dtype[np.uint64]]: ...
    @property
    def shape(self, /) -> tuple[int, int]:
        """
        The rectangular shape of the array
        """
        ...

def base_cell_relationship(
    base_cell: int, direction: Literal["S", "SW", "W", "NW", "N", "NE", "E", "SE"]
) -> tuple[int | None, Array1D[np.int8], Array1D[np.int8]]: ...
def cartesian_to_lonlat(
    x: np.ndarray[_S, np.dtype[np.float64]],
    y: np.ndarray[_S, np.dtype[np.float64]],
    z: np.ndarray[_S, np.dtype[np.float64]],
    ellipsoid_like: EllipsoidLike,
    nthreads: int,
) -> tuple[
    np.ndarray[_S, np.dtype[np.float64]], np.ndarray[_S, np.dtype[np.float64]]
]: ...
def lonlat_to_cartesian(
    longitude: np.ndarray[_S, np.dtype[np.float64]],
    latitude: np.ndarray[_S, np.dtype[np.float64]],
    ellipsoid_like: EllipsoidLike,
    nthreads: int,
) -> tuple[
    np.ndarray[_S, np.dtype[np.float64]],
    np.ndarray[_S, np.dtype[np.float64]],
    np.ndarray[_S, np.dtype[np.float64]],
]: ...
def resolve_ellipsoid(name: str) -> EllipsoidDict | SphereDict: ...
def vertex_to_lonlat(
    depth: int,
    ipix: NDArray[np.uint64],
    ellipsoid_like: EllipsoidLike,
    nthreads: int,
) -> tuple[NDArray[np.float64], NDArray[np.float64]]: ...

__all__ = [
    "geometry",
    "nested",
    "ring",
    "zuniq",
    "RaggedArray",
    "base_cell_relationship",
    "cartesian_to_lonlat",
    "lonlat_to_cartesian",
    "resolve_ellipsoid",
    "vertex_to_lonlat",
]
