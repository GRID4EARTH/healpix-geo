from __future__ import annotations

from typing import TYPE_CHECKING

import numpy as np

from healpix_geo import _healpix_geo_python
from healpix_geo.utils import check_depth

if TYPE_CHECKING:
    from numpy.typing import NDArray

    from healpix_geo.typing import ArrayLikeInt, EllipsoidLike


def vertex_to_lonlat(
    vertex_ids: ArrayLikeInt,
    depth: int,
    *,
    ellipsoid: EllipsoidLike = "sphere",
    num_threads: int = 0,
) -> tuple[NDArray[np.float64], NDArray[np.float64]]:
    """Longitude and latitude coordinates for the given vertex ids

    Parameters
    ----------
    vertex_ids : array-like of int
        The given vertex ids.
    depth : int
        The depth of the cells the vertices are computed for.
    ellipsoid : ellipsoid-like, default: "sphere"
        Reference ellipsoid to evaluate the healpix vertex ids on.
    num_threads : int, optional, default: 0
        Specifies the number of threads to use for the computation. The default (0)
        means it will choose the number of threads based on the RAYON_NUM_THREADS
        environment variable (if set), or the number of logical CPUs (otherwise)

    Returns
    -------
    longitude, latitude : numpy.ndarray of numpy.float64
        The coordinates of the given vertices. Both arrays have the same shape as the
        array of vertex ids.

    Raises
    ------
    ValueError
        If `vertex_ids` has values outside the range
        :math:`[0, 12 \\times 4^{depth} + 1]`.
    ValueError
        If `depth` is outside the range [0, 29].

    Examples
    --------
    >>> from healpix_geo import vertex_to_lonlat
    >>> import numpy as np
    >>> depth = 0
    >>> vertex_ids = np.arange(12 * 4**depth + 2, dtype=np.uint64)
    >>> lon, lat = vertex_to_lonlat(vertex_ids, depth, ellipsoid="sphere")
    >>> np.stack([lon, lat], axis=-1)
    array([[  0.       ,  90.       ],
           [  0.       ,  41.8103149],
           [ 90.       ,  41.8103149],
           [180.       ,  41.8103149],
           [270.       ,  41.8103149],
           [ 45.       ,   0.       ],
           [135.       ,   0.       ],
           [225.       ,   0.       ],
           [315.       ,   0.       ],
           [  0.       , -41.8103149],
           [ 90.       , -41.8103149],
           [180.       , -41.8103149],
           [270.       , -41.8103149],
           [  0.       , -90.       ]])
    """
    check_depth(depth)
    vertex_ids = np.atleast_1d(vertex_ids).astype(np.uint64)

    imax = 12 * 4**depth + 2
    if np.any(vertex_ids < 0) or np.any(vertex_ids >= imax):
        raise ValueError(f"Some vertex ids are outside the valid range [0, {imax - 1}]")

    return _healpix_geo_python.vertex_to_lonlat(
        depth, vertex_ids, ellipsoid, num_threads
    )
