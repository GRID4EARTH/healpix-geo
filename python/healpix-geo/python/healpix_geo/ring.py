import marray
import numpy as np

from healpix_geo import healpix_geo
from healpix_geo.utils import _check_depth, _check_ipixels, _check_ring


def healpix_to_lonlat(ipix, depth, ellipsoid="sphere", num_threads=0):
    r"""Get the longitudes and latitudes of the center of some HEALPix cells.

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes given as a `np.uint64` numpy array.
    depth : `numpy.ndarray`
        The HEALPix cell depth given as a `np.uint8` numpy array.
    ellipsoid : str, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If ``"sphere"``, this will return
        the same result as :py:func:`cdshealpix.ring.healpix_to_lonlat`.
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
    >>> from healpix_geo.ring import healpix_to_lonlat
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> depth = 3
    >>> lon, lat = healpix_to_lonlat(ipix, depth, ellipsoid="WGS84")
    >>> lon
    array([ 45. , 112.5, 292.5])
    >>> lat
    array([60.54441647, 78.33504545, 78.33504545])
    """
    _check_depth(depth)
    ipix = np.atleast_1d(ipix)
    _check_ipixels(data=ipix, depth=depth)
    ipix = ipix.astype(np.uint64)

    num_threads = np.uint16(num_threads)

    return healpix_geo.ring.healpix_to_lonlat(depth, ipix, ellipsoid, num_threads)


def lonlat_to_healpix(longitude, latitude, depth, ellipsoid="sphere", num_threads=0):
    r"""Get the HEALPix indexes that contains specific points.

    Parameters
    ----------
    lon : array-like
        The longitudes of the input points, in degrees.
    lat : array-like
        The latitudes of the input points, in degrees.
    depth : int or array-like of int
        The HEALPix cell depth given as a `np.uint8` numpy array.
    ellipsoid : str, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If ``"sphere"``, this will return
        the same result as :py:func:`cdshealpix.ring.lonlat_to_healpix`.
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
    >>> from healpix_geo.ring import lonlat_to_healpix
    >>> import numpy as np
    >>> lon = np.array([0, 50, 25], dtype="float64")
    >>> lat = np.array([6, -12, 45], dtype="float64")
    >>> depth = 3
    >>> ipix = lonlat_to_healpix(lon, lat, depth, ellipsoid="WGS84")
    >>> ipix
    array([336, 436, 114], dtype=uint64)
    """
    _check_depth(depth)
    longitude = np.atleast_1d(longitude).astype("float64")
    latitude = np.atleast_1d(latitude).astype("float64")

    num_threads = np.uint16(num_threads)

    return healpix_geo.ring.lonlat_to_healpix(
        depth, longitude, latitude, ellipsoid, num_threads
    )


def vertices(ipix, depth, ellipsoid, step=1, num_threads=0):
    """Get the longitudes and latitudes of the vertices of some HEALPix cells at a given depth.

    This method returns the 4 vertices of each cell in `ipix`.

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes given as a `np.uint64` numpy array.
    depth : int, or `numpy.ndarray`
        The depth of the HEALPix cells. If given as an array, should have the same shape than ipix
    ellipsoid : str, default: "sphere"
        Reference ellipsoid to evaluate healpix on. If ``"sphere"``, this will return
        the same result as :py:func:`cdshealpix.ring.vertices`.
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
        The sky coordinates of the 4 vertices of the HEALPix cells.
        `lon` and `lat` are of shape :math:`N` x :math:`4` numpy arrays where N is the number of HEALPix cell given in `ipix`.

    Raises
    ------
    ValueError
        When the HEALPix cell indexes given have values out of :math:`[0, 4^{29 - depth}[`.

    Examples
    --------
    >>> from healpix_geo.ring import vertices
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> depth = 12
    >>> lon, lat = vertices(ipix, depth, ellipsoid="sphere")
    >>> np.stack([lon, lat], axis=-1)
    array([[[ 45.        ,  89.93147196],
            [ 54.        ,  89.9428933 ],
            [ 45.        ,  89.95431464],
            [ 36.        ,  89.9428933 ]],
    <BLANKLINE>
           [[120.        ,  89.96573598],
            [135.        ,  89.97715732],
            [ 90.        ,  89.98857866],
            [ 90.        ,  89.97715732]],
    <BLANKLINE>
           [[300.        ,  89.96573598],
            [315.        ,  89.97715732],
            [270.        ,  89.98857866],
            [270.        ,  89.97715732]]])
    """
    _check_depth(depth)
    ipix = np.atleast_1d(ipix)
    _check_ipixels(data=ipix, depth=depth)
    ipix = ipix.astype(np.uint64)

    num_threads = np.uint16(num_threads)

    return healpix_geo.ring.vertices(depth, ipix, ellipsoid, step, num_threads)


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
    depth : int
        The depth of the HEALPix cells.
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
    >>> from healpix_geo.ring import bilinear_interpolation
    >>> import numpy as np

    Define coordinates
    >>> lon = np.array([-15.0, -10.0, -5.0, 0.0, 5.0])
    >>> lat = np.array([30.0, 35.0, 40.0, 45.0, 50.0])

    Compute interpolation weights
    >>> cell_ids, weights = bilinear_interpolation(lon, lat, depth=6, ellipsoid="WGS84")
    >>> cell_ids
    MArray(
        array([[12661, 12405, 12404, 12149],
               [10872, 10617, 10616, 10360],
               [ 9340,  9085,  9084,  8828],
               [ 7320,  7080,  7563,  7319],
               [ 5943,  5727,  5726,  5514]], dtype=uint64),
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

    ipix, weights = healpix_geo.ring.bilinear_interpolation(
        depth, longitude, latitude, ellipsoid, num_threads
    )

    xp = marray.masked_namespace(np)
    mask = weights == 0

    return xp.asarray(ipix, mask=mask), xp.asarray(weights, mask=mask)


def kth_neighbourhood(ipix, depth, ring, num_threads=0):
    """Get the kth ring neighbouring cells of some HEALPix cells at a given depth.

    This method returns a :math:`N` x :math:`(2 k + 1)^2` `np.uint64` numpy array containing the neighbours of each cell of the :math:`N` sized `ipix` array.
    This method is wrapped around the `kth_neighbourhood <https://docs.rs/cdshealpix/0.1.5/cdshealpix/nested/struct.Layer.html#method.neighbours_in_kth_ring>`__
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
        All its 8 neighbours occup the remaining elements of the line.

    Raises
    ------
    ValueError
        When the HEALPix cell indexes given have values out of :math:`[0, 4^{29 - depth}[`.

    Examples
    --------
    >>> from healpix_geo.ring import kth_neighbourhood
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> depth = 12
    >>> ring = 3
    >>> neighbours = kth_neighbourhood(ipix, depth, ring)
    >>> neighbours
    array([[ 42,  87,  62,  41,  25,  13,  26,  43,  63, 148, 115,  86,  61,
             40,  24,  12,   4,   0,   5,  14,  27,  44,  64,  88, 116,  83,
             59,  39,  23,  11,   3,   2,  66,   1,   6,  15,  28,  45, 225,
            184, 147, 114,  85,  60,  65,  89, 117, 149, 185],
           [  6,  14,   5,   0,  29,  15,   1,   7,  16,  43,  26,  13,   4,
             27,  11,   3,  18,   2,   8,  68,  46,  28,  17,  30,  47,  88,
             63,  42,  25,  12,  44,  64,  38,  22,  10,  23,  51,  32,   9,
             19,  33, 123,  93,  67,  45,  31,  48,  69,  94],
           [ 10,  20,   9,   2,  37,  21,   3,  11,  22,  53,  34,  19,   8,
             35,   7,   1,  12,   0,   4,  80,  56,  36,  23,  38,  57, 102,
             75,  52,  33,  18,  54,  76,  30,  16,   6,  17,  41,  24,   5,
             13,  25, 139, 107,  79,  55,  39,  58,  81, 108]])
    """
    _check_depth(depth)
    ipix = np.atleast_1d(ipix)
    _check_ipixels(data=ipix, depth=depth)
    ipix = ipix.astype(np.uint64)
    _check_ring(depth, ring)

    num_threads = np.uint16(num_threads)
    return healpix_geo.ring.kth_neighbourhood(depth, ipix, ring, num_threads)


def angular_distances(from_, to_, depth, num_threads=0):
    """Compute the angular distances

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

    distances = healpix_geo.ring.angular_distances(
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
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid is
        spherical, this will return the same result as
        :py:func:`cdshealpix.nested.zone_search` followed by a translation to the ring
        scheme.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    depths : numpy.ndarray
        The depths of the cell ids. If ``flat is True``, these will all have the same
        value.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the bounding box.
    """
    _check_depth(depth)

    return healpix_geo.ring.zone_coverage(depth, bbox, ellipsoid=ellipsoid, flat=flat)


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
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid is
        spherical, this will return the same result as
        :py:func:`cdshealpix.nested.box_search` followed by a translation to the ring
        scheme.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    depths : numpy.ndarray
        The depths of the cell ids. If ``flat is True``, these will all have the same
        value.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the box.
    """
    _check_depth(depth)

    if not isinstance(center, tuple):
        center = tuple(center)
    if not isinstance(size, tuple):
        size = tuple(size)

    return healpix_geo.ring.box_coverage(
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
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid is
        spherical, this will return the same result as
        :py:func:`cdshealpix.nested.polygon_search` followed by a translation to the ring
        scheme.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    depths : numpy.ndarray
        The depths of the cell ids. If ``flat is True``, these will all have the same
        value.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the polygon.
    """
    _check_depth(depth)

    return healpix_geo.ring.polygon_coverage(
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
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid is
        spherical, this will return the same result as
        :py:func:`cdshealpix.nested.cone_search` followed by a translation to the ring
        scheme.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    depths : numpy.ndarray
        The depths of the cell ids. If ``flat is True``, these will all have the same
        value.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the circle.
    """
    _check_depth(depth)

    if not isinstance(center, tuple):
        center = tuple(center)

    return healpix_geo.ring.cone_coverage(
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
        Reference ellipsoid to evaluate healpix on. If the reference ellipsoid is
        spherical, this will return the same result as
        :py:func:`cdshealpix.nested.polygon_search` followed by a translation to the ring
        scheme.
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

    return healpix_geo.ring.elliptical_cone_coverage(
        depth,
        center,
        ellipse_geometry,
        position_angle,
        delta_depth=delta_depth,
        ellipsoid=ellipsoid,
        flat=flat,
    )
