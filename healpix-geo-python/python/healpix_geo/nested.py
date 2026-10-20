from __future__ import annotations

from typing import TYPE_CHECKING, Any

import marray
import numpy as np

from healpix_geo import _healpix_geo_python
from healpix_geo.utils import (
    check_depth,
    check_ring,
    check_same_shape,
    validate_cell_ids,
    validate_depth,
    validate_tuple2,
    validate_tuple4,
)

if TYPE_CHECKING:
    from typing import Literal

    from numpy.typing import NDArray

    from healpix_geo.typing import Array1D, ArrayLikeFloat, ArrayLikeInt, EllipsoidLike

    Direction = Literal["S", "SW", "W", "NW", "N", "NE", "E", "SE"]

RangeMOCIndex = _healpix_geo_python.nested.RangeMOCIndex
internal_boundary = _healpix_geo_python.nested.internal_boundary


def create_empty(depth: int) -> RangeMOCIndex:
    return RangeMOCIndex.empty(depth)


def to_ring(
    ipix: ArrayLikeInt, depth: ArrayLikeInt, *, num_threads: int = 0
) -> NDArray[np.uint64]:
    """Convert from nested to ring

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the nested scheme.
    depth : int or array-like of int
        The depth of the HEALPix cells given as scalar or an array of the same shape as
        `ipix`.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    ring : numpy.ndarray of numpy.uint64
        The HEALPix cell indices in the ring scheme, with the same shape as the inputs.

    Raises
    ------
    ValueError
        If `ipix` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` has values outside the range [0, 29].
    ValueError
        If `depth` is an array with a different shape as `ipix`.

    Examples
    --------
    >>> import healpix_geo.nested
    >>> import numpy as np
    >>> ipix_nested = np.array([32, 125, 45, 91], dtype=np.uint64)
    >>> depth = np.array([1, 3, 2, 4], dtype=np.uint8)
    >>> ipix_ring = healpix_geo.nested.to_ring(ipix_nested, depth)
    >>> ipix_ring
    array([ 44,   7,   9, 432], dtype=uint64)
    """
    depth = validate_depth(depth)
    ipix = validate_cell_ids(ipix, depth)

    return _healpix_geo_python.nested.to_ring(ipix, depth, num_threads)


def to_zuniq(
    ipix: ArrayLikeInt, depth: ArrayLikeInt, *, num_threads: int = 0
) -> NDArray[np.uint64]:
    """Convert from nested to zuniq

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the nested scheme.
    depth : int or array-like of int
        The depth of the HEALPix cells given as scalar or an array of the same shape as
        `ipix`.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    zuniq : numpy.ndarray of numpy.uint64
        The cell indices in the zuniq scheme, with the same shape as the inputs.

    Raises
    ------
    ValueError
        If `ipix` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` has values outside the range [0, 29].
    ValueError
        If `depth` is an array with a different shape as `ipix`.

    Examples
    --------
    >>> import healpix_geo.nested
    >>> import numpy as np
    >>> ipix_nested = np.array([32, 125, 45, 91], dtype=np.uint64)
    >>> depth = np.array([1, 3, 2, 4], dtype=np.uint8)
    >>> ipix_zuniq = healpix_geo.nested.to_zuniq(ipix_nested, depth)
    >>> ipix_zuniq
    array([4683743612465315840, 1130403506469994496, 1639310264362860544,
            206039682952200192], dtype=uint64)
    """
    depth = validate_depth(depth)
    ipix = validate_cell_ids(ipix, depth)

    return _healpix_geo_python.nested.to_zuniq(ipix, depth, num_threads)


def healpix_to_lonlat(
    ipix: ArrayLikeInt,
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    num_threads: int = 0,
) -> tuple[NDArray[np.float64], NDArray[np.float64]]:
    """Get the longitudes and latitudes of the center of some HEALPix cells.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the nested indexing scheme.
    depth : int
        The depth of the HEALPix cells.
    ellipsoid : ellipsoid-like, default: "sphere"
        The reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.healpix_to_lonlat`.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    lon, lat : numpy.ndarray of numpy.float64
        The longitude and latitude of the HEALPix cells centers, in decimal degrees,
        with the same shape as the input.

    Raises
    ------
    ValueError
        If `ipix` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo.nested import healpix_to_lonlat
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> depth = 3
    >>> lon, lat = healpix_to_lonlat(ipix, depth, ellipsoid="WGS84")
    >>> lon
    array([ 5.625, 50.625, 28.125])
    >>> lat
    array([41.93785391, 19.55202227, 19.55202227])
    """
    check_depth(depth)
    ipix = validate_cell_ids(ipix, depth)

    return _healpix_geo_python.nested.healpix_to_lonlat(
        depth, ipix, ellipsoid, num_threads
    )


def lonlat_to_healpix(
    longitude: ArrayLikeFloat,
    latitude: ArrayLikeFloat,
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    num_threads: int = 0,
) -> NDArray[np.uint64]:
    """Get the HEALPix indices that contains specific points.

    Parameters
    ----------
    longitude : array-like of float
        The longitudes of the input points, in degrees.
    latitude : array-like of float
        The latitudes of the input points, in degrees.
    depth : int
        The depth of the HEALPix cells.
    ellipsoid : ellipsoid-like, default: "sphere"
        The reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.lonlat_to_healpix`.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    ipix : numpy.ndarray of numpy.uint64
        The corresponding HEALPix cell indices in the nested indexing scheme, with the
        same shape as the inputs.

    Raises
    ------
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If `longitude` and `latitude` do not have the same shape.
    ValueError
        If the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo.nested import lonlat_to_healpix
    >>> import numpy as np
    >>> lon = np.array([0, 50, 25], dtype=np.float64)
    >>> lat = np.array([6, -12, 45], dtype=np.float64)
    >>> depth = 3
    >>> ipix = lonlat_to_healpix(lon, lat, depth, ellipsoid="WGS84")
    >>> ipix
    array([304, 573,  38], dtype=uint64)
    """
    check_depth(depth)
    longitude = np.atleast_1d(longitude).astype(np.float64)
    latitude = np.atleast_1d(latitude).astype(np.float64)

    check_same_shape(longitude=longitude, latitude=latitude)

    return _healpix_geo_python.nested.lonlat_to_healpix(
        depth, longitude, latitude, ellipsoid, num_threads
    )


def healpix_to_cartesian(
    ipix: ArrayLikeInt,
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    num_threads: int = 0,
) -> tuple[NDArray[np.float64], NDArray[np.float64], NDArray[np.float64]]:
    """Get the cartesian coordinates of the center of the given HEALPix cells.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the nested indexing scheme.
    depth : int
        The depth of the HEALPix cells.
    ellipsoid : ellipsoid-like, default: "sphere"
        The reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.healpix_to_lonlat`.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    x, y, z : numpy.ndarray of numpy.float64
        The cartesian coordinates of the center of the HEALPix cells, in meters, with
        the same shape as the input.

    Raises
    ------
    ValueError
        If `ipix` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo.nested import healpix_to_cartesian
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> depth = 3
    >>> x, y, z = healpix_to_cartesian(ipix, depth, ellipsoid="WGS84")
    >>> x
    array([4728734.69011096, 3814362.85063174, 5302653.40426395])
    >>> y
    array([ 465739.71573273, 4647814.58136658, 2834327.29466645])
    >>> z
    array([4240471.60205904, 2121029.89621885, 2121029.89621885])
    """
    check_depth(depth)
    ipix = validate_cell_ids(ipix, depth)

    return _healpix_geo_python.nested.healpix_to_cartesian(
        depth, ipix, ellipsoid, num_threads
    )


def cartesian_to_healpix(
    x: ArrayLikeFloat,
    y: ArrayLikeFloat,
    z: ArrayLikeFloat,
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    num_threads: int = 0,
) -> NDArray[np.uint64]:
    """Get the HEALPix indices that contain specific points.

    Parameters
    ----------
    x : array-like of float
        The x coordinate of the input points, in meters.
    y : array-like of float
        The y coordinate of the input points, in meters.
    z : array-like of float
        The z coordinate of the input points, in meters.
    depth : int
        The depth of the HEALPix cells.
    ellipsoid : ellipsoid-like, default: "sphere"
        The reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.lonlat_to_healpix`.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    ipix : numpy.ndarray of numpy.uint64
        The corresponding HEALPix cell indices in the nested indexing scheme, with the
        same shape as the inputs.

    Raises
    ------
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If `x`, `y` and `z` do not have the same shape.
    ValueError
        If the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo.nested import cartesian_to_healpix
    >>> import numpy as np
    >>> x = np.array(
    ...     [6343428.894701699, 4010777.6054728953, 4094327.79214653], dtype=np.float64
    ... )
    >>> y = np.array([0.0, 4779858.620418726, 1909216.4044747741], dtype=np.float64)
    >>> z = np.array(
    ...     [662257.957592079, -1317402.5312295998, 4487348.408865919], dtype=np.float64
    ... )
    >>> depth = 3
    >>> ipix = cartesian_to_healpix(x, y, z, depth, ellipsoid="WGS84")
    >>> ipix
    array([304, 573,  38], dtype=uint64)
    """
    check_depth(depth)
    x = np.atleast_1d(x).astype(np.float64)
    y = np.atleast_1d(y).astype(np.float64)
    z = np.atleast_1d(z).astype(np.float64)

    check_same_shape(x=x, y=y, z=z)

    return _healpix_geo_python.nested.cartesian_to_healpix(
        depth, x, y, z, ellipsoid, num_threads
    )


def healpix_to_base_cell_coordinates(
    cell_ids: ArrayLikeInt, depth: ArrayLikeInt, *, num_threads: int = 0
) -> tuple[NDArray[np.uint8], NDArray[np.uint32], NDArray[np.uint32]]:
    """Convert cell indices to base cell-local coordinates.

    This is an integer-only topology operation. At ``depth``, each of the 12
    base faces is a ``2**depth`` by ``2**depth`` grid. Output shapes are
    preserved.

    Parameters
    ----------
    cell_ids : array-like of int
        The HEALPix cell indices in the nested indexing scheme.
    depth : int or array-like of int
        The depth of the HEALPix cells given as scalar or an array of the same shape as
        `cell_ids`.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    base_cell : numpy.ndarray of numpy.uint8
        Array of base cell indices, with the same shape as the input.
    i, j : numpy.ndarray of numpy.uint32
        Array of base cell-local coordinates, with the same shape as the input.

    Raises
    ------
    ValueError
        If `cell_ids` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` has values outside the range [0, 29].
    ValueError
        If `depth` is an array with a different shape as `cell_ids`.

    Examples
    --------
    >>> from healpix_geo.nested import healpix_to_base_cell_coordinates
    >>> base_cells, i, j = healpix_to_base_cell_coordinates([0, 3, 4, 47], 1)
    >>> base_cells
    array([ 0,  0,  1, 11], dtype=uint8)
    >>> i
    array([0, 1, 0, 1], dtype=uint32)
    >>> j
    array([0, 1, 0, 1], dtype=uint32)
    """
    depth = validate_depth(depth)
    cell_ids = validate_cell_ids(cell_ids, depth)
    cell_ids = np.ascontiguousarray(cell_ids)

    return _healpix_geo_python.nested.healpix_to_base_cell_coordinates(
        cell_ids, depth, num_threads
    )


def base_cell_coordinates_to_healpix(
    base_cell: ArrayLikeInt,
    i: ArrayLikeInt,
    j: ArrayLikeInt,
    depth: ArrayLikeInt,
    *,
    num_threads: int = 0,
) -> NDArray[np.uint64]:
    """Convert base cell-local coordinates to cell indices.

    ``base_cell``, ``i``, ``j``, and an array-valued ``depth`` must have the
    same shape. Coordinates must lie inside the selected base face.

    Parameters
    ----------
    base_cell : array-like of int
        Base cell id in the closed range ``[0, 11]``.
    i, j : array-like of int
        Base cell-local coordinates in the range of ``[0, 2**depth - 1]``.
    depth : int or array-like of int
        The depth of the HEALPix cells given as scalar or an array of the same shape as
        other inputs.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    ipix : numpy.ndarray of numpy.uint64
        The corresponding HEALPix cell indices in the nested indexing scheme, with the
        same shape as the inputs.

    Raises
    ------
    ValueError
        If `base_cell`, `i` and `j` do not have the same shape.
    ValueError
        If `depth` has values outside the range [0, 29].
    ValueError
        If `depth` is an array with a different shape as other input arrays.

    Examples
    --------
    >>> from healpix_geo.nested import base_cell_coordinates_to_healpix
    >>> base_cell_coordinates_to_healpix([0, 0, 1, 11], [0, 1, 0, 1], [0, 1, 0, 1], 1)
    array([ 0,  3,  4, 47], dtype=uint64)
    """
    depth = validate_depth(depth)

    # do not use validate_cell_ids to have more control over dtypes
    base_cell = np.atleast_1d(base_cell)
    i = np.atleast_1d(i)
    j = np.atleast_1d(j)

    check_same_shape(base_cell=base_cell, i=i, j=j)
    if (
        not isinstance(depth, int) and depth.ndim > 0
    ) and depth.shape != base_cell.shape:
        raise ValueError(
            "An array valued depth must have the same shape as the other arrays."
            f" Got {depth.shape}."
        )

    if np.any(base_cell < 0) or np.any(base_cell > 11):
        raise ValueError("Base cell must be in range [0, 11].")

    nside = 2**depth
    if np.any(i < 0) or np.any(i >= nside):
        raise ValueError("i must be in the [0, 2**depth - 1] closed range.")
    if np.any(j < 0) or np.any(j >= nside):
        raise ValueError("j must be in the [0, 2**depth - 1] closed range.")

    return _healpix_geo_python.nested.base_cell_coordinates_to_healpix(
        np.ascontiguousarray(base_cell, dtype=np.uint8),
        np.ascontiguousarray(i, dtype=np.uint32),
        np.ascontiguousarray(j, dtype=np.uint32),
        depth,
        num_threads,
    )


def vertices(
    ipix: ArrayLikeInt,
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    step: int = 1,
    num_threads: int = 0,
) -> tuple[NDArray[np.float64], NDArray[np.float64]]:
    """Get the longitudes and latitudes of the vertices of some HEALPix cells at a given depth.

    This method returns the 4 vertices of each cell in `ipix`.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the nested indexing scheme.
    depth : int
        The depth of the HEALPix cells.
    ellipsoid : ellipsoid-like, default: "sphere"
        The reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.vertices`.
    step : int, default: 1
        The number of subdivision of the cell edges. At 1, only return the 4 corners of
        the cell. At 2, return the 4 corners of the cell plus the center of each edge.
        And so on. The number of vertices returned is ``4 * step``.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    longitude, latitude : numpy.ndarray of numpy.float64
        The coordinates of the vertices of the HEALPix cells, in degrees, with the same
        shape is the input plus one axis of size ``4 * step``. If `ipix` is of shape S,
        the output will be of shape ``(*S, 4 * step)``.

    Raises
    ------
    ValueError
        If `ipix` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo.nested import vertices
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> depth = 12
    >>> lon, lat = vertices(ipix, depth, ellipsoid="sphere")
    >>> np.stack([lon, lat], axis=-1)
    array([[[4.49230957e+01, 6.52784088e-02],
            [4.49340820e+01, 7.46039007e-02],
            [4.49230957e+01, 8.39293945e-02],
            [4.49121094e+01, 7.46039007e-02]],
    <BLANKLINE>
           [[4.50109863e+01, 2.79764560e-02],
            [4.50219727e+01, 3.73019424e-02],
            [4.50109863e+01, 4.66274299e-02],
            [4.50000000e+01, 3.73019424e-02]],
    <BLANKLINE>
           [[4.49670410e+01, 2.79764560e-02],
            [4.49780273e+01, 3.73019424e-02],
            [4.49670410e+01, 4.66274299e-02],
            [4.49560547e+01, 3.73019424e-02]]])
    """
    check_depth(depth)
    ipix = validate_cell_ids(ipix, depth)
    return _healpix_geo_python.nested.vertices(
        depth, ipix, ellipsoid, step, num_threads
    )


def vertex_indices(
    ipix: ArrayLikeInt, depth: int, *, num_threads: int = 0
) -> NDArray[np.uint64]:
    """Get the indices of the 4 vertices of the given cells.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the nested indexing scheme.
    depth : int
        The depth of the HEALPix cells.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    vertex_ids : numpy.ndarray of numpy.uint64
        The indices of vertices of the given cells, with the same shape as the input.

    Raises
    ------
    ValueError
        If `ipix` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` is outside the range [0, 29].

    See Also
    --------
    healpix_geo.vertex_to_lonlat

    Examples
    --------
    >>> from healpix_geo.nested import vertex_indices
    >>> import numpy as np

    >>> ipix = np.array([42, 6, 10])
    >>> depth = 12

    >>> vertex_ids = vertex_indices(ipix, depth)
    >>> vertex_ids
    array([[100542461, 100526078, 100509693, 100526077],
           [100608001, 100591618, 100575233, 100591617],
           [100607999, 100591616, 100575231, 100591615]], dtype=uint64)
    """
    check_depth(depth)
    ipix = validate_cell_ids(ipix, depth)
    return _healpix_geo_python.nested.vertex_indices(depth, ipix, num_threads)


def bilinear_interpolation(
    longitude: ArrayLikeFloat,
    latitude: ArrayLikeFloat,
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    num_threads: int = 0,
) -> tuple[Any, Any]:
    """Get the cell indices and weights necessary to bilinearly interpolate the given values.

    The returned array are masked where the weights are zero. They are created using
    `marray <https://pypi.org/project/marray/>`__ and thus are compatible with the
    Python array API standard.

    Parameters
    ----------
    longitude : array-like of float
        The longitudes of the input points, in degrees.
    latitude : array-like of float
        The latitudes of the input points, in degrees.
    depth : int
        The depth of the HEALPix cells.
    ellipsoid : ellipsoid-like, default: "sphere"
        The reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.vertices`.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    cell_ids : masked numpy.ndarray of numpy.uint64
        The indices of the HEALPix cells around the given points, in the nested indexing
        scheme, with the same shape as the inputs plus one axis of size 4. If the inputs
        have shape S the output will be of shape ``(*S, 4)``. The array is masked where
        the weights are zero.
    weights : masked numpy.ndarray of numpy.float64
        The interpolation weights, with the same shape as the inputs plus one axis of
        size 4. The array is masked where the weights are zero.

    Raises
    ------
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If `longitude` and `latitude` do not have the same shape.
    ValueError
        If the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo.nested import bilinear_interpolation
    >>> import numpy as np

    Define coordinates

    >>> lon = np.array([-15.0, -10.0, -5.0, 0.0, 5.0])
    >>> lat = np.array([30.0, 35.0, 40.0, 45.0, 50.0])

    Compute interpolation weights

    >>> cell_ids, weights = bilinear_interpolation(lon, lat, depth=6, ellipsoid="WGS84")
    >>> cell_ids
    MArray(
        array([[13400, 13401, 13402, 13403],
               [13581, 13592, 13583, 13594],
               [13638, 13639, 13644, 13645],
               [ 2735,  2746, 13663, 13685],
               [ 2800,  2801,  2802,  2803]], dtype=uint64),
        array([[False, False, False, False],
               [False, False, False, False],
               [False, False, False, False],
               [False, False, False, False],
               [False, False, False, False]])
    )
    >>> weights
    MArray(
        array([[0.22596183, 0.68795133, 0.02128467, 0.06480216],
               [0.15244623, 0.78751507, 0.00973729, 0.05030142],
               [0.04859157, 0.12318081, 0.23429192, 0.5939357 ],
               [0.32719215, 0.17280785, 0.32719215, 0.17280785],
               [0.14255714, 0.34522269, 0.1497    , 0.36252017]]),
        array([[False, False, False, False],
               [False, False, False, False],
               [False, False, False, False],
               [False, False, False, False],
               [False, False, False, False]])
    )
    """
    check_depth(depth)
    longitude = np.atleast_1d(longitude).astype(np.float64)
    latitude = np.atleast_1d(latitude).astype(np.float64)

    check_same_shape(longitude=longitude, latitude=latitude)

    ipix, weights = _healpix_geo_python.nested.bilinear_interpolation(
        depth, longitude, latitude, ellipsoid, num_threads
    )

    xp = marray.masked_namespace(np)
    mask = weights == 0

    return xp.asarray(ipix, mask=mask), xp.asarray(weights, mask=mask)


def neighbours(
    ipix: ArrayLikeInt,
    depth: int,
    *,
    connectivity: Literal["edge", "vertex", "all"] = "all",
    num_threads: int = 0,
) -> NDArray[np.int64]:
    """Get the direction-preserving immediate neighbours of HEALPix cells.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the nested indexing scheme.
    depth : int
        The depth of the HEALPix cells.
    connectivity : {"edge", "vertex", "all"}, default: "all"
        The connectivity of the neighbours:

        - ``"edge"`` connectivity returns the four edge-sharing neighbours in ``SW, NW,
          NE, SE`` order.
        - ``"vertex"`` connectivity returns the four vertex (but not edge) sharing
          neighbours. The order is ``S, W, N, E``.
        - ``"all"`` returns all immediate neighbours. The order is always ``S, SW, W,
          NW, N, NE, E, SE``.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    neighbours : numpy.ndarray of numpy.int64
        The HEALPix cell indices of the neighbours of each cell, with the same shape as
        the input plus one axis of size 4 for edge or vertex connectivity and size 8 for
        both together. If `ipix` has shape S, the output will be of shape
        ``(*S, {4, 8})``. Missing neighbours will be represented by -1.

    Raises
    ------
    ValueError
        If `ipix` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` is outside the range [0, 29].

    """
    check_depth(depth)
    ipix = validate_cell_ids(ipix, depth)
    return _healpix_geo_python.nested.neighbours(depth, ipix, connectivity, num_threads)


def kth_neighbours(
    ipix: ArrayLikeInt, depth: int, ring: int, *, num_threads: int = 0
) -> NDArray[np.int64]:
    """Get the kth ring of neighbouring cells around some HEALPix cells at a given depth.

    This method returns an array containing the ``8 * ring`` neighbours of each input
    cell. This method is wrapped around the `kth_neighbours
    <https://docs.rs/cdshealpix/0.9.1/cdshealpix/nested/struct.Layer.html#method.kth_neighbours>`__
    method of the `cdshealpix Rust crate <https://crates.io/crates/cdshealpix>`__.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the nested indexing scheme.
    depth : int
        The depth of the HEALPix cells.
    ring : int
        The number of rings. `ring=0` returns just the input cell indices, `ring=1`
        returns the 8 (or 7) immediate neighbours, `ring=2` returns the 16 neighbours of
        the immediate neighbours, and so on.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    neighbours : numpy.ndarray of numpy.int64
        The HEALPix cell indices of the kth ring neighbours of each cell, with the same
        shape as the input plus one axis of size ``8 * ring``. If `ipix` has shape S,
        the output will be of shape ``(*S, 8 * ring)``. Missing neighbours will be
        represented by -1.

    Raises
    ------
    ValueError
        If `ipix` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If `ring` is outside its valid range :math:`[0, 2^{depth}[`

    Examples
    --------
    >>> from healpix_geo.nested import kth_neighbours
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> depth = 12
    >>> ring = 3
    >>> neighbours = kth_neighbours(ipix, depth, ring)
    >>> neighbours
    array([[ 72701297,  72701299,  72701305,  72701307,  72701393,  72701395,
             72701401,  72701404,  72701405,  72701301,  72701300,       136,
                  137,       140,       141,       135,       133,        47,
                   45,        39,        37,        36,        33,        32],
           [150994941, 150994943,  72701269,  72701271,  72701277,  72701279,
             72701301,  95070907,  95070905,  95070904,  95070893,  95070892,
             95070889,  95070888,        32,        33,        36,        37,
                   48,        49,        27,        25,        19,        17],
           [ 72701265,  72701267,  72701273,  72701275,  72701297,  72701299,
             72701305,  72701308,  72701309,  72701269,  72701268,        40,
                   41,        44,        45,        39,        37,        15,
                   13,         7,         5,         4,         1,         0]])
    """
    check_depth(depth)
    ipix = validate_cell_ids(ipix, depth)
    check_ring(depth, ring)

    return _healpix_geo_python.nested.kth_neighbours(depth, ipix, ring, num_threads)


def kth_neighbourhood(
    ipix: ArrayLikeInt, depth: int, ring: int, *, num_threads: int = 0
) -> NDArray[np.int64]:
    """Get the kth ring neighbouring cells of some HEALPix cells at a given depth.

    This method returns an array containing the :math:`(2\\cdot\\mathrm{ring}+1)^2`
    neighbours of each input cell. The "center" neighbour at index
    :math:`2\\cdot\\mathrm{ring}\\cdot(\\mathrm{ring}+1)` corresponds to the input cell
    (the 5th cell for ``ring=1`` for instance).

    This method is wrapped around the `kth_neighbourhood
    <https://docs.rs/cdshealpix/0.1.5/cdshealpix/nested/struct.Layer.html#method.kth_neighbourhood>`__
    method from the `cdshealpix Rust crate <https://crates.io/crates/cdshealpix>`__.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the nested indexing scheme.
    depth : int
        The depth of the HEALPix cells.
    ring : int
        The number of rings. `ring=0` returns just the input cell indices, `ring=1`
        returns the 8 (or 7) immediate neighbours, `ring=2` returns the 8 (or 7)
        immediate neighbours plus their immediate neighbours (a total of 24 cells), and
        so on.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    neighbours : numpy.ndarray of numpy.int64
        The HEALPix cell indices of the kth ring neighbours of each cell, with the same
        shape as the input plus one axis of size :math:`(2\\cdot\\mathrm{ring} + 1)^2`.
        If `ipix` has shape S, the output will be of shape ``(*S, (2 * ring + 1)**2)``.
        Missing neighbours will be represented by -1.

    Raises
    ------
    ValueError
        If `ipix` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If `ring` is outside its valid range :math:`[0, 2^{depth}[`

    Examples
    --------
    >>> from healpix_geo.nested import kth_neighbourhood
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> depth = 12
    >>> ring = 3
    >>> neighbours = kth_neighbourhood(ipix, depth, ring)
    >>> neighbours
    array([[       42,  72701309,  72701311,  72701397,       128,       129,
                   43,        41,        40,  72701302,  72701308,  72701310,
             72701396,  72701398,  72701399,  72701303,       130,       131,
                  134,       132,        46,        44,        38,        35,
                   34,  72701297,  72701299,  72701305,  72701307,  72701393,
             72701395,  72701401,  72701404,  72701405,  72701301,  72701300,
                  136,       137,       140,       141,       135,       133,
                   47,        45,        39,        37,        36,        33,
                   32],
           [        6,         1,         3,         9,        12,        13,
                    7,         5,         4,  95070890,  95070906,  95070895,
             95070894,  95070891,         0,         2,         8,        10,
                   11,        14,        15,        26,        24,        18,
                   16, 150994941, 150994943,  72701269,  72701271,  72701277,
             72701279,  72701301,  95070907,  95070905,  95070904,  95070893,
             95070892,  95070889,  95070888,        32,        33,        36,
                   37,        48,        49,        27,        25,        19,
                   17],
           [       10,  72701277,  72701279,  72701301,        32,        33,
                   11,         9,         8,  72701270,  72701276,  72701278,
             72701300,  72701302,  72701303,  72701271,        34,        35,
                   38,        36,        14,        12,         6,         3,
                    2,  72701265,  72701267,  72701273,  72701275,  72701297,
             72701299,  72701305,  72701308,  72701309,  72701269,  72701268,
                   40,        41,        44,        45,        39,        37,
                   15,        13,         7,         5,         4,         1,
                    0]])
    """
    check_depth(depth)
    ipix = validate_cell_ids(ipix, depth)
    check_ring(depth, ring)

    return _healpix_geo_python.nested.kth_neighbourhood(depth, ipix, ring, num_threads)


def zoom_to(
    ipix: ArrayLikeInt, depth: int, new_depth: int, *, num_threads: int = 0
) -> NDArray[np.uint64]:
    """Change the resolutions the given cell indices

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the nested indexing scheme.
    depth : int
        The depth of the HEALPix cells.
    new_depth : int
        The new depth of the HEALPix cells.

    Returns
    -------
    cells : numpy.ndarray of numpy.int64
        - If ``new_depth == depth``, the input indices.
        - If ``new_depth < depth``, the HEALPix cell indices of the parent cell of each
          input cell, with the same shape as the input.
        - If ``new_depth > depth``, the HEALpix cell indices of the children cells of
          each input cell, with the same shape as the input plus one axis of size
          :math:`4^{\\mathrm{new\\_depth}-\\mathrm{depth}}`.

    Raises
    ------
    ValueError
        If `ipix` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` or `new_depth` are outside the range [0, 29].
    """
    check_depth(depth)
    check_depth(new_depth)

    ipix = validate_cell_ids(ipix, depth)

    if depth == new_depth:
        return ipix

    return _healpix_geo_python.nested.zoom_to(depth, ipix, new_depth, num_threads)


def siblings(
    ipix: ArrayLikeInt, depth: int, *, num_threads: int = 0
) -> NDArray[np.uint64]:
    """Find the siblings for every cell

    Siblings are the 4 cells with the same parent cell. If ``depth=0``, the siblings are
    the 12 base cells.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the nested indexing scheme.
    depth : int
        The depth of the HEALPix cells.

    Returns
    -------
    cells : numpy.ndarray of numpy.uint64
        The HEALPix cell indices of the sibling for each cell, with the same shape as
        the input plus one axis of size 4 or 12. If `ipix` has shape S, the output will
        be of shape ``(*S, {4, 12})``.

    Raises
    ------
    ValueError
        If `ipix` has values outside the range :math:`[0, 12 \\times 4^{depth}[`.
    ValueError
        If `depth` is outside the range [0, 29].
    """
    check_depth(depth)
    ipix = validate_cell_ids(ipix, depth)

    return _healpix_geo_python.nested.siblings(depth, ipix, num_threads)


def angular_distances(
    from_: ArrayLikeInt, to_: ArrayLikeInt, depth: int, *, num_threads: int = 0
) -> NDArray[np.float64]:
    """Compute the angular distances between cell centers

    Parameters
    ----------
    from_ : array-like of int
        The source HEALPix cell indices. Should be 1D.
    to_ : array-like of int
        The destination HEALPix cell indices. Should be 2D.
    depth : int
        The depth of the HEALPix cells.
    num_threads : int, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    distances : numpy.ndarray
        The angular distances in radians.

    Raises
    ------
    ValueError
        If `from_` or `to_` have values outside the range :math:`[0, 12 \\times
        4^{depth}[`.
    ValueError
        If `from_` and `to_` have incompatible shapes.
    ValueError
        If `depth` is outside the range [0, 29].
    """
    check_depth(depth)

    from_ = validate_cell_ids(from_, depth)

    to_ = np.asarray(to_)
    mask = to_ != -1
    to_ = np.where(mask, to_, 0)
    to_ = validate_cell_ids(to_, depth)

    if from_.shape != to_.shape and from_.shape != to_.shape[:-1]:
        raise ValueError(
            "The shape of `from_` must be compatible with the shape of `to_`:\n"
            f"{to_.shape} or {to_.shape[:-1]} must be equal to {from_.shape}."
        )

    if from_.shape == to_.shape:
        intermediate_shape = to_.shape + (1,)
    else:
        intermediate_shape = to_.shape

    distances = _healpix_geo_python.nested.angular_distances(
        depth, from_, np.reshape(to_, intermediate_shape), num_threads
    )

    return np.where(mask, np.reshape(distances, to_.shape), np.nan)


def zone_coverage(
    bbox: tuple[float, float, float, float],
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]:
    """Search the cells covering the given bounding box

    Parameters
    ----------
    bbox : tuple of float
        The 2D bounding box to rasterize in order: ``[lon_min, lat_min, lon_max,
        lat_max]``, in degrees.
    depth : int
        The maximum depth of the cells to be returned.
    ellipsoid : ellipsoid-like, default: "sphere"
        The reference ellipsoid to evaluate healpix on.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray of numpy.uint64
        The HEALPix cell indices covered at least partially by the bounding box, in the
        nested indexing scheme.
    levels : numpy.ndarray of numpy.uint8
        The refinement levels of the cell ids. If `flat` is True, these will all have
        the same value.
    fully_covered : numpy.ndarray of numpy.bool
        Boolean array marking whether the cells are fully covered by the bounding box.

    Raises
    ------
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If `bbox` is not of length 4.
    """
    check_depth(depth)
    bbox = validate_tuple4(bbox, "bbox")

    return _healpix_geo_python.nested.zone_coverage(
        depth, bbox, ellipsoid=ellipsoid, flat=flat
    )


def box_coverage(
    center: tuple[float, float] | Array1D[np.floating],
    size: tuple[float, float] | Array1D[np.floating],
    angle: float,
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]:
    """Search the cells covering the given box.

    Parameters
    ----------
    center : tuple of float or numpy.ndarray of float
        The center of the box, in degrees, either as a 2-sized array or as a 2-tuple of
        float.
    size : tuple of float or numpy.ndarray of float
        The size of the box, in degrees, either as a 2-sized array of as a 2-tuple of
        float.
    angle : float
        The angle by which the box is rotated, in degrees.
    depth : int
        The maximum depth of the cells to be returned.
    ellipsoid : ellipsoid-like, default: "sphere"
        The reference ellipsoid to evaluate healpix on.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray of numpy.uint64
        The HEALPix cell indices covered at least partially by the bounding box, in the
        nested indexing scheme.
    levels : numpy.ndarray of numpy.uint8
        The refinement levels of the cell ids. If `flat` is True, these will all have
        the same value.
    fully_covered : numpy.ndarray of numpy.bool
        Boolean array marking whether the cells are fully covered by the bounding box.

    Raises
    ------
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If `center` and `size` are not one-dimensional of length 2.
    """
    check_depth(depth)

    center = validate_tuple2(center, "center")
    size = validate_tuple2(size, "size")

    return _healpix_geo_python.nested.box_coverage(
        depth, center, size, angle, ellipsoid=ellipsoid, flat=flat
    )


def polygon_coverage(
    vertices: ArrayLikeFloat,
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]:
    """Search the cells covering the given polygon.

    Parameters
    ----------
    vertices : array-like of float
        The vertices of the polygon without holes. Must be of shape ``(n, 2)``.
    depth : int
        The maximum depth of the cells to be returned.
    ellipsoid : ellipsoid-like, default: "sphere"
        The reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.polygon_search`.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray of numpy.uint64
        The HEALPix cell indices covered at least partially by the polygon, in the
        nested indexing scheme.
    levels : numpy.ndarray of numpy.uint8
        The refinement levels of the cell ids. If `flat` is True, these will all have
        the same value.
    fully_covered : numpy.ndarray of numpy.bool
        Boolean array marking whether the cells are fully covered by the polygon.

    Raises
    ------
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If `vertices` is not of shape (N, 2).
    """
    check_depth(depth)

    vertices = np.asarray(vertices, dtype=np.float64)
    if vertices.ndim != 2 or vertices.shape[1] != 2:
        raise IndexError("Vertices should be of shape (N, 2).")

    return _healpix_geo_python.nested.polygon_coverage(
        depth, vertices, ellipsoid=ellipsoid, flat=flat
    )


def cone_coverage(
    center: tuple[float, float] | Array1D[np.floating],
    radius: float,
    depth: int,
    *,
    delta_depth: int = 0,
    ellipsoid: EllipsoidLike = "sphere",
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]:
    """Search the cells covering the given cone

    Cone in this case means a circle on the surface of the reference ellipsoid.

    Parameters
    ----------
    center : tuple of float or numpy.ndarray of float
        The center of the cone, in degrees, either as a 2-sized array or as a 2-tuple of
        float.
    radius : float
        The radius of the cone, in degrees.
    depth : int
        The maximum depth of the cells to be returned.
    ellipsoid : ellipsoid-like, default: "sphere"
        The reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.cone_search`.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray of numpy.uint64
        The HEALPix cell indices covered at least partially by the cone, in the
        nested indexing scheme.
    levels : numpy.ndarray of numpy.uint8
        The refinement levels of the cell ids. If `flat` is True, these will all have
        the same value.
    fully_covered : numpy.ndarray of numpy.bool
        Boolean array marking whether the cells are fully covered by the cone.

    Raises
    ------
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If `center` is not one-dimensional of length 2.
    """
    check_depth(depth)

    center = validate_tuple2(center, "center")

    return _healpix_geo_python.nested.cone_coverage(
        depth,
        center,
        radius,
        delta_depth=delta_depth,
        ellipsoid=ellipsoid,
        flat=flat,
    )


def elliptical_cone_coverage(
    center: tuple[float, float] | Array1D[np.floating],
    ellipse_geometry: tuple[float, float] | Array1D[np.floating],
    position_angle: float,
    depth: int,
    *,
    delta_depth: int = 0,
    ellipsoid: EllipsoidLike = "sphere",
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]:
    """Search the cells covering the given elliptical cone.

    Elliptical cone in this case refers to an ellipse on the surface of the reference ellipsoid.

    Parameters
    ----------
    center : tuple of float or numpy.ndarray of float
        The center of the cone, in degrees, either as a 2-sized array or as a 2-tuple of
        float.
    ellipse_geometry : tuple of float or numpy.ndarray of float
        The semimajor and semimajor axis, in degrees, as a 2-sized array or as a 2-tuple
        of float.
    position_angle : float
        The orientation of the ellipse.
    depth : int
        The maximum depth of the cells to be returned.
    ellipsoid : ellipsoid-like, default: "sphere"
        The reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.elliptical_cone_search`.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray of numpy.uint64
        The HEALPix cell indices covered at least partially by the ellipse, in the
        nested indexing scheme.
    levels : numpy.ndarray of numpy.uint8
        The refinement levels of the cell ids. If `flat` is True, these will all have
        the same value.
    fully_covered : numpy.ndarray of numpy.bool
        Boolean array marking whether the cells are fully covered by the ellipse.

    Raises
    ------
    ValueError
        If `depth` is outside the range [0, 29].
    ValueError
        If `center` and `ellipse_geometry` are not one-dimensional of length 2.
    """
    check_depth(depth)

    center = validate_tuple2(center, "center")
    ellipse_geometry = validate_tuple2(ellipse_geometry, "ellipse_geometry")

    return _healpix_geo_python.nested.elliptical_cone_coverage(
        depth,
        center,
        ellipse_geometry,
        position_angle,
        delta_depth=delta_depth,
        ellipsoid=ellipsoid,
        flat=flat,
    )
