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


def healpix_to_cartesian(ipix, ellipsoid="sphere", num_threads=0):
    r"""Get the cartesian coordinates of the center of the given HEALPix cells.

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
    ipix = np.atleast_1d(ipix)
    ipix = ipix.astype(np.uint64)

    num_threads = np.uint16(num_threads)

    return healpix_geo.zuniq.healpix_to_cartesian(ipix, ellipsoid, num_threads)


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
    >>> from healpix_geo.zuniq import cartesian_to_healpix
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
    array([2742692173068632064, 5165628772593958912,  346777171307528192],
          dtype=uint64)
    """
    _check_depth(depth)
    x = np.atleast_1d(x).astype("float64")
    y = np.atleast_1d(y).astype("float64")
    z = np.atleast_1d(z).astype("float64")

    num_threads = np.uint16(num_threads)

    return healpix_geo.zuniq.cartesian_to_healpix(
        depth, x, y, z, ellipsoid, num_threads
    )


def vertices(ipix, ellipsoid, step=1, num_threads=0):
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

    return healpix_geo.zuniq.vertices(ipix, ellipsoid, step, num_threads)


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


def kth_neighbours(ipix, ring, num_threads=0):
    """Get the kth ring of neighbouring cells around some HEALPix cells at a given depth.

    This method returns a :math:`N` x :math:`8 k` `np.uint64` numpy array containing the neighbours of each cell of the :math:`N` sized `ipix` array.
    This method is wrapped around the `kth_neighbours <https://docs.rs/cdshealpix/0.9.1/cdshealpix/nested/struct.Layer.html#method.kth_neighbours>`__
    method of the `cdshealpix Rust crate <https://crates.io/crates/cdshealpix>`__.

    Parameters
    ----------
    ipix : `numpy.ndarray`
        The HEALPix cell indexes given as a `np.uint64` numpy array.
    ring : int
        The number of rings. `ring=0` returns just the input cell ids, `ring=1` returns the 8 (or 7) immediate
        neighbours, `ring=2` returns the 16 neighbours of the immediate neighbours, and so on.
    num_threads : int, optional
        Specifies the number of threads to use for the computation. Default to 0 means
        it will choose the number of threads based on the RAYON_NUM_THREADS environment variable (if set),
        or the number of logical CPUs (otherwise)

    Returns
    -------
    neighbours : `numpy.ndarray`
        A :math:`N` x :math:`8 k` `np.int64` numpy array containing the kth ring neighbours of each cell.

    Raises
    ------
    ValueError
        When the HEALPix cell indexes given have values out of :math:`[0, 4^{29 - depth}[`.

    Examples
    --------
    >>> from healpix_geo.zuniq import kth_neighbours
    >>> import numpy as np
    >>> ipix = np.array([1460288880640, 223338299392, 360777252864], dtype="uint64")
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
    ipix = np.astype(np.atleast_1d(ipix), np.uint64)

    num_threads = np.uint16(num_threads)
    return healpix_geo.zuniq.kth_neighbours(ipix, ring, num_threads)


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
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the bounding box.
    """
    _check_depth(depth)

    return healpix_geo.zuniq.zone_coverage(depth, bbox, ellipsoid=ellipsoid, flat=flat)


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
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the box.
    """
    _check_depth(depth)

    if not isinstance(center, tuple):
        center = tuple(center)
    if not isinstance(size, tuple):
        size = tuple(size)

    return healpix_geo.zuniq.box_coverage(
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
        :py:func:`cdshealpix.nested.polygon_search` followed by a translation to the zuniq
        scheme.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the polygon.
    """
    _check_depth(depth)

    return healpix_geo.zuniq.polygon_coverage(
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
        :py:func:`cdshealpix.nested.cone_search` followed by a translation to the zuniq
        scheme.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the circle.
    """
    _check_depth(depth)

    if not isinstance(center, tuple):
        center = tuple(center)

    return healpix_geo.zuniq.cone_coverage(
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
        :py:func:`cdshealpix.nested.polygon_search` followed by a translation to the zuniq
        scheme.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed depth.

    Returns
    -------
    cell_ids : numpy.ndarray
        The rasterized cell ids.
    fully_covered : numpy.ndarray
        Boolean array marking whether the cells are fully covered by the ellipse.
    """
    _check_depth(depth)

    if not isinstance(center, tuple):
        center = tuple(center)
    if not isinstance(ellipse_geometry, tuple):
        ellipse_geometry = tuple(ellipse_geometry)

    return healpix_geo.zuniq.elliptical_cone_coverage(
        depth,
        center,
        ellipse_geometry,
        position_angle,
        delta_depth=delta_depth,
        ellipsoid=ellipsoid,
        flat=flat,
    )
