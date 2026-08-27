import numpy as np
import pytest

import healpix_geo


@pytest.mark.parametrize(
    ["depth", "indexing_scheme", "from_", "to_", "expected"],
    (
        (
            1,
            "nested",
            np.array([0, 16, 25, 32, 46]),
            np.array([2, 15, 27, 40, 41], dtype="int64"),
            np.array(
                [0.51262797, 1.60992678, 0.51347673, 0.82227572, 0.57850402],
                dtype="float64",
            ),
        ),
        (
            1,
            "ring",
            np.array([0, 16, 25, 32, 46]),
            np.array([2, 15, 27, 40, 41], dtype="int64"),
            np.array(
                [0.82227572, 0.73824548, 1.57079633, 0.51262797, 0.48146047],
                dtype="float64",
            ),
        ),
        (
            1,
            "nested",
            np.array([0, 16, 25, 32, 46]),
            np.array([[2], [15], [27], [40], [41]], dtype="int64"),
            np.array(
                [[0.51262797], [1.60992678], [0.51347673], [0.82227572], [0.57850402]],
                dtype="float64",
            ),
        ),
        (
            2,
            "ring",
            np.array([0, 16, 25, 32, 46]),
            np.array([[2, 4], [15, 7], [27, 26], [40, -1], [-1, 41]], dtype="int64"),
            np.array(
                [
                    [0.4089604, 0.23486912],
                    [0.30291976, 0.283655],
                    [0.57850402, 0.29185825],
                    [1.87523829, np.nan],
                    [np.nan, 1.60781736],
                ],
                dtype="float64",
            ),
        ),
    ),
)
def test_distance(depth, indexing_scheme, from_, to_, expected):
    if indexing_scheme == "nested":
        angular_distances = healpix_geo.nested.angular_distances
    elif indexing_scheme == "ring":
        angular_distances = healpix_geo.ring.angular_distances

    actual = angular_distances(from_, to_, depth)

    np.testing.assert_allclose(actual, expected)


@pytest.mark.parametrize("indexing_scheme", ["ring", "nested"])
def test_distance_error(indexing_scheme):
    if indexing_scheme == "nested":
        angular_distances = healpix_geo.nested.angular_distances
    elif indexing_scheme == "ring":
        angular_distances = healpix_geo.ring.angular_distances

    from_ = np.array([4, 7])
    to_ = np.array([[2, 3], [4, 6], [5, 4]])
    depth = 1

    with pytest.raises(ValueError, match="The shape of `from_` must be compatible"):
        angular_distances(from_, to_, depth)
