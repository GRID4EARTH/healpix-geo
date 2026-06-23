import numpy as np

from healpix_geo import geometry, healpix_geo, nested, ring, zuniq
from healpix_geo.geometry import Bbox


def cartesian_to_lonlat(x, y, z, ellipsoid="sphere", num_threads=0):
    """Convert cartesian coordinates to geographic coordinates on the surface of the ellipsoid.

    This drops the height coordinate, resulting in a lossy conversion for coordinates that are not on the surface.

    Parameters
    ----------
    x, y, z : array-like
        The cartesian coordinates in meters. All arrays must have the exactly same shape (no broadcasting).
    ellipsoid : ellipsoid-like, default: "sphere"
        The ellipsoid to evaluate the coordinates on.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    lon, lat : array-like
        The equivalent geographic coordinates.

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

    x = np.atleast_1d(x).astype("float64")
    y = np.atleast_1d(y).astype("float64")
    z = np.atleast_1d(z).astype("float64")

    num_threads = np.uint16(num_threads)

    return healpix_geo.cartesian_to_lonlat(x, y, z, ellipsoid, num_threads)


def lonlat_to_cartesian(longitude, latitude, ellipsoid="sphere", num_threads=0):
    """Convert geographic coordinates to cartesian coordinates.

    Parameters
    ----------
    longitude, latitude : array-like
        The geographic coordinates in degrees. All arrays must have the exactly same shape (no broadcasting).
    ellipsoid : ellipsoid-like, default: "sphere"
        The ellipsoid to evaluate the coordinates on.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    x, y, z : array-like
        The equivalent cartesian coordinates.

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
    longitude = np.atleast_1d(longitude).astype("float64")
    latitude = np.atleast_1d(latitude).astype("float64")

    num_threads = np.uint16(num_threads)

    return healpix_geo.lonlat_to_cartesian(longitude, latitude, ellipsoid, num_threads)


__all__ = [
    "nested",
    "ring",
    "zuniq",
    "slices",
    "geometry",
    "Bbox",
]
