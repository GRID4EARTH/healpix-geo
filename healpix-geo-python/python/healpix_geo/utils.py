"""Utilities."""

from collections.abc import Sequence
from typing import TypeVar, cast

import numpy as np
from numpy.typing import NDArray

from healpix_geo.typing import Array1D, ArrayLikeInt


def check_depth(depth: ArrayLikeInt) -> None:
    """Check depth is in valid range [0, 29].

    Raises
    ------
    ValueError
        If `depth` has values outside its valid range [0, 29].
    """
    depth = np.array(depth)
    if np.any(depth < 0) or np.any(depth > 29):
        raise ValueError("Depth must be in the [0, 29] closed range.")


def check_ring(depth: int | NDArray[np.uint8], ring: int) -> None:
    """Check ring is in valid range [0, 2**depth].

    Raises
    ------
    ValueError
        If `ring` is outside its valid range :math:`[0, 2^{depth}[`
    """
    nside = 2 ** np.asarray(depth, dtype=np.uint64)

    if np.any(ring > nside):
        raise ValueError(
            "Crossing base cell boundaries more than once is not supported. Received "
            f" ring={ring}, but expected an integer in the range of [0, {nside}]."
        )


def check_cell_ids(cell_ids: ArrayLikeInt, depth: ArrayLikeInt | None = None) -> None:
    """Check cell ids are in valid range.

    If depth is an array, check it has the same shape as cell ids.

    Raises
    ------
    ValueError
        If `cell_ids` has negative values, or values outside the valid range :math:`[0,
        12 \\times 4^{depth}[` when `depth` is supplied.
    ValueError
        If `depth` is an array with a different shape as `cell_ids`.
    """
    cell_ids = np.atleast_1d(cell_ids)
    if depth is None:
        if np.any(cell_ids < 0):
            raise ValueError("The input HEALPix cells contains a negative value.")
        return

    depth = np.asarray(depth, dtype=np.uint64)
    npix = 12 * 4**depth

    if depth.ndim > 0 and cell_ids.shape != depth.shape:
        raise ValueError(
            "An array valued depth must have the same shape as the cell ids."
        )

    if np.any(cell_ids >= npix) or np.any(cell_ids < 0):
        raise ValueError(
            f"The input HEALPix cells contains a value out of [0, {npix - 1}]"
        )


def validate_depth(depth: ArrayLikeInt) -> int | NDArray[np.uint8]:
    """Check depth value and return it as unsigned integer array.

    Raises
    ------
    ValueError
        If `depth` has values outside its valid range [0, 29].
    """
    depth = np.asarray(depth)
    check_depth(depth)
    return int(depth.item()) if depth.ndim == 0 else depth.astype(np.uint8)


def validate_cell_ids(
    cell_ids: ArrayLikeInt, depth: int | ArrayLikeInt | None = None
) -> NDArray[np.uint64]:
    """Check cell ids values and return it as unsigned integer array.

    If depth is None, only check that the ids are positive, otherwise also check they
    are below max index and have same shape.

    Raises
    ------
    ValueError
        If `cell_ids` has negative values, or values outside the valid range :math:`[0,
        12 \\times 4^{depth}[` when `depth` is supplied.
    ValueError
        If `depth` is an array with a different shape as `cell_ids`.
    """
    cell_ids = np.asarray(cell_ids)
    check_cell_ids(cell_ids, depth)
    return np.atleast_1d(cell_ids.astype(np.uint64))


def check_same_shape(*arrays: np.ndarray, **arrays_kw: np.ndarray) -> None:
    """Check if all passed arrays have the same shape.

    Pass arrays as keyword arguments to get a more informative error message.

    Raises
    ------
    ValueError:
        All arrays do not have the same shape.
    """
    shapes = {a.shape for a in arrays} | {a.shape for a in arrays_kw.values()}
    if len(shapes) != 1:
        shapes_str = [str(a.shape) for a in arrays]
        shapes_str += [f"{name}={a.shape}" for name, a in arrays_kw.items()]
        raise ValueError(
            f"All arrays must have the same shape. Received {', '.join(shapes_str)}"
        )


_T = TypeVar("_T")


def _check_tuple_length(
    seq: Sequence | Array1D, length: int, *, name: str = ""
) -> None:
    """Check sequence is of correct length."""
    if len(seq) != length or (isinstance(seq, np.ndarray) and seq.ndim != 1):
        msg = f"Expected tuple of length {length}"
        if name:
            msg += f" for argument '{name}'"
        msg += f", received '{seq}'"
        raise ValueError(msg)


def validate_tuple2(seq: Sequence[float] | Array1D, name: str = "") -> tuple[_T, _T]:
    """Check sequence is of length 2 and return as a tuple.

    Raise
    -----
    ValueError
        The sequence is not one-dimensional of length 2.
    """
    _check_tuple_length(seq, 2, name=name)
    return cast(tuple[_T, _T], tuple(seq))


def validate_tuple4(
    seq: Sequence[_T] | Array1D, name: str = ""
) -> tuple[_T, _T, _T, _T]:
    """Check sequence is of length 4 and return as a tuple.

    Raise
    -----
    ValueError
        The sequence is not one-dimensional of length 4.
    """
    _check_tuple_length(seq, 4, name=name)
    return cast(tuple[_T, _T, _T, _T], tuple(seq))
