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
