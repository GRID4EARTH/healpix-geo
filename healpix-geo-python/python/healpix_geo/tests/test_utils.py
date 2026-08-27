from contextlib import nullcontext

import numpy as np
import pytest

from healpix_geo import utils


@pytest.mark.parametrize(
    ["depth", "context"],
    (
        pytest.param(0, nullcontext(), id="depth-0"),
        pytest.param(15, nullcontext(), id="depth-15"),
        pytest.param(29, nullcontext(), id="depth-29"),
        pytest.param(
            -4,
            pytest.raises(
                ValueError, match=r"Depth must be in the \[0, 29\] closed range"
            ),
            id="depth--4",
        ),
        pytest.param(
            45,
            pytest.raises(
                ValueError, match=r"Depth must be in the \[0, 29\] closed range"
            ),
            id="depth-45",
        ),
    ),
)
def test_check_depth(depth, context):
    with context:
        utils._check_depth(depth)


@pytest.mark.parametrize(
    ["data", "depth", "context"],
    (
        pytest.param(
            np.array([-1, 5, 7], dtype=np.int64),
            0,
            pytest.raises(
                ValueError, match="The input HEALPix cells contains a value out of"
            ),
            id="negative-value",
        ),
        pytest.param(
            np.array([4, 48], dtype=np.uint64),
            1,
            pytest.raises(
                ValueError, match="The input HEALPix cells contains a value out of"
            ),
            id="exceeding-value",
        ),
        pytest.param(
            np.array([14, 924, 483, 20132]),
            np.array(10, dtype=np.uint8),
            nullcontext(),
            id="uint8-depth",
        ),
        pytest.param(
            np.array([58, 12]),
            np.array([2, 1]),
            nullcontext(),
            id="variable-depth",
        ),
    ),
)
def test_check_ipixels(data, depth, context):
    with context:
        utils._check_ipixels(data, depth)


@pytest.mark.parametrize(
    ["depth", "ring", "context"],
    (
        pytest.param(2, 4, nullcontext(), id="passing-int"),
        pytest.param(
            np.array(10, dtype=np.uint8), 43, nullcontext(), id="passing-uint8"
        ),
        pytest.param(
            1,
            3,
            pytest.raises(
                ValueError,
                match="Crossing base cell boundaries more than once is not supported",
            ),
            id="failing-int",
        ),
    ),
)
def test_check_ring(depth, ring, context):
    with context:
        utils._check_ring(depth, ring)
