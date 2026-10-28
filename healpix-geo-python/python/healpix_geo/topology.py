from __future__ import annotations

from typing import TYPE_CHECKING

import numpy as np

from healpix_geo import _healpix_geo_python

if TYPE_CHECKING:
    from numpy.typing import NDArray

    from healpix_geo.nested import Direction


def base_cell_relationship(
    base_cell: int, direction: Direction
) -> tuple[int | None, NDArray[np.int8], NDArray[np.int8]]:
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
    displacement_i, displacement_j : numpy.ndarray of numpy.int8
        The change in direction between the base vectors of the source and target base
        cells.

    Raises
    ------
    ValueError
        If `base_cell` is outside the range [0, 11].

    Examples
    --------
    >>> from healpix_geo.topology import base_cell_relationship
    >>> target_cell, delta_i, delta_j = base_cell_relationship(0, "N")
    >>> target_cell
    2
    >>> delta_i
    array([-1,  0], dtype=int8)
    >>> delta_j
    array([ 0, -1], dtype=int8)
    >>> target_cell, _, _ = base_cell_relationship(4, "N")
    >>> target_cell is None
    True
    """
    if base_cell < 0 or base_cell > 11:
        raise ValueError("Base cell must be in range [0, 11].")

    return _healpix_geo_python.base_cell_relationship(base_cell, direction)
