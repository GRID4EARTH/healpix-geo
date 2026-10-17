import numpy as np
import pytest

from healpix_geo import RaggedArray, nested


@pytest.mark.parametrize("ellipsoid", ["sphere", "WGS84"])
@pytest.mark.parametrize("flat", [False, True])
@pytest.mark.parametrize("num_threads", [0, 1, 4, 64])
@pytest.mark.parametrize("delta_depth", [0, 1])
def test_batched_cones_equal_single_cones(ellipsoid, flat, num_threads, delta_depth):
    centers = np.array(
        [[45, 45], [179.999, 0], [-179.999, -30], [12, 89.9], [40, -89.9]]
    )
    result = nested.cone_coverage(
        centers,
        0.5,
        8,
        ellipsoid=ellipsoid,
        flat=flat,
        delta_depth=delta_depth,
        num_threads=num_threads,
    )
    assert result[0].offsets is result[1].offsets is result[2].offsets
    for i, center in enumerate(centers):
        expected = nested.cone_coverage(
            center, 0.5, 8, ellipsoid=ellipsoid, flat=flat, delta_depth=delta_depth
        )
        for actual, reference in zip(result, expected, strict=True):
            start, stop = actual.offsets[i : i + 2]
            np.testing.assert_array_equal(actual.data[start:stop], reference.data)
            assert actual.data.flags.c_contiguous


@pytest.mark.parametrize(
    ["center", "rows"],
    (
        pytest.param((45, 45), 1, id="tuple"),
        pytest.param([45, 45], 1, id="list"),
        pytest.param(np.array([45, 45]), 1, id="1d-array"),
        pytest.param(np.zeros((1, 2)), 1, id="one-row"),
        pytest.param(np.zeros((0, 2)), 0, id="empty"),
    ),
)
def test_always_ragged(center, rows):
    result = nested.cone_coverage(center, 0.5, 8)
    for array in result:
        assert isinstance(array, RaggedArray)
        assert array.offsets.shape == (rows + 1,)
        assert array.offsets[0] == 0
        assert array.offsets[-1] == array.data.size


def test_single_center_is_one_row():
    actual = nested.cone_coverage((45.0, 45.0), 0.5, 8)
    expected = nested.cone_coverage([[45.0, 45.0]], 0.5, 8)
    for a, b in zip(actual, expected, strict=True):
        np.testing.assert_array_equal(a.data, b.data)
        np.testing.assert_array_equal(a.offsets, b.offsets)


def test_noncontiguous_centers():
    centers = np.array([[45.0, 0.0, 45.0, 0.0], [12.0, 0.0, 89.0, 0.0]])[:, ::2]
    assert not centers.flags.c_contiguous
    actual = nested.cone_coverage(centers, 0.5, 8)
    expected = nested.cone_coverage(centers.copy(), 0.5, 8)
    for a, b in zip(actual, expected, strict=True):
        np.testing.assert_array_equal(a.data, b.data)
        np.testing.assert_array_equal(a.offsets, b.offsets)


@pytest.mark.parametrize(
    "centers", [[], [1], [1, 2, 3], np.empty((2, 3)), np.empty((2, 2, 1))]
)
def test_invalid_shape(centers):
    with pytest.raises(ValueError, match="shape"):
        nested.cone_coverage(centers, 0.5, 8)


@pytest.mark.parametrize("center", [[45.0, 45.0], [[45.0, 45.0]]])
def test_invalid_combined_depth(center):
    with pytest.raises(ValueError, match="delta_depth"):
        nested.cone_coverage(center, 0.5, 8, delta_depth=255)
