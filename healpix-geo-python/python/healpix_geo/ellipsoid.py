from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from healpix_geo.typing import EllipsoidLike


def resolve(name: str) -> EllipsoidLike:
    """Look up the ellipsoid based on a name

    Parameters
    ----------
    name : str
        The name of the ellipsoid.

    Returns
    -------
    mapping
        The parameters of the ellipsoid. The passed name will be included.

    Raises
    ------
    ValueError
        If the name is unknown.
    """
    from healpix_geo._healpix_geo_python import resolve_ellipsoid

    return resolve_ellipsoid(name)
