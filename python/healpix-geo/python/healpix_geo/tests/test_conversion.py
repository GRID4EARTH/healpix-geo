import numpy as np
import pytest

import healpix_geo


class TestZuniq:
    @pytest.mark.parametrize(
        ["cell_ids", "expected_cell_ids", "expected_depths"],
        (
            (
                np.array(
                    [2017612633061982208, 1801439850948198400, 1747396655419752448],
                    dtype="uint64",
                ),
                np.array([3, 12, 48]),
                np.array([0, 1, 2]),
            ),
            (
                np.array(
                    [485262859849170944, 519039857054449664, 552816854259728384],
                    dtype="uint64",
                ),
                np.array([215, 230, 245]),
                np.array([4, 4, 4]),
            ),
            (
                np.array(
                    [
                        [2017612633061982208],
                        [1801439850948198400],
                        [1747396655419752448],
                    ],
                    dtype="uint64",
                ),
                np.array([[3], [12], [48]]),
                np.array([[0], [1], [2]]),
            ),
            (
                np.array(
                    [[[485262859849170944, 519039857054449664, 552816854259728384]]],
                    dtype="uint64",
                ),
                np.array([[[215, 230, 245]]]),
                np.array([[[4, 4, 4]]]),
            ),
        ),
    )
    def test_to_nested(self, cell_ids, expected_cell_ids, expected_depths):
        actual_cell_ids, actual_depths = healpix_geo.zuniq.to_nested(cell_ids)

        np.testing.assert_equal(actual_cell_ids, expected_cell_ids)
        np.testing.assert_equal(actual_depths, expected_depths)

    @pytest.mark.parametrize(
        ["cell_ids", "expected_cell_ids", "expected_depths"],
        (
            (
                np.array(
                    [2017612633061982208, 2810246167479189504, 4017210867614482432],
                    dtype="uint64",
                ),
                np.array([3, 12, 48]),
                np.array([0, 1, 2]),
            ),
            (
                np.array(
                    [2192127118622588928, 269090077735387136, 1658450562779185152],
                    dtype="uint64",
                ),
                np.array([215, 230, 245]),
                np.array([4, 4, 4]),
            ),
            (
                np.array(
                    [
                        [2017612633061982208],
                        [2810246167479189504],
                        [4017210867614482432],
                    ],
                    dtype="uint64",
                ),
                np.array([[3], [12], [48]]),
                np.array([[0], [1], [2]]),
            ),
            (
                np.array(
                    [[[2192127118622588928, 269090077735387136, 1658450562779185152]]],
                    dtype="uint64",
                ),
                np.array([[[215, 230, 245]]]),
                np.array([[[4, 4, 4]]]),
            ),
        ),
    )
    def test_to_ring(self, cell_ids, expected_cell_ids, expected_depths):
        actual_cell_ids, actual_depths = healpix_geo.zuniq.to_ring(cell_ids)

        np.testing.assert_equal(actual_cell_ids, expected_cell_ids)
        np.testing.assert_equal(actual_depths, expected_depths)


class TestNested:
    @pytest.mark.parametrize(
        ["cell_ids", "depths", "expected_cell_ids"],
        (
            (
                np.array([3, 19, 111], dtype="uint64"),
                np.array([0, 1, 2], dtype="uint8"),
                np.array([3, 12, 48]),
            ),
            (
                np.array([973, 119, 736], dtype="uint64"),
                4,
                np.array([215, 230, 245]),
            ),
            (
                np.array([[3], [19], [111]], dtype="uint64"),
                np.array([[0], [1], [2]], dtype="uint8"),
                np.array([[3], [12], [48]]),
            ),
            (
                np.array([[[973, 119, 736]]], dtype="uint64"),
                np.array([[[4, 4, 4]]], dtype="uint8"),
                np.array([[[215, 230, 245]]]),
            ),
        ),
    )
    def test_to_ring(self, cell_ids, depths, expected_cell_ids):
        actual_cell_ids = healpix_geo.nested.to_ring(cell_ids, depths)

        np.testing.assert_equal(actual_cell_ids, expected_cell_ids)

    @pytest.mark.parametrize(
        ["cell_ids", "depths", "expected"],
        (
            (
                np.array([3, 12, 48]),
                np.array([0, 1, 2]),
                np.array(
                    [2017612633061982208, 1801439850948198400, 1747396655419752448],
                    dtype="uint64",
                ),
            ),
            (
                np.array([215, 230, 245]),
                np.array([4, 4, 4]),
                np.array(
                    [485262859849170944, 519039857054449664, 552816854259728384],
                    dtype="uint64",
                ),
            ),
            (
                np.array([[3], [12], [48]]),
                np.array([[0], [1], [2]]),
                np.array(
                    [
                        [2017612633061982208],
                        [1801439850948198400],
                        [1747396655419752448],
                    ],
                    dtype="uint64",
                ),
            ),
            (
                np.array([[[215, 230, 245]]]),
                np.array([[[4, 4, 4]]]),
                np.array(
                    [[[485262859849170944, 519039857054449664, 552816854259728384]]],
                    dtype="uint64",
                ),
            ),
        ),
    )
    def test_to_zuniq(self, cell_ids, depths, expected):
        actual = healpix_geo.nested.to_zuniq(cell_ids, depths)

        np.testing.assert_equal(actual, expected)


class TestRing:
    @pytest.mark.parametrize(
        ["cell_ids", "depths", "expected_cell_ids"],
        (
            (
                np.array([3, 19, 86], dtype="uint64"),
                np.array([0, 1, 2], dtype="uint8"),
                np.array([3, 12, 48]),
            ),
            (
                np.array([90, 114, 27], dtype="uint64"),
                4,
                np.array([215, 230, 245]),
            ),
            (
                np.array([[3], [19], [86]], dtype="uint64"),
                np.array([[0], [1], [2]], dtype="uint8"),
                np.array([[3], [12], [48]]),
            ),
            (
                np.array([[[90, 114, 27]]], dtype="uint64"),
                np.array([[[4, 4, 4]]], dtype="uint8"),
                np.array([[[215, 230, 245]]]),
            ),
        ),
    )
    def test_to_nested(self, cell_ids, depths, expected_cell_ids):
        actual_cell_ids = healpix_geo.ring.to_nested(cell_ids, depths)

        np.testing.assert_equal(actual_cell_ids, expected_cell_ids)

    @pytest.mark.parametrize(
        ["cell_ids", "depths", "expected"],
        (
            (
                np.array([3, 19, 86]),
                np.array([0, 1, 2]),
                np.array(
                    [2017612633061982208, 1801439850948198400, 1747396655419752448],
                    dtype="uint64",
                ),
            ),
            (
                np.array([90, 114, 27]),
                np.array([4, 4, 4]),
                np.array(
                    [485262859849170944, 519039857054449664, 552816854259728384],
                    dtype="uint64",
                ),
            ),
            (
                np.array([[3], [19], [86]]),
                np.array([[0], [1], [2]]),
                np.array(
                    [
                        [2017612633061982208],
                        [1801439850948198400],
                        [1747396655419752448],
                    ],
                    dtype="uint64",
                ),
            ),
            (
                np.array([[[90, 114, 27]]]),
                np.array([[[4, 4, 4]]]),
                np.array(
                    [[[485262859849170944, 519039857054449664, 552816854259728384]]],
                    dtype="uint64",
                ),
            ),
        ),
    )
    def test_to_zuniq(self, cell_ids, depths, expected):
        actual = healpix_geo.ring.to_zuniq(cell_ids, depths)

        np.testing.assert_equal(actual, expected)
