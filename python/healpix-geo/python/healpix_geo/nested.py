import marray
import numpy as np

from healpix_geo import healpix_geo
from healpix_geo.utils import _check_depth, _check_ipixels, _check_ring

RangeMOCIndex = healpix_geo.nested.RangeMOCIndex
internal_boundary = healpix_geo.nested.internal_boundary


def create_empty(depth):
    return RangeMOCIndex.empty(depth)


def healpix_to_lonlat(ipix, depth, ellipsoid="sphere", num_threads=0):
    r"""Get the longitudes and latitudes of the center of some HEALPix cells.

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes given as a `np.uint64` numpy array.
    depth : `numpy.ndarray`
        The HEALPix cell depth given as a `np.uint8` numpy array.
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.healpix_to_lonlat`.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    lon, lat : array-like
        The coordinates of the center of the HEALPix cells given as a longitude, latitude tuple.

    Raises
    ------
    ValueError
        When the HEALPix cell indexes given have values out of :math:`[0, 4^{29 - depth}[`.
    ValueError
        When the name of the ellipsoid is unknown.

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
    _check_depth(depth)
    ipix = np.atleast_1d(ipix)
    _check_ipixels(data=ipix, depth=depth)
    ipix = ipix.astype(np.uint64)

    num_threads = np.uint16(num_threads)

    return healpix_geo.nested.healpix_to_lonlat(depth, ipix, ellipsoid, num_threads)


def lonlat_to_healpix(longitude, latitude, depth, ellipsoid="sphere", num_threads=0):
    r"""Get the HEALPix indexes that contains specific points.

    Parameters
    ----------
    longitude : array-like
        The longitudes of the input points, in degrees.
    latitude : array-like
        The latitudes of the input points, in degrees.
    depth : int or array-like of int
        The HEALPix cell depth given as a `np.uint8` numpy array.
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.lonlat_to_healpix`.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    ipix : `numpy.ndarray`
        A numpy array containing all the HEALPix cell indexes stored as `np.uint64`.

    Raises
    ------
    ValueError
        When the number of longitudes and latitudes given do not match.
    ValueError
        When the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo.nested import lonlat_to_healpix
    >>> import numpy as np
    >>> lon = np.array([0, 50, 25], dtype="float64")
    >>> lat = np.array([6, -12, 45], dtype="float64")
    >>> depth = 3
    >>> ipix = lonlat_to_healpix(lon, lat, depth, ellipsoid="WGS84")
    >>> ipix
    array([304, 573,  38], dtype=uint64)
    """
    _check_depth(depth)
    longitude = np.atleast_1d(longitude).astype("float64")
    latitude = np.atleast_1d(latitude).astype("float64")

    num_threads = np.uint16(num_threads)

    return healpix_geo.nested.lonlat_to_healpix(
        depth,
        longitude,
        latitude,
        ellipsoid,
        num_threads,
    )


def healpix_to_cartesian(ipix, depth, ellipsoid="sphere", num_threads=0):
    r"""Get the cartesian coordinates of the center of the given HEALPix cells.

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes given as a `np.uint64` numpy array.
    depth : `numpy.ndarray`
        The HEALPix cell depth given as a `np.uint8` numpy array.
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.healpix_to_lonlat`.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    x, y, z : array-like
        The coordinates of the center of the HEALPix cells.

    Raises
    ------
    ValueError
        When the HEALPix cell indexes given have values out of :math:`[0, 4^{29 - depth}[`.
    ValueError
        When the name of the ellipsoid is unknown.

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
    _check_depth(depth)
    ipix = np.atleast_1d(ipix)
    _check_ipixels(data=ipix, depth=depth)
    ipix = ipix.astype(np.uint64)

    num_threads = np.uint16(num_threads)

    return healpix_geo.nested.healpix_to_cartesian(depth, ipix, ellipsoid, num_threads)


def cartesian_to_healpix(x, y, z, depth, ellipsoid="sphere", num_threads=0):
    r"""Get the HEALPix indexes that contain specific points.

    Parameters
    ----------
    x : array-like
        The x coordinate of the input points, in meters.
    y : array-like
        The y coordinate of the input points, in meters.
    z : array-like
        The z coordinate of the input points, in meters.
    depth : int or array-like of int
        The HEALPix cell depth given as a `np.uint8` numpy array.
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.lonlat_to_healpix`.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    ipix : `numpy.ndarray`
        A numpy array containing all the HEALPix cell indexes stored as `np.uint64`.

    Raises
    ------
    ValueError
        When the number of longitudes and latitudes given do not match.
    ValueError
        When the name of the ellipsoid is unknown.

    Examples
    --------
    >>> from healpix_geo.nested import cartesian_to_healpix
    >>> import numpy as np
    >>> x = np.array(
    ...     [6343428.894701699, 4010777.6054728953, 4094327.79214653], dtype="float64"
    ... )
    >>> y = np.array([0.0, 4779858.620418726, 1909216.4044747741], dtype="float64")
    >>> z = np.array(
    ...     [662257.957592079, -1317402.5312295998, 4487348.408865919], dtype="float64"
    ... )
    >>> depth = 3
    >>> ipix = cartesian_to_healpix(x, y, z, depth, ellipsoid="WGS84")
    >>> ipix
    array([304, 573,  38], dtype=uint64)
    """
    _check_depth(depth)
    x = np.atleast_1d(x).astype("float64")
    y = np.atleast_1d(y).astype("float64")
    z = np.atleast_1d(z).astype("float64")

    num_threads = np.uint16(num_threads)

    return healpix_geo.nested.cartesian_to_healpix(
        depth, x, y, z, ellipsoid, num_threads
    )


def vertices(ipix, depth, ellipsoid="sphere", step=1, num_threads=0):
    """Get the longitudes and latitudes of the vertices of some HEALPix cells at a given depth.

    This method returns the 4 vertices of each cell in `ipix`.

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes given as a `np.uint64` numpy array.
    depth : int, or `numpy.ndarray`
        The depth of the HEALPix cells. If given as an array, should have the same shape than ipix
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.vertices`.
    step : int, default: 1
        The number of vertices returned per HEALPix side. By default it is set to 1 meaning that
        it will only return the vertices of the cell. 2 means that it will return the vertices of
        the cell plus one more vertex per edge (the center of the edge). More generally, the number
        of vertices returned is ``4 * step``.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    longitude, latitude : array-like
        The coordinates of the 4 vertices of the HEALPix cells.
        `lon` and `lat` are of shape :math:`N` x :math:`4` numpy arrays where N is the number of HEALPix cell given in `ipix`.

    Raises
    ------
    ValueError
        When the HEALPix cell indexes given have values out of :math:`[0, 4^{29 - depth}[`.

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
    _check_depth(depth)
    ipix = np.atleast_1d(ipix)
    _check_ipixels(data=ipix, depth=depth)
    ipix = ipix.astype(np.uint64)

    num_threads = np.uint16(num_threads)

    return healpix_geo.nested.vertices(depth, ipix, ellipsoid, step, num_threads)


def bilinear_interpolation(
    longitude, latitude, depth, *, ellipsoid="sphere", num_threads=0
):
    """Get the cell ids and weights necessary to bilinearly interpolate the given values.

    Parameters
    ----------
    longitude : array-like
        The longitudes of the input points, in degrees.
    latitude : array-like
        The latitudes of the input points, in degrees.
    depth : int, or `numpy.ndarray`
        The depth of the HEALPix cells. If given as an array, should have the same shape than ipix
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.vertices`.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    cell_ids : array-like
        The neighbours above and below the given points as a :math:`N` x :math:`4` masked array.
    weights : array-like
        The associated weights as a :math:`N` x :math:`4` masked array.

    Raises
    ------
    ValueError
        When the HEALPix cell indexes given have values out of :math:`[0, 4^{29 - depth})`.

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
    _check_depth(depth)
    longitude = np.atleast_1d(longitude).astype("float64")
    latitude = np.atleast_1d(latitude).astype("float64")

    num_threads = np.uint16(num_threads)

    ipix, weights = healpix_geo.nested.bilinear_interpolation(
        depth, longitude, latitude, ellipsoid, num_threads
    )

    xp = marray.masked_namespace(np)
    mask = weights == 0

    return xp.asarray(ipix, mask=mask), xp.asarray(weights, mask=mask)


def kth_neighbourhood(ipix, depth, ring, num_threads=0):
    """Get the kth ring neighbouring cells of some HEALPix cells at a given depth.

    This method returns a :math:`N` x :math:`(2 k + 1)^2` `np.uint64` numpy array containing the neighbours of each cell of the :math:`N` sized `ipix` array.
    This method is wrapped around the `kth_neighbourhood <https://docs.rs/cdshealpix/0.1.5/cdshealpix/nested/struct.Layer.html#method.kth_neighbourhood>`__
    method from the `cdshealpix Rust crate <https://crates.io/crates/cdshealpix>`__.

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes given as a `np.uint64` numpy array.
    depth : int
        The depth of the HEALPix cells.
    ring : int
        The number of rings. `ring=0` returns just the input cell ids, `ring=1` returns the 8 (or 7) immediate
        neighbours, `ring=2` returns the 8 (or 7) immediate neighbours plus their immediate neighbours (a total of 24 cells), and so on.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    neighbours : `numpy.ndarray`
        A :math:`N` x :math:`(2 k + 1)^2` `np.int64` numpy array containing the kth ring neighbours of each cell.
        The :math:`5^{th}` element corresponds to the index of HEALPix cell from which the neighbours are evaluated.
        All its 8 neighbours occupy the remaining elements of the line.

    Raises
    ------
    ValueError
        When the HEALPix cell indexes given have values out of :math:`[0, 4^{29 - depth}[`.

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
    _check_depth(depth)
    ipix = np.atleast_1d(ipix)
    _check_ipixels(data=ipix, depth=depth)
    ipix = ipix.astype(np.uint64)
    _check_ring(depth, ring)

    num_threads = np.uint16(num_threads)
    return healpix_geo.nested.kth_neighbourhood(depth, ipix, ring, num_threads)


def zoom_to(ipix, depth, new_depth, num_threads=0):
    r"""Change the resolutions the given cell ids

    Parameters
    ----------
    ipix : numpy.ndarray
        The HEALPix cell indexes given as a `np.uint64` numpy array.
    depth : int
        The depth of the HEALPix cells.
    new_depth : int
        The new depth of the HEALPix cells.

    Returns
    -------
    cells : numpy.ndarray
        A :math:`N` (`depth >= new_depth`) or :math:`N` x :math:`4^{\delta d}` `np.int64` numpy array containing the parents or children of the given cells.
        If `depth == new_depth`, returns the input pixels
    """
    _check_depth(depth)
    _check_depth(new_depth)

    if depth == new_depth:
        return ipix

    ipix = np.atleast_1d(ipix)
    _check_ipixels(data=ipix, depth=depth)
    ipix = ipix.astype(np.uint64)

    num_threads = np.uint16(num_threads)
    return healpix_geo.nested.zoom_to(depth, ipix, new_depth, num_threads)


def siblings(ipix, depth, num_threads=0):
    r"""Find the siblings for every cell

    Parameters
    ----------
    ipix : numpy.ndarray
        The HEALPix cell indexes given as a `np.uint64` numpy array.
    depth : int
        The depth of the HEALPix cells.

    Returns
    -------
    cells : numpy.ndarray
        A :math:`N` x :math:`4` or :math:`N` x :math:`12` `np.uint64` numpy array containing the siblings of the given cells.
        If `depth == 0`, the siblings are the base cells.
    """
    _check_depth(depth)
    ipix = np.atleast_1d(ipix)
    _check_ipixels(data=ipix, depth=depth)
    ipix = ipix.astype(np.uint64)

    num_threads = np.uint16(num_threads)

    return healpix_geo.nested.siblings(depth, ipix, num_threads)


def angular_distances(from_, to_, depth, num_threads=0):
    """Compute the angular distances between cell centers

    Parameters
    ----------
    from_ : numpy.ndarray
        The source Healpix cell indexes given as a ``np.uint64`` numpy array. Should be 1D.
    to_ : numpy.ndarray
        The destination Healpix cell indexes given as a ``np.uint64`` numpy array.
        Should be 2D.
    depth : int
        The depth of the Healpix cells.
    num_threads : int, default: 0
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    distances : numpy.ndarray
        The angular distances in radians.

    Raises
    ------
    ValueError
        When the Healpix cell indexes given have values out of :math:`[0, 4^{depth}[`.
    """
    _check_depth(depth)

    from_ = np.atleast_1d(from_)
    _check_ipixels(data=from_, depth=depth)
    from_ = from_.astype("uint64")

    mask = to_ != -1
    masked_to = np.where(mask, to_, 0)

    to_ = np.atleast_1d(masked_to)
    _check_ipixels(data=to_, depth=depth)
    to_ = to_.astype("uint64")

    if from_.shape != to_.shape and from_.shape != to_.shape[:-1]:
        raise ValueError(
            "The shape of `from_` must be compatible with the shape of `to_`:\n"
            f"{to_.shape} or {to_.shape[:-1]} must be equal to {from_.shape}."
        )

    if from_.shape == to_.shape:
        intermediate_shape = to_.shape + (1,)
    else:
        intermediate_shape = to_.shape

    num_threads = np.uint16(num_threads)

    distances = healpix_geo.nested.angular_distances(
        depth, from_, np.reshape(to_, intermediate_shape), num_threads
    )

    return np.where(mask, np.reshape(distances, to_.shape), np.nan)


def zone_coverage(bbox, depth, *, ellipsoid="sphere", flat=True):
    """Search the cells covering the given bounding box

    Parameters
    ----------
    bbox : tuple of float
        The 2D bounding box to rasterize.
    depth : int
        The maximum depth of the cells to be returned.
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate healpix on.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    depths : numpy.ndarray
        The depths of the cell ids. If ``flat is True``, these will all have the same value.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the bounding box.
    """
    _check_depth(depth)

    return healpix_geo.nested.zone_coverage(depth, bbox, ellipsoid=ellipsoid, flat=flat)


def box_coverage(center, size, angle, depth, *, ellipsoid="sphere", flat=True):
    """Search the cells covering the given box.

    Parameters
    ----------
    center : numpy.ndarray or tuple of float
        The center of the box, either as a 2-sized array or as a 2-tuple of float.
    size : numpy.ndarray or tuple of float
        The size of the box, in degree.
    angle : float
        The angle by which the box is rotated, in degree.
    depth : int
        The maximum depth of the cells to be returned.
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate healpix on.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    depths : numpy.ndarray
        The depths of the cell ids. If ``flat is True``, these will all have the same value.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the box.
    """
    _check_depth(depth)

    if not isinstance(center, tuple):
        center = tuple(center)
    if not isinstance(size, tuple):
        size = tuple(size)

    return healpix_geo.nested.box_coverage(
        depth, center, size, angle, ellipsoid=ellipsoid, flat=flat
    )


def polygon_coverage(vertices, depth, *, ellipsoid="sphere", flat=True):
    """Search the cells covering the given polygon.

    Parameters
    ----------
    vertices : numpy.ndarray
        The vertices of the polygon without holes. Must be an array of shape ``(n, 2)``.
    depth : int
        The maximum depth of the cells to be returned.
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.polygon_search`.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    depths : numpy.ndarray
        The depths of the cell ids. If ``flat is True``, these will all have the same value.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the polygon.
    """
    _check_depth(depth)

    return healpix_geo.nested.polygon_coverage(
        depth, vertices, ellipsoid=ellipsoid, flat=flat
    )


def cone_coverage(
    center, radius, depth, *, delta_depth=0, ellipsoid="sphere", flat=True
):
    """Search the cells covering the given cone

    Cone in this case means a circle on the surface of the reference ellipsoid.

    Parameters
    ----------
    center : numpy.ndarray or tuple of float
        The center of the box, either as a 2-sized array or as a 2-tuple of float.
    radius : float
        The radius of the cone, in degree.
    depth : int
        The maximum depth of the cells to be returned.
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.cone_search`.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    depths : numpy.ndarray
        The depths of the cell ids. If ``flat is True``, these will all have the same value.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the circle.
    """
    _check_depth(depth)

    if not isinstance(center, tuple):
        center = tuple(center)

    return healpix_geo.nested.cone_coverage(
        depth, center, radius, delta_depth=delta_depth, ellipsoid=ellipsoid, flat=flat
    )


def elliptical_cone_coverage(
    center,
    ellipse_geometry,
    position_angle,
    depth,
    *,
    delta_depth=0,
    ellipsoid="sphere",
    flat=True,
):
    """Search the cells covering the given elliptical cone.

    Elliptical cone in this case refers to an ellipse on the surface of the reference ellipsoid.

    Parameters
    ----------
    center : numpy.ndarray or tuple of float
        The center of the box, either as a 2-sized array or as a 2-tuple of float.
    ellipse_geometry : numpy.ndarray or tuple of float
        The semimajor and semimajor axis, as a 2-sized array or as a 2-tuple of float.
    position_angle : float
        The orientation of the ellipse.
    depth : int
        The maximum depth of the cells to be returned.
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid
        is spherical, this will return the same result as
        :py:func:`cdshealpix.nested.elliptical_cone_search`.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    depths : numpy.ndarray
        The depths of the cell ids. If ``flat is True``, these will all have the same value.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the ellipse.
    """
    _check_depth(depth)

    if not isinstance(center, tuple):
        center = tuple(center)
    if not isinstance(ellipse_geometry, tuple):
        ellipse_geometry = tuple(ellipse_geometry)

    return healpix_geo.nested.elliptical_cone_coverage(
        depth,
        center,
        ellipse_geometry,
        position_angle,
        delta_depth=delta_depth,
        ellipsoid=ellipsoid,
        flat=flat,
    )
