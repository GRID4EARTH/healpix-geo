import numpy as np
import pytest

import healpix_geo


@pytest.mark.parametrize(
    ["depth", "new_depth", "indexing_scheme"],
    (
        pytest.param(1, 1, "nested", id="identity"),
        pytest.param(1, 0, "nested", id="parents-one step-base cells"),
        pytest.param(2, 0, "nested", id="parents-two step-base cells"),
        pytest.param(2, 1, "nested", id="parents-one step-normal"),
        pytest.param(1, 2, "nested", id="children-one step"),
        pytest.param(0, 2, "nested", id="children-two step-base cells"),
    ),
)
def test_zoom_to(depth, new_depth, indexing_scheme):
    cell_ids = np.arange(12 * 4**depth)
    if depth == new_depth:
        expected = cell_ids
    elif depth > new_depth:
        relative_depth = depth - new_depth
        expected = np.repeat(
            np.arange(12 * 4**new_depth),
            4**relative_depth,
        )
    elif depth < new_depth:
        expected = np.reshape(np.arange(12 * 4**new_depth), (cell_ids.size, -1))

    if indexing_scheme == "nested":
        zoom_to = healpix_geo.nested.zoom_to

    actual = zoom_to(cell_ids, depth, new_depth)

    np.testing.assert_equal(actual, expected)


@pytest.mark.parametrize(
    ["cell_ids", "depth", "indexing_scheme"],
    (
        pytest.param(
            np.arange(12 * 4, dtype="uint64"), 1, "nested", id="nested-normal"
        ),
        pytest.param(
            np.array([1, 15, 53, 67, 150], dtype="uint64"),
            2,
            "nested",
            id="nested-normal-subset",
        ),
        pytest.param(
            np.arange(12, dtype="uint64"), 0, "nested", id="nested-base cells"
        ),
        pytest.param(
            np.array([0, 4, 6, 11], dtype="uint64"),
            0,
            "nested",
            id="nested-base cells-subset",
        ),
    ),
)
def test_siblings(cell_ids, depth, indexing_scheme):

    if depth != 0:
        first = cell_ids // 4 * 4
        expected = first[:, None] + np.arange(4)
    else:
        expected = np.repeat(
            np.arange(12, dtype="uint64")[None, ...], cell_ids.size, axis=0
        )

    if indexing_scheme == "nested":
        siblings = healpix_geo.nested.siblings

    actual = siblings(cell_ids, depth)
    np.testing.assert_equal(actual, expected)
