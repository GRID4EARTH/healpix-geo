from typing import Literal, TypeVar

import numpy as np
from numpy.typing import NDArray

from healpix_geo.typing import Array1D, Array2D, DepthType, EllipsoidLike

_S = TypeVar("_S", bound=tuple[int, ...])

def angular_distances(
    depth: int, from_: Array1D[np.uint64], to_: Array2D[np.uint64], /, nthreads: int
) -> Array2D[np.float64]:
    """
    Wrapper of `UnitVect3.ang_dist`
    The given array must be of the same size as `ipix`.
    """
    ...

def bilinear_interpolation(
    depth: int,
    longitude: NDArray[np.float64],
    latitude: NDArray[np.float64],
    ellipsoid_like: EllipsoidLike,
    nthreads: int,
) -> tuple[NDArray[np.uint64], NDArray[np.float64]]: ...
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
    connectivity: Literal["vertex", "edge", "all"],
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
def to_nested(
    ring: np.ndarray[_S, np.dtype[np.uint64]], depth: DepthType, nthreads: int
) -> np.ndarray[_S, np.dtype[np.uint64]]: ...
def to_zuniq(
    ring: np.ndarray[_S, np.dtype[np.uint64]], depth: DepthType, nthreads: int
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
