import numpy as np
import pytest

from healpix_geo import RaggedArray


@pytest.mark.parametrize(
    "offsets",
    (
        pytest.param(np.array([0, 3, 4, 6], dtype="int64"), id="int64"),
        pytest.param(np.array([0, 3, 4, 6], dtype="uint64"), id="uint64"),
        pytest.param(np.array([0, 3, 4, 6], dtype="int8"), id="int8"),
    ),
)
def test_init(offsets):
    data = np.linspace(0, 1, 7, dtype="float16")

    obj = RaggedArray(offsets, data)

    # ragged array is a container and thus must not copy for uint64
    assert offsets.dtype != np.dtype("uint64") or obj.offsets is offsets
    assert obj.data is data
    assert obj.dtype == data.dtype
    assert obj.shape == (3, 3)
    assert obj.ndim == 2


@pytest.mark.parametrize(
    ["offsets", "message"],
    (
        pytest.param(
            np.array([[0, 1], [2, 3]], dtype="int64"),
            "offsets must be 1-dimensional",
            id="2-d",
        ),
        pytest.param(
            np.array([0.1, 0.2], dtype="float16"),
            "offsets must be of integer dtype",
            id="float",
        ),
    ),
)
def test_init_errors(offsets, message):
    data = np.arange(5, dtype="int8")

    with pytest.raises(ValueError, match=message):
        RaggedArray(offsets, data)


@pytest.mark.parametrize(
    ["func", "expected"],
    (
        pytest.param(
            lambda x: x * 10,
            np.linspace(0, 10, 7, dtype="float32"),
            id="scalar product",
        ),
        pytest.param(
            lambda x: x + 1, np.linspace(1, 2, 7, dtype="float32"), id="scalar addition"
        ),
    ),
)
def test_apply_elementwise(func, expected):
    offsets = np.array([0, 3, 4, 6], dtype="uint64")
    data = np.linspace(0, 1, 7, dtype="float32")
    obj = RaggedArray(offsets, data)

    actual = obj.apply_elementwise(func)

    np.testing.assert_allclose(actual.data, expected)
    assert actual.offsets is obj.offsets


def test_as_awkward():
    ak = pytest.importorskip("awkward")

    offsets = np.array([0, 3, 4, 6], dtype="uint64")
    data = np.linspace(0, 1, 7, dtype="float32")
    obj = RaggedArray(offsets, data)

    actual = obj.as_awkward()
    expected = ak.Array(
        [data[start:stop] for start, stop in zip(offsets[:-1], offsets[1:])]
    )

    assert ak.to_list(actual) == ak.to_list(expected)


def test_as_ragged():
    ragged = pytest.importorskip("ragged")

    offsets = np.array([0, 3, 4, 6], dtype="uint64")
    data = np.linspace(0, 1, 7, dtype="float32")
    obj = RaggedArray(offsets, data)

    actual = obj.as_ragged()
    expected = ragged.array(
        [data[start:stop] for start, stop in zip(offsets[:-1], offsets[1:])]
    )

    assert ragged.all(actual == expected)


def test_as_masked_array():
    offsets = np.array([0, 3, 4, 6], dtype="uint64")
    data = np.arange(7, dtype="int64")
    obj = RaggedArray(offsets, data)

    actual = obj.as_masked_array()
    expected_data = np.array([[0, 1, 2], [3, 0, 0], [4, 5, 0]], dtype="int64")
    expected_mask = np.array(
        [[False, False, False], [False, True, True], [False, False, True]], dtype="bool"
    )

    np.testing.assert_equal(actual.data, expected_data)
    np.testing.assert_equal(actual.mask, expected_mask)


@pytest.mark.parametrize(
    "data",
    (
        pytest.param(np.linspace(0, 1, 7, dtype="float16"), id="float16"),
        pytest.param(np.arange(7, dtype="complex64"), id="complex64"),
    ),
)
def test_as_masked_array_errors(data):
    offsets = np.array([0, 3, 4, 6], dtype="uint64")

    obj = RaggedArray(offsets, data)
    with pytest.raises(TypeError, match="Unsupported data dtype"):
        obj.as_masked_array()
