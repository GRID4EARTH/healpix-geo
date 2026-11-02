from contextlib import nullcontext

import numpy as np
import pytest

from healpix_geo import validation


class TestCheckDepth:
    @pytest.mark.parametrize("depth", [0, 15, 29, [0], [29], np.array([0, 15, 29])])
    def test_pass(self, depth):
        validation.check_depth(depth)

    @pytest.mark.parametrize(
        "depth", [-1, -5, 30, 45, [-1], [5, 30], np.array([5, 30])]
    )
    def test_outside_range(self, depth):
        with pytest.raises(
            ValueError, match=r"Depth must be in the \[0, 29\] closed range"
        ):
            validation.check_depth(depth)


class TestCheckCellIds:
    @pytest.mark.parametrize(
        ["cell_ids", "depth"],
        [
            (0, 10),
            (0, np.array(10)),
            ([0], 10),
            ([0, 2, 11], 0),
            (np.array([14, 924, 483, 20132]), 10),
        ],
    )
    def test_pass_int_depth(self, cell_ids, depth):
        validation.check_cell_ids(cell_ids, depth)

    @pytest.mark.parametrize(
        ["cell_ids", "depth"],
        [
            (12, 0),
            (-1, 10),
            ([192], 2),
            ([5, 4000], 4),
            (np.array([5, 4000]), np.array(4)),
        ],
    )
    def test_outside_range(self, cell_ids, depth):
        with pytest.raises(
            ValueError, match="The input HEALPix cells contains a value out of"
        ):
            validation.check_cell_ids(cell_ids, depth)

    @pytest.mark.parametrize(
        "cell_ids",
        [0, 190, 16549809516, [190], np.array([0, 190, 16549809516])],
    )
    def test_pass_no_depth(self, cell_ids):
        validation.check_cell_ids(cell_ids)

    @pytest.mark.parametrize("cell_ids", [-1, -5, -68980, [-5], np.array([-1, 5])])
    def test_no_depth_negative(self, cell_ids):
        with pytest.raises(
            ValueError, match="The input HEALPix cells contains a negative value"
        ):
            validation.check_cell_ids(cell_ids)

    @pytest.mark.parametrize(
        ["cell_ids", "depth"],
        [(5, [0]), ([5, 30], [0, 2]), (np.array([[5], [30]]), np.array([[0], [2]]))],
    )
    def test_pass_array_depth(self, cell_ids, depth):
        validation.check_cell_ids(cell_ids, depth)

    @pytest.mark.parametrize(
        ["cell_ids", "depth"],
        [
            (12, [0]),
            ([12, 12], [2, 0]),
            (np.array([[12], [12]]), np.array([[2], [0]])),
        ],
    )
    def test_outside_range_array_depth(self, cell_ids, depth):
        with pytest.raises(
            ValueError, match="The input HEALPix cells contains a value out of"
        ):
            validation.check_cell_ids(cell_ids, depth)

    @pytest.mark.parametrize(
        ["cell_ids", "depth"],
        [
            ([0], [0, 1]),
            (0, [0, 1]),
            (np.array([0, 1]), [0, 1, 2]),
            (np.array([0, 1]), np.array([0, 1, 2])),
            (np.array([0, 1]), np.array([[0], [1]])),
        ],
    )
    def test_bad_depth_shape(self, cell_ids, depth):
        with pytest.raises(
            ValueError,
            match="An array valued depth must have the same shape as the cell ids",
        ):
            validation.check_cell_ids(cell_ids, depth)


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
        validation.check_ring(depth, ring)


class TestValidateDepth:
    @pytest.mark.parametrize(
        ["depth", "expected"],
        [
            (2, 2),
            ([2], np.array([2], dtype=np.uint8)),
            ([4, 6], np.array([4, 6], dtype=np.uint8)),
            ([[4], [6]], np.array([[4], [6]], dtype=np.uint8)),
            (np.array(2), 2),
            (np.array([4, 6]), np.array([4, 6], dtype=np.uint8)),
            (np.array([[4], [6]]), np.array([[4], [6]], dtype=np.uint8)),
        ],
    )
    def test_validate_depth(self, depth, expected):
        validated = validation.validate_depth(depth)
        if isinstance(expected, int):
            assert validated == expected
        else:
            assert isinstance(validated, np.ndarray)
            assert isinstance(expected, np.ndarray)
            assert validated.shape == expected.shape
            assert validated.dtype == expected.dtype
            np.testing.assert_array_equal(validated, expected)

    @pytest.mark.parametrize("depth", [-1, 30, 45, [-1], np.array(45)])
    def test_outside_range(self, depth):
        with pytest.raises(
            ValueError, match=r"Depth must be in the \[0, 29\] closed range"
        ):
            validation.check_depth(depth)


class TestValidateCellIds:
    @pytest.mark.parametrize(
        ["cell_ids", "depth", "expected"],
        [
            (641, 8, np.array([641], dtype=np.uint64)),
            ([641], 8, np.array([641], dtype=np.uint64)),
            ([641, 798], 8, np.array([641, 798], dtype=np.uint64)),
            ([[641], [798]], 8, np.array([[641], [798]], dtype=np.uint64)),
            (np.array(641), 8, np.array([641], dtype=np.uint64)),
            (np.array([641, 798]), 8, np.array([641, 798], dtype=np.uint64)),
            (np.array([[641], [798]]), 8, np.array([[641], [798]], dtype=np.uint64)),
            (np.array([190, 4000]), [2, 5], np.array([190, 4000], dtype=np.uint64)),
        ],
    )
    def test_basic(self, cell_ids, depth, expected):
        validated = validation.validate_cell_ids(cell_ids, depth)
        assert validated.shape == expected.shape
        assert validated.dtype == expected.dtype
        np.testing.assert_array_equal(validated, expected)

    @pytest.mark.parametrize(
        ["cell_ids", "depth"],
        [
            (-5, 2),
            (np.array([6, -1]), 2),
            (192, 2),
            (4000, 4),
            (np.array([4000]), 4),
            (np.array([100, 4000]), 4),
            (np.array([100, 4000]), np.array([2, 4])),
        ],
    )
    def test_outside_range(self, cell_ids, depth):
        with pytest.raises(
            ValueError, match="The input HEALPix cells contains a value out of"
        ):
            validation.validate_cell_ids(cell_ids, depth)

    @pytest.mark.parametrize(
        ["cell_ids", "expected"],
        [
            (0, np.array([0], dtype=np.uint64)),
            (5, np.array([5], dtype=np.uint64)),
            (1598061948, np.array([1598061948], dtype=np.uint64)),
        ],
    )
    def test_no_depth(self, cell_ids, expected):
        validated = validation.validate_cell_ids(cell_ids)
        assert validated.shape == expected.shape
        assert validated.dtype == expected.dtype
        np.testing.assert_array_equal(validated, expected)

    @pytest.mark.parametrize("cell_ids", [-1, -5, -98098])
    def test_no_depth_negative(self, cell_ids):
        with pytest.raises(
            ValueError, match="The input HEALPix cells contains a negative value."
        ):
            validation.validate_cell_ids(cell_ids)


class TestValidateTuple:
    @pytest.mark.parametrize("seq", [[0, 1], (0.5, 1.5), np.array([0, 1])])
    def test_pass_2(self, seq):
        validated = validation.validate_tuple2(seq)
        assert isinstance(validated, tuple)
        assert len(validated) == 2

    @pytest.mark.parametrize(
        "seq", [[0, 1, 2, 3], (0.5, 1.5, 2.5, 3.5), np.array([0, 1, 2, 3])]
    )
    def test_pass_4(self, seq):
        validated = validation.validate_tuple4(seq)
        assert isinstance(validated, tuple)
        assert len(validated) == 4

    @pytest.mark.parametrize("seq", [[0], (0.5, 1.5, 2.5), np.array([0, 1, 2, 3])])
    def test_bad_2(self, seq):
        with pytest.raises(ValueError, match="Expected tuple of length 2"):
            validation.validate_tuple2(seq)

    @pytest.mark.parametrize("seq", [[0], (0.5, 1.5, 2.5), np.array([0, 1])])
    def test_bad_4(self, seq):
        with pytest.raises(ValueError, match="Expected tuple of length 4"):
            validation.validate_tuple4(seq)


class TestCheckSameShape:
    @pytest.mark.parametrize(
        "arrays",
        [
            (np.array(0),),
            (np.ones(8),),
            (np.array(0), np.array(8)),
            (np.ones(5), np.ones(5)),
            (np.ones((2, 3)), np.ones((2, 3)), np.ones((2, 3))),
        ],
    )
    def test_pass(self, arrays):
        # all as args
        validation.check_same_shape(*arrays)

        # all as kwargs
        n = len(arrays)
        names = [chr(97 + i) for i in range(n)]
        validation.check_same_shape(**dict(zip(names, arrays)))

        # half as kwargs
        validation.check_same_shape(
            *arrays[: n // 2], **dict(zip(names[n // 2 :], arrays[n // 2 :]))
        )

    @pytest.mark.parametrize(
        "arrays",
        [
            (np.array(0), np.ones(1)),
            (np.array(0), np.ones(2)),
            (np.ones(1), np.ones(2)),
            (np.ones((2, 3)), np.ones((2, 3)), np.ones((2, 1))),
        ],
    )
    def test_fail(self, arrays):
        match = "All arrays must have the same shape."
        # all as args
        with pytest.raises(ValueError, match=match):
            validation.check_same_shape(*arrays)

        # all as kwargs
        n = len(arrays)
        names = [chr(97 + i) for i in range(n)]
        with pytest.raises(ValueError, match=match):
            validation.check_same_shape(**dict(zip(names, arrays)))

        # half as kwargs
        with pytest.raises(ValueError, match=match):
            validation.check_same_shape(
                *arrays[: n // 2], **dict(zip(names[n // 2 :], arrays[n // 2 :]))
            )
