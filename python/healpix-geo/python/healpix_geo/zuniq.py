import marray
import numpy as np

from healpix_geo import healpix_geo
from healpix_geo.utils import _check_depth, _check_ipixels


def from_nested(ipix, depth, num_threads=0):
    """Convert from nested to zuniq

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes in the nested scheme given as a `np.uint64` numpy array.
    depth : int or array-like of int
        The HEALPix cell depth given as scalar or a `np.uint8` numpy array.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    zuniq : array-like of int
        The cell ids in the zuniq scheme.

    Examples
    --------
    >>> import healpix_geo.zuniq
    >>> import numpy as np
    >>> ipix_nested = np.array([32, 125, 45, 91], dtype="uint64")
    >>> depth = np.array([1, 3, 2, 4], dtype="uint8")
    >>> ipix_zuniq = healpix_geo.zuniq.from_nested(ipix_nested, depth)
    >>> ipix_zuniq
    array([4683743612465315840, 1130403506469994496, 1639310264362860544,
            206039682952200192], dtype=uint64)
    """
    _check_depth(depth)

    ipix = np.atleast_1d(ipix)
    _check_ipixels(data=ipix, depth=depth)
    ipix = ipix.astype(np.uint64)

    depth = depth if isinstance(depth, int) else depth.astype("uint8")
    num_threads = np.uint16(num_threads)

    return healpix_geo.zuniq.from_nested(ipix, depth, num_threads)


def to_nested(ipix, num_threads=0):
    """Convert from zuniq to nested

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes in the zuniq scheme given as a `np.uint64` numpy array.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    nested : array-like of int
        The cell ids in the nested scheme.
    depth : int or array-like of int
        The HEALPix cell depth given as scalar or a `np.uint8` numpy array.

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
    ...     dtype="uint64",
    ... )
    >>> ipix_nested, depth = healpix_geo.zuniq.to_nested(ipix_zuniq)
    >>> ipix_nested
    array([ 32, 125,  45,  91], dtype=uint64)
    >>> depth
    array([1, 3, 2, 4], dtype=uint8)
    """
    ipix = np.atleast_1d(ipix).astype(np.uint64)

    num_threads = np.uint16(num_threads)

    return healpix_geo.zuniq.to_nested(ipix, num_threads)


def healpix_to_lonlat(ipix, ellipsoid, num_threads=0):
    r"""Get the longitudes and latitudes of the center of some HEALPix cells.

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes given as a `np.uint64` numpy array.
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
    >>> from healpix_geo.zuniq import healpix_to_lonlat
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> lon, lat = healpix_to_lonlat(ipix, ellipsoid="WGS84")
    >>> lon
    array([44.99999975, 45.00000008, 44.99999992])
    >>> lat
    array([2.85869025e-07, 1.42934512e-07, 1.42934512e-07])
    """
    ipix = np.atleast_1d(ipix).astype(np.uint64)

    num_threads = np.uint16(num_threads)

    return healpix_geo.zuniq.healpix_to_lonlat(ipix, ellipsoid, num_threads)


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
    >>> from healpix_geo.zuniq import lonlat_to_healpix
    >>> import numpy as np
    >>> lon = np.array([0, 50, 25], dtype="float64")
    >>> lat = np.array([6, -12, 45], dtype="float64")
    >>> depth = 3
    >>> ipix = lonlat_to_healpix(lon, lat, depth, ellipsoid="WGS84")
    >>> ipix
    array([2742692173068632064, 5165628772593958912,  346777171307528192],
          dtype=uint64)
    """
    _check_depth(depth)
    longitude = np.atleast_1d(longitude).astype("float64")
    latitude = np.atleast_1d(latitude).astype("float64")

    num_threads = np.uint16(num_threads)

    return healpix_geo.zuniq.lonlat_to_healpix(
        depth, longitude, latitude, ellipsoid, num_threads
    )


def vertices(ipix, ellipsoid, num_threads=0):
    """Get the longitudes and latitudes of the vertices of some HEALPix cells in zuniq encoding.

    This method returns the 4 vertices of each cell in `ipix`.

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes given as a `np.uint64` numpy array.
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
    longitude, latitude : array-like
        The sky coordinates of the 4 vertices of the HEALPix cells.
        `lon` and `lat` are of shape :math:`N` x :math:`4` numpy arrays where N is the number of HEALPix cell given in `ipix`.

    Raises
    ------
    ValueError
        When the HEALPix cell indexes given have values out of :math:`[0, 4^{29 - depth}[`.

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
    ipix = np.atleast_1d(ipix).astype(np.uint64)

    num_threads = np.uint16(num_threads)

    return healpix_geo.zuniq.vertices(ipix, ellipsoid, num_threads)


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
    depth : int or array-like
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
    >>> from healpix_geo.zuniq import bilinear_interpolation
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

    ipix, weights = healpix_geo.zuniq.bilinear_interpolation(
        depth, longitude, latitude, ellipsoid, num_threads
    )

    xp = marray.masked_namespace(np)
    mask = weights == 0

    return xp.asarray(ipix, mask=mask), xp.asarray(weights, mask=mask)


def kth_neighbourhood(ipix, ring, num_threads=0):
    """Get the kth ring neighbouring cells of some HEALPix cells.

    This method returns a :math:`N` x :math:`(2 k + 1)^2` `np.uint64` numpy array containing the neighbours of each cell of the :math:`N` sized `ipix` array.
    This method is wrapped around the `kth_neighbourhood <https://docs.rs/cdshealpix/0.1.5/cdshealpix/nested/struct.Layer.html#method.kth_neighbourhood>`__
    method from the `cdshealpix Rust crate <https://crates.io/crates/cdshealpix>`__.

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes given as a `np.uint64` numpy array.
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
    >>> from healpix_geo.zuniq import kth_neighbourhood
    >>> import numpy as np
    >>> ipix = np.array([1460288880640, 223338299392, 360777252864], dtype="uint64")
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
    ipix = np.astype(np.atleast_1d(ipix), np.uint64)

    num_threads = np.uint16(num_threads)
    return healpix_geo.zuniq.kth_neighbourhood(ipix, ring, num_threads)
