from __future__ import annotations

from typing import TYPE_CHECKING

from healpix_geo import _healpix_geo_python

if TYPE_CHECKING:
    import numpy as np
    import numpy.typing as npt

    from healpix_geo.typing import Direction


def base_cell_relationship(
    base_cell: int, direction: Direction
) -> tuple[int, npt.NDArray[np.int32], npt.NDArray[np.int32]] | None:
    """Return the adjacent base cell and relative coordinate orientation.

    Parameters
    ----------
    base_cell : int
        Source base cell id in the closed range ``[0, 11]``.
    direction : {"S", "SW", "W", "NW", "N", "NE", "E", "SE"}
        Direction of the target cell in the base cell's local coordinate system.

    Returns
    -------
    target_cell : int
        The id of the target base cell.
    displacement_i, displacement_j : array-like of int32
        The change in direction between the base vectors of the source and target base cells.

    Examples
    --------
    >>> from healpix_geo import base_cell_relationship
    >>> base_cell_relationship(0, "N")
    >>> base_cell_relationship(4, "N") is None
    """
    return _healpix_geo_python.base_cell_relationship(base_cell, direction)
