import numpy as np
from numpy.typing import NDArray

import healpix_geo.ellipsoid  # noqa: F401
from healpix_geo import _healpix_geo_python, geometry, nested, ring, zuniq
from healpix_geo._healpix_geo_python import RaggedArray
from healpix_geo.geometry import Bbox
from healpix_geo.mesh import vertex_to_lonlat
from healpix_geo.typing import ArrayLikeFloat, EllipsoidLike
from healpix_geo.validation import check_same_shape


def cartesian_to_lonlat(
    x: ArrayLikeFloat,
    y: ArrayLikeFloat,
    z: ArrayLikeFloat,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    num_threads: int = 0,
) -> tuple[NDArray[np.float64], NDArray[np.float64]]:
    """Convert cartesian coordinates to geographic coordinates on the surface of the ellipsoid.

    This drops the height coordinate, resulting in a lossy conversion for coordinates
    that are not on the surface.

    Parameters
    ----------
    x, y, z : array-like of float
        The cartesian coordinates in meters. All arrays must have the exactly same shape
        (no broadcasting).
    ellipsoid : ellipsoid-like, default: "sphere"
        The ellipsoid to evaluate the coordinates on.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    lon, lat : numpy.ndarray of numpy.float64
        The equivalent geographic coordinates, with the same shape as the inputs.

    Raises
    ------
    ValueError
        If `x`, `y` and `z` do not have the same shape.
    ValueError
        If the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo import cartesian_to_lonlat
    >>> import numpy as np
    >>> x = np.array([4728734.69011096, 3814362.85063174, 5302653.40426395])
    >>> y = np.array([465739.71573273, 4647814.58136658, 2834327.29466645])
    >>> z = np.array([4240471.60205904, 2121029.89621885, 2121029.89621885])

    >>> lon, lat = cartesian_to_lonlat(x, y, z, ellipsoid="WGS84")
    >>> lon
    array([ 5.625, 50.625, 28.125])
    >>> lat
    array([41.93785391, 19.55202227, 19.55202227])
    """
    x = np.atleast_1d(x).astype(np.float64)
    y = np.atleast_1d(y).astype(np.float64)
    z = np.atleast_1d(z).astype(np.float64)

    check_same_shape(x=x, y=y, z=z)

    return _healpix_geo_python.cartesian_to_lonlat(x, y, z, ellipsoid, num_threads)


def lonlat_to_cartesian(
    longitude: ArrayLikeFloat,
    latitude: ArrayLikeFloat,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    num_threads: int = 0,
) -> tuple[NDArray[np.float64], NDArray[np.float64], NDArray[np.float64]]:
    """Convert geographic coordinates to cartesian coordinates.

    Parameters
    ----------
    longitude, latitude : array-like of float
        The geographic coordinates in degrees. All arrays must have the exactly same
        shape (no broadcasting).
    ellipsoid : ellipsoid-like, default: "sphere"
        The ellipsoid to evaluate the coordinates on.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    x, y, z : numpy.ndarray of numpy.float64
        The equivalent cartesian coordinates, with the same shape as the inputs.

    Raises
    ------
    ValueError
        If `longitude` and `latitude` do not have the same shape.
    ValueError
        If the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo import lonlat_to_cartesian
    >>> import numpy as np
    >>> lon = np.array([5.625, 50.625, 28.125])
    >>> lat = np.array([41.93785391, 19.55202227, 19.55202227])

    >>> x, y, z = lonlat_to_cartesian(lon, lat, ellipsoid="WGS84")
    >>> x
    array([4728734.69012279, 3814362.85054704, 5302653.4041462 ])
    >>> y
    array([ 465739.7157339 , 4647814.58126337, 2834327.29460352])
    >>> z
    array([4240471.60204581, 2121029.8965948 , 2121029.8965948 ])
    """
    longitude = np.atleast_1d(longitude).astype(np.float64)
    latitude = np.atleast_1d(latitude).astype(np.float64)

    check_same_shape(longitude=longitude, latitude=latitude)

    return _healpix_geo_python.lonlat_to_cartesian(
        longitude, latitude, ellipsoid, num_threads
    )


__all__ = [
    "nested",
    "ring",
    "zuniq",
    "geometry",
    "Bbox",
    "RaggedArray",
    "vertex_to_lonlat",
    "lonlat_to_cartesian",
    "cartesian_to_lonlat",
]
