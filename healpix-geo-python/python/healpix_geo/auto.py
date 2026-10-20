from __future__ import annotations

from dataclasses import dataclass
from types import ModuleType
from typing import TYPE_CHECKING, Literal

import numpy as np

from healpix_geo.typing import EllipsoidLike
from healpix_geo.utils import validate_cell_ids

if TYPE_CHECKING:
    from typing import Any

    from numpy.typing import NDArray

    from healpix_geo.typing import Array1D, ArrayLikeFloat, ArrayLikeInt


def _dispatch_module(indexing_scheme: str) -> ModuleType:
    from healpix_geo import nested, ring, zuniq

    modules = {
        "nested": nested,
        "ring": ring,
        "zuniq": zuniq,
    }

    module = modules.get(indexing_scheme)
    if module is None:
        raise ValueError(
            f"unknown indexing scheme: {indexing_scheme}."
            f" Available are: {', '.join(modules.keys())}"
        )

    return module


SchemeType = Literal["nested", "ring", "zuniq"]


@dataclass(frozen=True)
class Grid:
    """Describe the parameters of a given grid.

    Parameters
    ----------
    level: int, optional
        The refinement level of the grid.
    indexing_scheme: {"nested", "ring", "zuniq"}
        The indexing scheme of the grid.
    ellipsoid: ellipsoid-like, default: "sphere"
        The reference ellipsoid of the grid.
    """

    level: int | None
    """The refinement level of the grid."""

    indexing_scheme: SchemeType = "nested"
    """The indexing scheme of the grid."""

    ellipsoid: EllipsoidLike = "sphere"
    """The reference ellipsoid of the grid."""

    def __post_init__(self) -> None:
        if isinstance(self.ellipsoid, str):
            from healpix_geo.ellipsoid import resolve

            # normalize to an object or dict
            object.__setattr__(self, "ellipsoid", resolve(self.ellipsoid))

    def _as_params(self) -> dict:
        params: dict[str, Any] = {"ellipsoid": self.ellipsoid}
        if self.indexing_scheme != "zuniq":
            params["depth"] = self.level

        return params


def convert(
    ipix: ArrayLikeInt,
    grid: Grid,
    *,
    to: SchemeType,
    level: int | NDArray[np.uint8] | None = None,
    num_threads: int = 0,
) -> NDArray[np.uint64] | tuple[NDArray[np.uint64], int | NDArray[np.uint8]]:
    """Convert the cell ids to a different indexing scheme

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices, in the indexing scheme specified by `grid`.
    grid : Grid
        The definition of the HEALPix grid.
    to : {"nested", "ring", "zuniq"}
        The indexing scheme to convert to. If equal to the grid object's
        indexing scheme the input cell ids will be returned unmodified.
    level : int or `numpy.ndarray`, optional
        The levels in case the cells have a different sizes.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    converted : numpy.ndarray of numpy.uint64
        The HEALPix cell indices in the target indexing scheme, with the same shape as
        the input.
    levels : int or numpy.ndarray of numpy.uint8
        The refinement level of the cell indices. Not returned if the output and input
        schemes are identical.

        - For ``nested`` and ``ring``: the `grid` object refinement level
        - For ``zuniq``: an array of refinement levels

    Raises
    ------
    ValueError
        If `ipix` has negative values for ``zuniq``, or outside the range
        :math:`[0, 12 \\times 4^{level}[` for ``nested`` and ``ring``.
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If `level` is specified for ``zuniq`` or no level is specified for ``nested`` or
        ``ring``, neither via `grid.level` or `level`
    ValueError
        If `to` indexing scheme is unknown.
    """
    base_indexing_schemes = {"nested", "ring"}
    if grid.indexing_scheme == to:
        return validate_cell_ids(ipix)

    params: dict[str, Any] = {"num_threads": num_threads}
    if grid.indexing_scheme not in base_indexing_schemes and level is not None:
        raise ValueError(
            f"indexing scheme {grid.indexing_scheme} already contains"
            " the refinement level in the cell indices"
        )
    elif grid.indexing_scheme in base_indexing_schemes:
        if level is None:
            level = grid.level
        if level is None:
            raise ValueError(
                "A level must be supplied via `grid.level` or `level` for "
                f"target indexing scheme '{to}' (received None)."
            )
        params["depth"] = level

    module = _dispatch_module(grid.indexing_scheme)
    converter = getattr(module, f"to_{to}", None)
    if converter is None:
        raise ValueError(f"unknown indexing scheme: {to!r}")

    result = converter(ipix, **params)
    if grid.indexing_scheme not in {"nested", "ring"} and to in {"nested", "ring"}:
        return result
    else:
        return result, level  # type: ignore[invalid-return-type]


def healpix_to_lonlat(
    ipix: ArrayLikeInt, grid: Grid, *, num_threads: int = 0
) -> tuple[NDArray[np.float64], NDArray[np.float64]]:
    """Get the longitudes and latitudes of the center of some HEALPix cells.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices, in the indexing scheme specified by `grid`.
    grid : Grid
        The definition of the HEALPix grid.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    lon, lat : numpy.ndarray of numpy.float64
        The longitude and latitude of the HEALPix cells centers, in degrees,
        with the same shape as the input.

    Raises
    ------
    ValueError
        If `ipix` has negative values for ``zuniq``, or outside the range
        :math:`[0, 12 \\times 4^{level}[` for ``nested`` and ``ring``.
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.

    Examples
    --------
    >>> import healpix_geo.auto as hg
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> grid = hg.Grid(level=3, indexing_scheme="nested", ellipsoid="WGS84")
    >>> lon, lat = hg.healpix_to_lonlat(ipix, grid)
    >>> lon
    array([ 5.625, 50.625, 28.125])
    >>> lat
    array([41.93785391, 19.55202227, 19.55202227])
    """
    module = _dispatch_module(grid.indexing_scheme)
    params = grid._as_params()
    return module.healpix_to_lonlat(ipix, num_threads=num_threads, **params)


def lonlat_to_healpix(
    lon: ArrayLikeFloat,
    lat: ArrayLikeFloat,
    grid: Grid,
    *,
    num_threads: int = 0,
) -> NDArray[np.uint64]:
    """Get the HEALPix indices that contains specific points.

    Parameters
    ----------
    lon : array-like of float
        The longitudes of the input points, in degrees.
    lat : array-like of float
        The latitudes of the input points, in degrees.
    grid : Grid
        The definition of the HEALPix grid.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    ipix : numpy.ndarray of numpy.uint64
        The corresponding HEALPix cells indices, in the indexing scheme specified by
        `grid`, with the same shape as the inputs.

    Raises
    ------
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If `longitude` and `latitude` do not have the same shape.

    Examples
    --------
    >>> import healpix_geo.auto as hg
    >>> import numpy as np
    >>> lon = np.array([0, 50, 25], dtype=np.float64)
    >>> lat = np.array([6, -12, 45], dtype=np.float64)
    >>> grid = hg.Grid(level=3, indexing_scheme="nested", ellipsoid="WGS84")
    >>> ipix = hg.lonlat_to_healpix(lon, lat, grid)
    >>> ipix
    array([304, 573,  38], dtype=uint64)
    """
    module = _dispatch_module(grid.indexing_scheme)
    params = {"depth": grid.level, "ellipsoid": grid.ellipsoid}

    return module.lonlat_to_healpix(lon, lat, num_threads=num_threads, **params)


def healpix_to_cartesian(
    ipix: ArrayLikeInt, grid: Grid, *, num_threads: int = 0
) -> tuple[NDArray[np.float64], NDArray[np.float64], NDArray[np.float64]]:
    """Get the cartesian coordinates of the center of the given HEALPix cells.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cells indices, in the indexing scheme specified by `grid`.
    grid : Grid
        The definition of the HEALPix grid.
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
        If `ipix` has negative values for ``zuniq``, or outside the range
        :math:`[0, 12 \\times 4^{level}[` for ``nested`` and ``ring``.
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.

    Examples
    --------
    >>> import healpix_geo.auto as hg
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> grid = hg.Grid(level=3, indexing_scheme="nested", ellipsoid="WGS84")
    >>> x, y, z = hg.healpix_to_cartesian(ipix, grid)
    >>> x
    array([4728734.69011096, 3814362.85063174, 5302653.40426395])
    >>> y
    array([ 465739.71573273, 4647814.58136658, 2834327.29466645])
    >>> z
    array([4240471.60205904, 2121029.89621885, 2121029.89621885])
    """
    module = _dispatch_module(grid.indexing_scheme)
    params = grid._as_params()

    return module.healpix_to_cartesian(ipix, num_threads=num_threads, **params)


def cartesian_to_healpix(
    x: ArrayLikeFloat,
    y: ArrayLikeFloat,
    z: ArrayLikeFloat,
    grid: Grid,
    *,
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
    grid : Grid
        The definition of the HEALPix grid.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    ipix : numpy.ndarray of numpy.uint64
        The corresponding HEALPix cells indices, in the indexing scheme specified by
        `grid`, with the same shape as the inputs.

    Raises
    ------
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If `x`, `y` and `z` do not have the same shape.

    Examples
    --------
    >>> import healpix_geo.auto as hg
    >>> import numpy as np
    >>> x = np.array([4728734.69011096, 3814362.85063174, 5302653.40426395])
    >>> y = np.array([465739.71573273, 4647814.58136658, 2834327.29466645])
    >>> z = np.array([4240471.60205904, 2121029.89621885, 2121029.89621885])
    >>> grid = hg.Grid(level=3, indexing_scheme="nested", ellipsoid="WGS84")
    >>> ipix = hg.cartesian_to_healpix(x, y, z, grid)
    >>> ipix
    array([42,  6, 10], dtype=uint64)
    """
    module = _dispatch_module(grid.indexing_scheme)
    params = {"depth": grid.level, "ellipsoid": grid.ellipsoid}

    return module.cartesian_to_healpix(x, y, z, num_threads=num_threads, **params)


def vertices(
    ipix: ArrayLikeInt, grid: Grid, *, step: int = 1, num_threads: int = 0
) -> tuple[NDArray[np.float64], NDArray[np.float64]]:
    """Get the longitudes and latitudes of the vertices of some HEALPix cells.

    This method returns the 4 vertices of each cell in `ipix`.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices, in the indexing scheme specified by `grid`.
    grid : Grid
        The definition of the HEALPix grid.
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
        If `ipix` has negative values for ``zuniq``, or outside the range
        :math:`[0, 12 \\times 4^{level}[` for ``nested`` and ``ring``.
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If the name of the ellipsoid is unknown.

    Examples
    --------
    Imports

    >>> import healpix_geo.auto as hg
    >>> import numpy as np

    Set up the grid and cell ids

    >>> ipix = np.array([42, 6, 10])
    >>> grid = hg.Grid(level=12, indexing_scheme="nested", ellipsoid="sphere")
    >>> grid
    Grid(level=12, indexing_scheme='nested', ellipsoid={'name': 'sphere', 'radius': 6370997.0})

    Compute just the vertices:

    >>> lon, lat = hg.vertices(ipix, grid)
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

    Subsample the edges to have 3 additional points per edge:

    >>> lon, lat = hg.vertices(ipix, grid, step=4)
    >>> np.stack([lon, lat], axis=-1)
    array([[[4.49230957e+01, 6.52784088e-02],
            [4.49258423e+01, 6.76097816e-02],
            [4.49285889e+01, 6.99411545e-02],
            [4.49313354e+01, 7.22725275e-02],
            [4.49340820e+01, 7.46039007e-02],
            [4.49313354e+01, 7.69352739e-02],
            [4.49285889e+01, 7.92666473e-02],
            [4.49258423e+01, 8.15980209e-02],
            [4.49230957e+01, 8.39293945e-02],
            [4.49203491e+01, 8.15980209e-02],
            [4.49176025e+01, 7.92666473e-02],
            [4.49148560e+01, 7.69352739e-02],
            [4.49121094e+01, 7.46039007e-02],
            [4.49148560e+01, 7.22725275e-02],
            [4.49176025e+01, 6.99411545e-02],
            [4.49203491e+01, 6.76097816e-02]],
    <BLANKLINE>
           [[4.50109863e+01, 2.79764560e-02],
            [4.50137329e+01, 3.03078275e-02],
            [4.50164795e+01, 3.26391991e-02],
            [4.50192261e+01, 3.49705707e-02],
            [4.50219727e+01, 3.73019424e-02],
            [4.50192261e+01, 3.96333142e-02],
            [4.50164795e+01, 4.19646860e-02],
            [4.50137329e+01, 4.42960579e-02],
            [4.50109863e+01, 4.66274299e-02],
            [4.50082397e+01, 4.42960579e-02],
            [4.50054932e+01, 4.19646860e-02],
            [4.50027466e+01, 3.96333142e-02],
            [4.50000000e+01, 3.73019424e-02],
            [4.50027466e+01, 3.49705707e-02],
            [4.50054932e+01, 3.26391991e-02],
            [4.50082397e+01, 3.03078275e-02]],
    <BLANKLINE>
           [[4.49670410e+01, 2.79764560e-02],
            [4.49697876e+01, 3.03078275e-02],
            [4.49725342e+01, 3.26391991e-02],
            [4.49752808e+01, 3.49705707e-02],
            [4.49780273e+01, 3.73019424e-02],
            [4.49752808e+01, 3.96333142e-02],
            [4.49725342e+01, 4.19646860e-02],
            [4.49697876e+01, 4.42960579e-02],
            [4.49670410e+01, 4.66274299e-02],
            [4.49642944e+01, 4.42960579e-02],
            [4.49615479e+01, 4.19646860e-02],
            [4.49588013e+01, 3.96333142e-02],
            [4.49560547e+01, 3.73019424e-02],
            [4.49588013e+01, 3.49705707e-02],
            [4.49615479e+01, 3.26391991e-02],
            [4.49642944e+01, 3.03078275e-02]]])
    """
    module = _dispatch_module(grid.indexing_scheme)
    params = grid._as_params()

    return module.vertices(ipix, num_threads=num_threads, step=step, **params)


def bilinear_interpolation(
    longitude: ArrayLikeFloat,
    latitude: ArrayLikeFloat,
    grid: Grid,
    *,
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
    grid : Grid
        The definition of the HEALPix grid.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    cell_ids : masked numpy.ndarray of numpy.uint64
        The indices of the HEALPix cells around the given points, in the indexing
        scheme specified by `grid`, with the same shape as the inputs plus one axis
        of size 4. If the inputs have shape S the output will be of shape ``(*S, 4)``.
        The array is masked where the weights are zero.
    weights : masked numpy.ndarray of numpy.float64
        The interpolation weights, with the same shape as the inputs plus one axis of
        size 4. The array is masked where the weights are zero.

    Raises
    ------
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If `longitude` and `latitude` do not have the same shape.

    Examples
    --------
    >>> import healpix_geo.auto as hg
    >>> import numpy as np

    Define the grid

    >>> grid = hg.Grid(level=12, indexing_scheme="nested", ellipsoid="sphere")
    >>> grid
    Grid(level=12, indexing_scheme='nested', ellipsoid={'name': 'sphere', 'radius': 6370997.0})

    Define coordinates

    >>> lon = np.array([-15.0, -10.0, -5.0, 0.0, 5.0], dtype=np.float64)
    >>> lat = np.array([30.0, 35.0, 40.0, 45.0, 50.0], dtype=np.float64)

    Compute interpolation weights

    >>> cell_ids, weights = hg.bilinear_interpolation(lon, lat, grid)
    >>> cell_ids
    MArray(
        array([[54892952, 54892953, 54892954, 54892955],
               [55675332, 55675333, 55675334, 55675335],
               [55890822, 55890823, 55890828, 55890829],
               [11206655, 11250346, 55967743, 56055125],
               [11481364, 11481365, 11481366, 11481367]], dtype=uint64),
        array([[False, False, False, False],
               [False, False, False, False],
               [False, False, False, False],
               [False, False, False, False],
               [False, False, False, False]])
    )
    >>> weights
    MArray(
        array([[0.13888889, 0.69444444, 0.02777778, 0.13888889],
               [0.21156076, 0.15051566, 0.37273788, 0.2651857 ],
               [0.12397948, 0.17692801, 0.28803912, 0.4110534 ],
               [0.00248345, 0.49751655, 0.00248345, 0.49751655],
               [0.0540781 , 0.08623943, 0.33131993, 0.52836254]]),
        array([[False, False, False, False],
               [False, False, False, False],
               [False, False, False, False],
               [False, False, False, False],
               [False, False, False, False]])
    )
    """
    module = _dispatch_module(grid.indexing_scheme)
    params = grid._as_params() | {"depth": grid.level}

    return module.bilinear_interpolation(
        longitude, latitude, **params, num_threads=num_threads
    )


def kth_neighbours(
    ipix: ArrayLikeInt, grid: Grid, *, ring: int, num_threads: int = 0
) -> NDArray[np.int64]:
    """Get the kth ring neighbouring cells of some HEALPix cells.

    This method returns an array containing the ``8 * ring`` neighbours of each input
    cell. This method is wrapped around the `kth_neighbours
    <https://docs.rs/cdshealpix/0.9.1/cdshealpix/nested/struct.Layer.html#method.kth_neighbours>`__
    method of the `cdshealpix Rust crate <https://crates.io/crates/cdshealpix>`__.

    Parameters
    ----------
    ipix : array-like of int
        The HEALPix cell indices, in the indexing scheme specified by `grid`.
    grid : Grid
        The definition of the HEALPix grid.
    ring : int
        The number of rings. `ring=0` returns just the input cell ids, `ring=1` returns
        the 8 (or 7) immediate neighbours, `ring=2` returns the 16 neighbours of the
        immediate neighbours, and so on.
    num_threads : int, default: 0
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
        If `ipix` has negative values for ``zuniq``, or outside the range
        :math:`[0, 12 \\times 4^{level}[` for ``nested`` and ``ring``.
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If `ring` is outside its valid range :math:`[0, 2^{level}[`

    Examples
    --------
    >>> import healpix_geo.auto as hg
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> grid = hg.Grid(level=12, indexing_scheme="nested", ellipsoid="sphere")
    >>> grid
    Grid(level=12, indexing_scheme='nested', ellipsoid={'name': 'sphere', 'radius': 6370997.0})
    >>> ring = 3
    >>> neighbours = hg.kth_neighbours(ipix, grid, ring=ring)
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
    module = _dispatch_module(grid.indexing_scheme)
    params = {}
    if grid.indexing_scheme != "zuniq":
        params["depth"] = grid.level

    return module.kth_neighbours(ipix, ring=ring, num_threads=num_threads, **params)


def kth_neighbourhood(
    ipix: ArrayLikeInt, grid: Grid, *, ring: int, num_threads: int = 0
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
        The HEALPix cell indices, in the indexing scheme specified by `grid`.
    grid : Grid
        The definition of the HEALPix grid.
    ring : int
        The number of rings. ``ring=0`` returns just the input cell ids, `ring=1`
        returns the 8 (or 7) immediate neighbours, ``ring=2`` returns the 8 (or 7)
        immediate neighbours plus their immediate neighbours (a total of 24 cells), and
        so on.
    num_threads : int, default: 0
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
        If `ipix` has negative values for ``zuniq``, or outside the range
        :math:`[0, 12 \\times 4^{level}[` for ``nested`` and ``ring``.
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If `ring` is outside its valid range :math:`[0, 2^{level}[`

    Examples
    --------
    >>> import healpix_geo.auto as hg
    >>> import numpy as np
    >>> ipix = np.array([42, 6, 10])
    >>> grid = hg.Grid(level=12, indexing_scheme="nested", ellipsoid="sphere")
    >>> grid
    Grid(level=12, indexing_scheme='nested', ellipsoid={'name': 'sphere', 'radius': 6370997.0})
    >>> ring = 3
    >>> neighbours = hg.kth_neighbourhood(ipix, grid, ring=ring)
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
    module = _dispatch_module(grid.indexing_scheme)
    params = {}
    if grid.indexing_scheme != "zuniq":
        params["depth"] = grid.level

    return module.kth_neighbourhood(ipix, ring=ring, num_threads=num_threads, **params)


def zone_coverage(
    bbox: tuple[float, float, float, float], grid: Grid, *, flat: bool = True
) -> (
    tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]
    | tuple[Array1D[np.uint64], Array1D[np.bool]]
):
    """Search the cells covering the given bounding box

    Parameters
    ----------
    bbox : tuple of float
        The 2D bounding box to rasterize in order: ``[lon_min, lat_min, lon_max,
        lat_max]``, in degrees.
    grid : Grid
        The definition of the HEALPix grid. The grid level will be the maximum level of
        cells to be returned.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed grid level.

    Returns
    -------
    cell_ids : numpy.ndarray of numpy.uint64
        The HEALPix cell indices covered at least partially by the bounding box, in the
        indexing scheme specified by `grid`.
    levels : numpy.ndarray of numpy.uint8
        The refinement levels of the cell ids. If `flat` is True, these will all have
        the same value. Only returned for ``nested`` and ``ring``.
    fully_covered : numpy.ndarray of numpy.bool
        Boolean array marking whether the cells are fully covered by the bounding box.

    Raises
    ------
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If `bbox` is not of length 4.
    """
    module = _dispatch_module(grid.indexing_scheme)
    return module.zone_coverage(
        bbox, depth=grid.level, ellipsoid=grid.ellipsoid, flat=flat
    )


def box_coverage(
    center: tuple[float, float] | Array1D[np.floating],
    size: tuple[float, float] | Array1D[np.floating],
    angle: float,
    grid: Grid,
    *,
    flat: bool = True,
) -> (
    tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]
    | tuple[Array1D[np.uint64], Array1D[np.bool]]
):
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
        The angle by which the box is rotated, in degree.
    grid : Grid
        The definition of the HEALPix grid. The grid level will be the maximum level of
        cells to be returned.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed grid level.

    Returns
    -------
    cell_ids : numpy.ndarray of numpy.uint64
        The HEALPix cell indices covered at least partially by the bounding box, in the
        indexing scheme specified by `grid`.
    levels : numpy.ndarray of numpy.uint8
        The refinement levels of the cell ids. If `flat` is True, these will all have
        the same value. Only returned for ``nested`` and ``ring``.
    fully_covered : numpy.ndarray of numpy.bool
        Boolean array marking whether the cells are fully covered by the bounding box.

    Raises
    ------
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If `center` and `size` are not one-dimensional of length 2.
    """
    module = _dispatch_module(grid.indexing_scheme)
    return module.box_coverage(
        center=center,
        size=size,
        angle=angle,
        depth=grid.level,
        ellipsoid=grid.ellipsoid,
        flat=flat,
    )


def polygon_coverage(
    vertices: ArrayLikeFloat, grid: Grid, *, flat: bool = True
) -> (
    tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]
    | tuple[Array1D[np.uint64], Array1D[np.bool]]
):
    """Search the cells covering the given polygon.

    Parameters
    ----------
    vertices : array-like of float
        The vertices of the polygon without holes. Must be of shape ``(N, 2)``.
    grid : Grid
        The definition of the HEALPix grid. The grid level will be the maximum level of
        cells to be returned.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed grid level.

    Returns
    -------
    cell_ids : numpy.ndarray of numpy.uint64
        The HEALPix cell indices covered at least partially by the polygon, in the
        indexing scheme specified by `grid`.
    levels : numpy.ndarray of numpy.uint8
        The refinement levels of the cell ids. If `flat` is True, these will all have
        the same value. Only returned for ``nested`` and ``ring``.
    fully_covered : numpy.ndarray of numpy.bool
        Boolean array marking whether the cells are fully covered by the polygon.

    Raises
    ------
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If `vertices` is not of shape (N, 2).
    """
    module = _dispatch_module(grid.indexing_scheme)
    return module.polygon_coverage(
        vertices=vertices, depth=grid.level, ellipsoid=grid.ellipsoid, flat=flat
    )


def cone_coverage(
    center: tuple[float, float] | Array1D[np.floating],
    radius: float,
    grid: Grid,
    *,
    delta_level: int = 0,
    flat: bool = True,
) -> (
    tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]
    | tuple[Array1D[np.uint64], Array1D[np.bool]]
):
    """Search the cells covering the given cone

    Cone in this case means a circle on the surface of the reference ellipsoid.

    Parameters
    ----------
    center : tuple of float or numpy.ndarray of float
        The center of the cone, in degrees, either as a 2-sized array or as a 2-tuple of
        float.
    radius : float
        The radius of the cone, in degrees.
    grid : Grid
        The definition of the HEALPix grid. The grid level will be the maximum level of
        cells to be returned.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed grid level.

    Returns
    -------
    cell_ids : numpy.ndarray of numpy.uint64
        The HEALPix cell indices covered at least partially by the cone, in the
        indexing scheme specified by `grid`.
    levels : numpy.ndarray of numpy.uint8
        The refinement levels of the cell ids. If `flat` is True, these will all have
        the same value. Only returned for ``nested`` and ``ring``.
    fully_covered : numpy.ndarray of numpy.bool
        Boolean array marking whether the cells are fully covered by the cone.

    Raises
    ------
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If `center` is not one-dimensional of length 2.
    """
    module = _dispatch_module(grid.indexing_scheme)
    return module.cone_coverage(
        center=center,
        radius=radius,
        depth=grid.level,
        ellipsoid=grid.ellipsoid,
        delta_depth=delta_level,
        flat=flat,
    )


def elliptical_cone_coverage(
    center: tuple[float, float] | Array1D[np.floating],
    ellipse_geometry: tuple[float, float] | Array1D[np.floating],
    position_angle: float,
    grid: Grid,
    *,
    delta_level: int = 0,
    flat: bool = True,
) -> (
    tuple[Array1D[np.uint64], Array1D[np.uint8], Array1D[np.bool]]
    | tuple[Array1D[np.uint64], Array1D[np.bool]]
):
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
        The orientation of the ellipse, in degrees.
    grid : Grid
        The definition of the HEALPix grid. The grid level will be the maximum level of
        cells to be returned.
    flat : bool, default: True
        If ``True``, the cells returned will all be at the passed grid level.

    Returns
    -------
    cell_ids : numpy.ndarray of numpy.uint64
        The HEALPix cell indices covered at least partially by the ellipse, in the
        indexing scheme specified by `grid`.
    levels : numpy.ndarray of numpy.uint8
        The refinement levels of the cell ids. If `flat` is True, these will all have
        the same value. Only returned for ``nested`` and ``ring``.
    fully_covered : numpy.ndarray of numpy.bool
        Boolean array marking whether the cells are fully covered by the ellipse.

    Raises
    ------
    ValueError
        If `grid.level` is outside the range [0, 29] for ``nested`` or ``ring`` indexing
        schemes.
    ValueError
        If `center` and `ellipse_geometry` are not one-dimensional of length 2.

    """
    module = _dispatch_module(grid.indexing_scheme)
    return module.elliptical_cone_coverage(
        center=center,
        ellipse_geometry=ellipse_geometry,
        position_angle=position_angle,
        depth=grid.level,
        ellipsoid=grid.ellipsoid,
        delta_depth=delta_level,
        flat=flat,
    )
