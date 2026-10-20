from __future__ import annotations

from typing import TYPE_CHECKING

import marray
import numpy as np

from healpix_geo import _healpix_geo_python
from healpix_geo.utils import (
    check_depth,
    check_same_shape,
    validate_cell_ids,
    validate_tuple2,
    validate_tuple4,
)

if TYPE_CHECKING:
    from typing import Any, Literal

    from numpy.typing import NDArray

    from healpix_geo.typing import Array1D, ArrayLikeFloat, ArrayLikeInt, EllipsoidLike


def to_nested(
    ipix: ArrayLikeInt, *, num_threads: int = 0
) -> tuple[NDArray[np.uint64], NDArray[np.uint8]]:
    """Convert from zuniq to nested

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the zuniq scheme.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    nested : numpy.ndarray of numpy.uint64
        The HEALPix cell indices in the nested scheme, with the same shape as the
        inputs.
    depth : numpy.ndarray of numpy.uint8
        The depth of the HEALPix cells for every cell.

    Raises
    ------
    ValueError
        If `ipix` has negative values.

    Examples
    --------
    >>> import healpix_geo.zuniq
    >>> import numpy as np
    >>> ipix_zuniq = np.array(
    ...     [
    ...         4683743612465315840,
    ...         1130403506469994496,
    ...         1639310264362860544,
    ...         206039682952200192,
    ...     ],
    ...     dtype=np.uint64,
    ... )
    >>> ipix_nested, depth = healpix_geo.zuniq.to_nested(ipix_zuniq)
    >>> ipix_nested
    array([ 32, 125,  45,  91], dtype=uint64)
    >>> depth
    array([1, 3, 2, 4], dtype=uint8)
    """
    ipix = validate_cell_ids(ipix)

    return _healpix_geo_python.zuniq.to_nested(ipix, num_threads)


def to_ring(
    ipix: ArrayLikeInt, *, num_threads: int = 0
) -> tuple[NDArray[np.uint64], NDArray[np.uint8]]:
    """Convert from zuniq to ring

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the zuniq scheme.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    ring : numpy.ndarray of numpy.uint64
        The HEALPix cell indices in the ring scheme, with the same shape as the inputs.
    depth : numpy.ndarray of numpy.uint8
        The depth of the HEALPix cells for every cell.

    Raises
    ------
    ValueError
        If `ipix` has negative values.

    Examples
    --------
    >>> import healpix_geo.zuniq
    >>> import numpy as np
    >>> ipix_zuniq = np.array(
    ...     [
    ...         4683743612465315840,
    ...         1130403506469994496,
    ...         1639310264362860544,
    ...         206039682952200192,
    ...     ],
    ...     dtype=np.uint64,
    ... )
    >>> ipix_ring, depth = healpix_geo.zuniq.to_ring(ipix_zuniq)
    >>> ipix_ring
    array([ 44,   7,   9, 432], dtype=uint64)
    >>> depth
    array([1, 3, 2, 4], dtype=uint8)
    """
    ipix = validate_cell_ids(ipix)

    return _healpix_geo_python.zuniq.to_ring(ipix, num_threads)


def healpix_to_lonlat(
    ipix: ArrayLikeInt, *, ellipsoid: EllipsoidLike = "sphere", num_threads: int = 0
) -> tuple[NDArray[np.float64], NDArray[np.float64]]:
    """Get the longitudes and latitudes of the center of some HEALPix cells.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the zuniq indexing scheme.
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
        If the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo.zuniq import healpix_to_lonlat
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> lon, lat = healpix_to_lonlat(ipix, ellipsoid="WGS84")
    >>> lon
    array([44.99999975, 45.00000008, 44.99999992])
    >>> lat
    array([2.85869025e-07, 1.42934512e-07, 1.42934512e-07])
    """
    ipix = validate_cell_ids(ipix)

    return _healpix_geo_python.zuniq.healpix_to_lonlat(ipix, ellipsoid, num_threads)


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
    lon : array-like of float
        The longitudes of the input points, in degrees.
    lat : array-like of float
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
        The corresponding HEALPix cell indices in the zuniq indexing scheme, with the
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
    >>> from healpix_geo.zuniq import lonlat_to_healpix
    >>> import numpy as np
    >>> lon = np.array([0, 50, 25], dtype=np.float64)
    >>> lat = np.array([6, -12, 45], dtype=np.float64)
    >>> depth = 3
    >>> ipix = lonlat_to_healpix(lon, lat, depth, ellipsoid="WGS84")
    >>> ipix
    array([2742692173068632064, 5165628772593958912,  346777171307528192],
          dtype=uint64)
    """
    check_depth(depth)
    longitude = np.atleast_1d(longitude).astype(np.float64)
    latitude = np.atleast_1d(latitude).astype(np.float64)

    check_same_shape(longitude=longitude, latitude=latitude)

    return _healpix_geo_python.zuniq.lonlat_to_healpix(
        depth, longitude, latitude, ellipsoid, num_threads
    )


def healpix_to_cartesian(
    ipix: ArrayLikeInt, *, ellipsoid: EllipsoidLike = "sphere", num_threads: int = 0
) -> tuple[NDArray[np.float64], NDArray[np.float64], NDArray[np.float64]]:
    """Get the cartesian coordinates of the center of the given HEALPix cells.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the zuniq indexing scheme.
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
        If `ipix` has negative values.
    ValueError
        If the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo.zuniq import healpix_to_cartesian
    >>> import numpy as np
    >>> ipix = np.array([382805968326492160, 58546795155816448, 94575592174780416])
    >>> x, y, z = healpix_to_cartesian(ipix, ellipsoid="WGS84")
    >>> x
    array([4728734.69011096, 3814362.85063174, 5302653.40426395])
    >>> y
    array([ 465739.71573273, 4647814.58136658, 2834327.29466645])
    >>> z
    array([4240471.60205904, 2121029.89621885, 2121029.89621885])
    """
    ipix = validate_cell_ids(ipix)

    return _healpix_geo_python.zuniq.healpix_to_cartesian(ipix, ellipsoid, num_threads)


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
        The corresponding HEALPix cell indices in the zuniq indexing scheme, with the
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
    >>> from healpix_geo.zuniq import cartesian_to_healpix
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
    array([2742692173068632064, 5165628772593958912,  346777171307528192],
          dtype=uint64)
    """
    check_depth(depth)
    x = np.atleast_1d(x).astype(np.float64)
    y = np.atleast_1d(y).astype(np.float64)
    z = np.atleast_1d(z).astype(np.float64)

    check_same_shape(x=x, y=y, z=z)

    return _healpix_geo_python.zuniq.cartesian_to_healpix(
        depth, x, y, z, ellipsoid, num_threads
    )


def vertices(
    ipix: ArrayLikeInt,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    step: int = 1,
    num_threads: int = 0,
) -> tuple[NDArray[np.float64], NDArray[np.float64]]:
    """Get the longitudes and latitudes of the vertices of some HEALPix cells in zuniq encoding.

    This method returns the 4 vertices of each cell in `ipix`.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the zuniq indexing scheme.
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
        If `ipix` has negative values.
    ValueError
        If the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo.zuniq import vertices
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> depth = 12
    >>> lon, lat = vertices(ipix, ellipsoid="sphere")
    >>> np.stack([lon, lat], axis=-1)
    array([[[4.49999997e+01, 2.13443412e-07],
            [4.49999998e+01, 2.84591215e-07],
            [4.49999997e+01, 3.55739019e-07],
            [4.49999997e+01, 2.84591215e-07]],
    <BLANKLINE>
           [[4.50000001e+01, 7.11478039e-08],
            [4.50000002e+01, 1.42295608e-07],
            [4.50000001e+01, 2.13443412e-07],
            [4.50000000e+01, 1.42295608e-07]],
    <BLANKLINE>
           [[4.49999999e+01, 7.11478039e-08],
            [4.50000000e+01, 1.42295608e-07],
            [4.49999999e+01, 2.13443412e-07],
            [4.49999998e+01, 1.42295608e-07]]])
    """
    ipix = validate_cell_ids(ipix)

    return _healpix_geo_python.zuniq.vertices(ipix, ellipsoid, step, num_threads)


def vertex_indices(
    ipix: NDArray[np.uint64],
    *,
    num_threads: int = 0,
) -> NDArray[np.uint64]:
    """Get the indices of the 4 vertices of the given cells.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the zuniq indexing scheme.
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
        If `ipix` has negative values.

    See Also
    --------
    healpix_geo.vertex_to_lonlat

    Examples
    --------
    >>> from healpix_geo.zuniq import vertex_indices
    >>> import numpy as np

    >>> ipix = np.array([42, 6, 10])

    >>> vertex_ids = vertex_indices(ipix)
    >>> vertex_ids
    array([[1729382249662513151, 1729382247515029504, 1729382245367545855,
            1729382247515029503],
           [1729382253957480449, 1729382251809996802, 1729382249662513153,
            1729382251809996801],
           [1729382253957480448, 1729382251809996801, 1729382249662513152,
            1729382251809996800]], dtype=uint64)
    """
    ipix = validate_cell_ids(ipix)

    return _healpix_geo_python.zuniq.vertex_indices(ipix, num_threads)


def bilinear_interpolation(
    longitude: ArrayLikeFloat,
    latitude: ArrayLikeFloat,
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    num_threads: int = 0,
) -> tuple[Any, Any]:
    """Get the cell ids and weights necessary to bilinearly interpolate the given values.

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
        The indices of the HEALPix cells around the given points, in the zuniq indexing
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
    >>> from healpix_geo.zuniq import bilinear_interpolation
    >>> import numpy as np

    Define coordinates

    >>> lon = np.array([-15.0, -10.0, -5.0, 0.0, 5.0])
    >>> lat = np.array([30.0, 35.0, 40.0, 45.0, 50.0])

    Compute interpolation weights

    >>> cell_ids, weights = bilinear_interpolation(lon, lat, depth=6, ellipsoid="WGS84")
    >>> cell_ids
    MArray(
        array([[1885952712705572864, 1886093450193928192, 1886234187682283520,
                1886374925170638848],
               [1911426198097887232, 1912974310469795840, 1911707673074597888,
                1913255785446506496],
               [1919448234934140928, 1919588972422496256, 1920292659864272896,
                1920433397352628224],
               [ 384987399395999744,  386535511767908352, 1922966672143024128,
                1926062896886841344],
               [ 394135336139096064,  394276073627451392,  394416811115806720,
                 394557548604162048]], dtype=uint64),
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

    ipix, weights = _healpix_geo_python.zuniq.bilinear_interpolation(
        depth, longitude, latitude, ellipsoid, num_threads
    )

    xp = marray.masked_namespace(np)
    mask = weights == 0

    return xp.asarray(ipix, mask=mask), xp.asarray(weights, mask=mask)


def neighbours(
    ipix: ArrayLikeInt,
    *,
    connectivity: Literal["edge", "vertex", "all"] = "all",
    num_threads: int = 0,
) -> NDArray[np.int64]:
    """Get the direction-preserving immediate neighbours of some HEALPix cells at a given depth.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the zuniq indexing scheme.
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
        If `ipix` has negative values.
    """
    ipix = validate_cell_ids(ipix)

    return _healpix_geo_python.zuniq.neighbours(ipix, connectivity, num_threads)


def kth_neighbours(
    ipix: ArrayLikeInt, ring: int, *, num_threads: int = 0
) -> NDArray[np.int64]:
    """Get the kth ring of neighbouring cells around some HEALPix cells at a given depth.

    This method returns an array containing the ``8 * ring`` neighbours of each input
    cell. This method is wrapped around the `kth_neighbours
    <https://docs.rs/cdshealpix/0.9.1/cdshealpix/nested/struct.Layer.html#method.kth_neighbours>`__
    method of the `cdshealpix Rust crate <https://crates.io/crates/cdshealpix>`__.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices in the zuniq indexing scheme.
    ring : int
        The number of rings. `ring=0` returns just the input cell ids, `ring=1` returns
        the 8 (or 7) immediate neighbours, `ring=2` returns the 16 neighbours of the
        immediate neighbours, and so on.
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
        If `ipix` has negative values.

    Examples
    --------
    >>> from healpix_geo.zuniq import kth_neighbours
    >>> import numpy as np
    >>> ipix = np.array([1460288880640, 223338299392, 360777252864], dtype=np.uint64)
    >>> ring = 3
    >>> neighbours = kth_neighbours(ipix, ring)
    >>> neighbours
    array([[2497997561114132480, 2497997629833609216, 2497997835992039424,
            2497997904711516160, 2498000859649015808, 2498000928368492544,
            2498001134526922752, 2498001237606137856, 2498001271965876224,
            2497997698553085952, 2497997664193347584,       4690104287232,
                  4724464025600,       4827543240704,       4861902979072,
                  4655744548864,       4587025072128,       1632087572480,
                  1563368095744,       1357209665536,       1288490188800,
                  1254130450432,       1151051235328,       1116691496960],
           [5188146684831465472, 5188146753550942208, 2497996599041458176,
            2497996667760934912, 2497996873919365120, 2497996942638841856,
            2497997698553085952, 3266611508108328960, 3266611439388852224,
            3266611405029113856, 3266611027071991808, 3266610992712253440,
            3266610889633038336, 3266610855273299968,       1116691496960,
                  1151051235328,       1254130450432,       1288490188800,
                  1666447310848,       1700807049216,        944892805120,
                   876173328384,        670014898176,        601295421440],
           [2497996461602504704, 2497996530321981440, 2497996736480411648,
            2497996805199888384, 2497997561114132480, 2497997629833609216,
            2497997835992039424, 2497997939071254528, 2497997973430992896,
            2497996599041458176, 2497996564681719808,       1391569403904,
                  1425929142272,       1529008357376,       1563368095744,
                  1357209665536,       1288490188800,        532575944704,
                   463856467968,        257698037760,        188978561024,
                   154618822656,         51539607552,         17179869184]])
    """
    ipix = validate_cell_ids(ipix)

    return _healpix_geo_python.zuniq.kth_neighbours(ipix, ring, num_threads)


def kth_neighbourhood(
    ipix: ArrayLikeInt, ring: int, *, num_threads: int = 0
) -> NDArray[np.int64]:
    """Get the kth ring neighbouring cells of some HEALPix cells.

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
        The HEALPix cell indices in the zuniq indexing scheme.
    ring : int
        The number of rings. `ring=0` returns just the input cell ids, `ring=1` returns
        the 8 (or 7) immediate neighbours, `ring=2` returns the 8 (or 7) immediate
        neighbours plus their immediate neighbours (a total of 24 cells), and so on.
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
        If `ipix` has negative values.

    Examples
    --------
    >>> from healpix_geo.zuniq import kth_neighbourhood
    >>> import numpy as np
    >>> ipix = np.array([1460288880640, 223338299392, 360777252864], dtype=np.uint64)
    >>> ring = 3
    >>> neighbours = kth_neighbourhood(ipix, ring)
    >>> neighbours
    array([[      1460288880640, 2497997973430992896, 2497998042150469632,
            2498000997087969280,       4415226380288,       4449586118656,
                  1494648619008,       1425929142272,       1391569403904,
            2497997732912824320, 2497997939071254528, 2497998007790731264,
            2498000962728230912, 2498001031447707648, 2498001065807446016,
            2497997767272562688,       4483945857024,       4518305595392,
                  4621384810496,       4552665333760,       1597727834112,
                  1529008357376,       1322849927168,       1219770712064,
                  1185410973696, 2497997561114132480, 2497997629833609216,
            2497997835992039424, 2497997904711516160, 2498000859649015808,
            2498000928368492544, 2498001134526922752, 2498001237606137856,
            2498001271965876224, 2497997698553085952, 2497997664193347584,
                  4690104287232,       4724464025600,       4827543240704,
                  4861902979072,       4655744548864,       4587025072128,
                  1632087572480,       1563368095744,       1357209665536,
                  1288490188800,       1254130450432,       1151051235328,
                  1116691496960],
           [       223338299392,         51539607552,        120259084288,
                   326417514496,        429496729600,        463856467968,
                   257698037760,        188978561024,        154618822656,
            3266610923992776704, 3266611473748590592, 3266611095791468544,
            3266611061431730176, 3266610958352515072,         17179869184,
                    85899345920,        292057776128,        360777252864,
                   395136991232,        498216206336,        532575944704,
                   910533066752,        841813590016,        635655159808,
                   566935683072, 5188146684831465472, 5188146753550942208,
            2497996599041458176, 2497996667760934912, 2497996873919365120,
            2497996942638841856, 2497997698553085952, 3266611508108328960,
            3266611439388852224, 3266611405029113856, 3266611027071991808,
            3266610992712253440, 3266610889633038336, 3266610855273299968,
                  1116691496960,       1151051235328,       1254130450432,
                  1288490188800,       1666447310848,       1700807049216,
                   944892805120,        876173328384,        670014898176,
                   601295421440],
           [       360777252864, 2497996873919365120, 2497996942638841856,
            2497997698553085952,       1116691496960,       1151051235328,
                   395136991232,        326417514496,        292057776128,
            2497996633401196544, 2497996839559626752, 2497996908279103488,
            2497997664193347584, 2497997732912824320, 2497997767272562688,
            2497996667760934912,       1185410973696,       1219770712064,
                  1322849927168,       1254130450432,        498216206336,
                   429496729600,        223338299392,        120259084288,
                    85899345920, 2497996461602504704, 2497996530321981440,
            2497996736480411648, 2497996805199888384, 2497997561114132480,
            2497997629833609216, 2497997835992039424, 2497997939071254528,
            2497997973430992896, 2497996599041458176, 2497996564681719808,
                  1391569403904,       1425929142272,       1529008357376,
                  1563368095744,       1357209665536,       1288490188800,
                   532575944704,        463856467968,        257698037760,
                   188978561024,        154618822656,         51539607552,
                    17179869184]])
    """
    ipix = validate_cell_ids(ipix)

    return _healpix_geo_python.zuniq.kth_neighbourhood(ipix, ring, num_threads)


def zone_coverage(
    bbox: tuple[float, float, float, float],
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.bool]]:
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
        zuniq indexing scheme.
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

    return _healpix_geo_python.zuniq.zone_coverage(
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
) -> tuple[Array1D[np.uint64], Array1D[np.bool]]:
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
        zuniq indexing scheme.
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

    return _healpix_geo_python.zuniq.box_coverage(
        depth, center, size, angle, ellipsoid=ellipsoid, flat=flat
    )


def polygon_coverage(
    vertices: ArrayLikeFloat,
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    flat: bool = True,
) -> tuple[Array1D[np.uint64], Array1D[np.bool]]:
    """Search the cells covering the given polygon.

    Parameters
    ----------
    vertices : array-like of float
        The vertices of the polygon without holes. Must be of shape ``(N, 2)``.
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
        zuniq indexing scheme.
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

    return _healpix_geo_python.zuniq.polygon_coverage(
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
) -> tuple[Array1D[np.uint64], Array1D[np.bool]]:
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
        zuniq indexing scheme.
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

    return _healpix_geo_python.zuniq.cone_coverage(
        depth, center, radius, delta_depth=delta_depth, ellipsoid=ellipsoid, flat=flat
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
) -> tuple[Array1D[np.uint64], Array1D[np.bool]]:
    """Search the cells covering the given elliptical cone.

    Elliptical cone in this case refers to an ellipse on the surface of the reference
    ellipsoid.

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
        zuniq indexing scheme.
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

    return _healpix_geo_python.zuniq.elliptical_cone_coverage(
        depth,
        center,
        ellipse_geometry,
        position_angle,
        delta_depth=delta_depth,
        ellipsoid=ellipsoid,
        flat=flat,
    )
