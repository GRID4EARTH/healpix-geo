import numpy as np


def _check_depth(depth):
    ravel_depth = np.ravel(np.atleast_1d(depth))
    if any(ravel_depth < 0) or any(ravel_depth > 29):
        raise ValueError("Depth must be in the [0, 29] closed range")


def _check_ipixels(data, depth):
    npix = 12 * 4 ** np.asarray(depth, dtype=np.uint64)
    if (data >= npix).any() or (data < 0).any():
        raise ValueError(
            f"The input HEALPix cells contains a value out of [0, {npix - 1}]"
        )


def _check_ring(depth, ring):
    nside = 2 ** np.asarray(depth, dtype=np.uint64)

    if ring > nside:
        raise ValueError(
            "Crossing base cell boundaries more than once is not supported."
            f" Received ring={ring}, but expected an integer in the range of [0, {nside}]."
        )


def _prepare_distances(from_, to_):
    """Normalise the cell ids passed to the distance functions.

    Returns ``from_`` as a 1D ``uint64`` array, ``to_`` as a 2D ``uint64``
    array with one row per entry of ``from_`` (padding values of ``-1``
    replaced by the source cell), the mask of valid entries and the shape of
    the result.
    """
    from_ = np.atleast_1d(from_)
    to_ = np.atleast_1d(to_)

    output_shape = to_.shape
    if from_.shape == to_.shape:
        to_ = to_[..., np.newaxis]
    elif from_.shape != to_.shape[:-1]:
        raise ValueError(
            "The shape of `from_` must be compatible with the shape of `to_`:\n"
            f"{to_.shape} or {to_.shape[:-1]} must be equal to {from_.shape}."
        )

    mask = to_ != -1
    from_ = from_.astype("uint64")
    to_ = np.where(mask, to_.astype("uint64"), from_[..., np.newaxis])

    return (
        from_.reshape(-1),
        to_.reshape(from_.size, to_.shape[-1]),
        mask.reshape(output_shape),
        output_shape,
    )
