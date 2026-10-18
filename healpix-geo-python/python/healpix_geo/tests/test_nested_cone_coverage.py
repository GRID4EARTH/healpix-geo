from contextlib import nullcontext

import numpy as np
import pytest

from healpix_geo import RaggedArray, nested

CENTERS = np.array(
    [[45, 45], [179.999, 0], [-179.999, -30], [12, 89.9], [40, -89.9]],
    dtype="float64",
)


class TestConeCoverage:
    @pytest.mark.parametrize(
        "row",
        (
            pytest.param(0, id="mid-latitude"),
            pytest.param(1, id="antimeridian-east"),
            pytest.param(2, id="antimeridian-west"),
            pytest.param(3, id="north-pole"),
            pytest.param(4, id="south-pole"),
        ),
    )
    @pytest.mark.parametrize("ellipsoid", ["sphere", "WGS84"])
    @pytest.mark.parametrize("flat", [False, True])
    @pytest.mark.parametrize("num_threads", [0, 1, 4, 64])
    @pytest.mark.parametrize("delta_depth", [0, 1])
    def test_rows_equal_single_centers(
        self, row, ellipsoid, flat, num_threads, delta_depth
    ):
        kwargs = {"ellipsoid": ellipsoid, "flat": flat, "delta_depth": delta_depth}

        cell_ids, depths, fully_covered = nested.cone_coverage(
            CENTERS, 0.5, 8, num_threads=num_threads, **kwargs
        )
        expected_cell_ids, expected_depths, expected_fully_covered = (
            nested.cone_coverage(CENTERS[row], 0.5, 8, **kwargs)
        )

        assert cell_ids.offsets is depths.offsets is fully_covered.offsets
        start, stop = cell_ids.offsets[row : row + 2]
        np.testing.assert_array_equal(cell_ids.data[start:stop], expected_cell_ids.data)
        np.testing.assert_array_equal(depths.data[start:stop], expected_depths.data)
        np.testing.assert_array_equal(
            fully_covered.data[start:stop], expected_fully_covered.data
        )

    @pytest.mark.parametrize(
        ["center", "rows"],
        (
            pytest.param((45, 45), 1, id="tuple"),
            pytest.param([45, 45], 1, id="list"),
            pytest.param(np.array([45, 45]), 1, id="1d-array"),
            pytest.param(np.zeros((1, 2)), 1, id="one-row"),
            pytest.param(np.zeros((0, 2)), 0, id="empty"),
            pytest.param(CENTERS, 5, id="many"),
        ),
    )
    def test_return_type(self, center, rows):
        cell_ids, depths, fully_covered = nested.cone_coverage(center, 0.5, 8)

        assert isinstance(cell_ids, RaggedArray)
        assert isinstance(depths, RaggedArray)
        assert isinstance(fully_covered, RaggedArray)
        assert cell_ids.offsets is depths.offsets is fully_covered.offsets
        assert cell_ids.offsets.shape == (rows + 1,)
        assert cell_ids.offsets[0] == 0
        assert cell_ids.offsets[-1] == cell_ids.data.size == depths.data.size
        assert fully_covered.data.size == cell_ids.data.size

    @pytest.mark.parametrize(
        ["center", "expected"],
        (
            pytest.param((45.0, 45.0), [[45.0, 45.0]], id="single-center"),
            pytest.param(
                np.array([[45.0, 0.0, 12.0, 0.0], [12.0, 0.0, 89.0, 0.0]])[:, ::2],
                [[45.0, 12.0], [12.0, 89.0]],
                id="non-contiguous",
            ),
        ),
    )
    def test_equivalent_inputs(self, center, expected):
        actual_cell_ids, actual_depths, _ = nested.cone_coverage(center, 0.5, 8)
        expected_cell_ids, expected_depths, _ = nested.cone_coverage(
            np.array(expected), 0.5, 8
        )

        np.testing.assert_array_equal(
            actual_cell_ids.offsets, expected_cell_ids.offsets
        )
        np.testing.assert_array_equal(actual_cell_ids.data, expected_cell_ids.data)
        np.testing.assert_array_equal(actual_depths.data, expected_depths.data)

    @pytest.mark.parametrize(
        ["center", "delta_depth", "context"],
        (
            pytest.param([45.0, 45.0], 0, nullcontext(), id="single-center"),
            pytest.param([[45.0, 45.0]], 0, nullcontext(), id="one-row"),
            pytest.param(
                [], 0, pytest.raises(ValueError, match="shape"), id="shape-empty"
            ),
            pytest.param(
                [1], 0, pytest.raises(ValueError, match="shape"), id="shape-1"
            ),
            pytest.param(
                [1, 2, 3], 0, pytest.raises(ValueError, match="shape"), id="shape-3"
            ),
            pytest.param(
                np.empty((2, 3)),
                0,
                pytest.raises(ValueError, match="shape"),
                id="shape-2x3",
            ),
            pytest.param(
                np.empty((2, 2, 1)),
                0,
                pytest.raises(ValueError, match="shape"),
                id="shape-3d",
            ),
            pytest.param(
                [45.0, 45.0],
                255,
                pytest.raises(ValueError, match="delta_depth"),
                id="delta-depth-overflow-single",
            ),
            pytest.param(
                [[45.0, 45.0]],
                255,
                pytest.raises(ValueError, match="delta_depth"),
                id="delta-depth-overflow-batch",
            ),
        ),
    )
    def test_validation(self, center, delta_depth, context):
        with context:
            nested.cone_coverage(center, 0.5, 8, delta_depth=delta_depth)
