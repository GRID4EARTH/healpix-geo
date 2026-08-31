import numpy as np
import pytest

from healpix_geo import nested, topology

DIRECTIONS = ("N", "NE", "E", "SE", "S", "SW", "W", "NW")
BASE_CELL_NEIGHBOURS = (
    (2, 1, None, 5, 8, 4, None, 3),
    (3, 2, None, 6, 9, 5, None, 0),
    (0, 3, None, 7, 10, 6, None, 1),
    (1, 0, None, 4, 11, 7, None, 2),
    (None, 0, 5, 8, None, 11, 7, 3),
    (None, 1, 6, 9, None, 8, 4, 0),
    (None, 2, 7, 10, None, 9, 5, 1),
    (None, 3, 4, 11, None, 10, 6, 2),
    (0, 5, None, 9, 10, 11, None, 4),
    (1, 6, None, 10, 11, 8, None, 5),
    (2, 7, None, 11, 8, 9, None, 6),
    (3, 4, None, 8, 9, 10, None, 7),
)


def generate_expected_base_cells():
    return [
        pytest.param(base_cell, direction, expected, id=f"{base_cell}-{direction}")
        for base_cell, neighbours in enumerate(BASE_CELL_NEIGHBOURS)
        for direction, expected in zip(DIRECTIONS, neighbours)
    ]


class TestBaseCellRelationship:
    @pytest.mark.parametrize(
        ["base_cell", "direction", "expected_base_cell"], generate_expected_base_cells()
    )
    def test_all_cells_and_directions(self, base_cell, direction, expected_base_cell):
        neighbour, delta_i, delta_j = topology.base_cell_relationship(
            base_cell, direction
        )

        assert neighbour == expected_base_cell

    @pytest.mark.parametrize(
        ["base_cell", "direction", "expected_delta_i", "expected_delta_j"],
        (
            (0, "N", np.array([-1, 0], dtype="int8"), np.array([0, -1], dtype="int8")),
            (0, "NE", np.array([0, -1], dtype="int8"), np.array([1, 0], dtype="int8")),
            (4, "E", np.array([1, 0], dtype="int8"), np.array([0, 1], dtype="int8")),
            (8, "SE", np.array([0, 1], dtype="int8"), np.array([-1, 0], dtype="int8")),
            (8, "S", np.array([-1, 0], dtype="int8"), np.array([0, -1], dtype="int8")),
        ),
    )
    def test_base_cell_relationship_orientation_cases(
        self, base_cell, direction, expected_delta_i, expected_delta_j
    ):
        # already checked the neighbour
        _, actual_delta_i, actual_delta_j = topology.base_cell_relationship(
            base_cell, direction
        )

        np.testing.assert_equal(actual_delta_i, expected_delta_i)
        np.testing.assert_equal(actual_delta_j, expected_delta_j)

    @pytest.mark.parametrize("base_cell", [-1, 12, 256])
    def test_base_cell_relationship_rejects_invalid_base_cell(self, base_cell):
        with pytest.raises(ValueError, match="base_cell"):
            topology.base_cell_relationship(base_cell, "N")

    @pytest.mark.parametrize("base_cell", ["0", 0.0, [0]])
    def test_face_neighbour_transform_rejects_non_integer_base_cell(self, base_cell):
        with pytest.raises(TypeError, match="base_cell must be an integer"):
            topology.base_cell_relationship(base_cell, "N")

    def test_base_cell_relationship_rejects_invalid_direction(self):
        with pytest.raises(ValueError, match="direction"):
            topology.base_cell_relationship(0, "up")
        with pytest.raises(TypeError, match="direction"):
            topology.base_cell_relationship(0, 0)


@pytest.mark.parametrize("depth", range(6))
def test_exhaustive_small_depth_round_trip(depth):
    pixels = np.arange(12 * 4**depth, dtype=np.uint64)

    face, x, y = nested.healpix_to_base_cell_coordinates(pixels, depth)

    np.testing.assert_array_equal(
        nested.base_cell_coordinates_to_healpix(face, x, y, depth), pixels
    )
    actual_face, actual_x, actual_y = nested.healpix_to_base_cell_coordinates(
        nested.base_cell_coordinates_to_healpix(face, x, y, depth), depth
    )
    np.testing.assert_array_equal(actual_face, face)
    np.testing.assert_array_equal(actual_x, x)
    np.testing.assert_array_equal(actual_y, y)


def test_level_zero_is_the_base_face():
    pixels = np.arange(12, dtype=np.uint64)

    face, x, y = nested.healpix_to_base_cell_coordinates(pixels, 0)

    np.testing.assert_array_equal(face, np.arange(12, dtype=np.uint8))
    np.testing.assert_array_equal(x, np.zeros(12, dtype=np.uint32))
    np.testing.assert_array_equal(y, np.zeros(12, dtype=np.uint32))


def test_scalar_inputs_return_length_one_arrays():
    face, x, y = nested.healpix_to_base_cell_coordinates(47, 1)

    np.testing.assert_array_equal(face, np.array([11], dtype=np.uint8))
    np.testing.assert_array_equal(x, np.array([1], dtype=np.uint32))
    np.testing.assert_array_equal(y, np.array([1], dtype=np.uint32))
    np.testing.assert_array_equal(
        nested.base_cell_coordinates_to_healpix(11, 1, 1, 1),
        np.array([47], dtype=np.uint64),
    )


def test_multidimensional_shape_and_dtypes_are_preserved():
    pixels = np.arange(48, dtype=np.uint64).reshape(2, 3, 8)

    face, x, y = nested.healpix_to_base_cell_coordinates(pixels, 1)

    assert face.shape == pixels.shape
    assert x.shape == pixels.shape
    assert y.shape == pixels.shape
    assert face.dtype == np.uint8
    assert x.dtype == np.uint32
    assert y.dtype == np.uint32
    actual = nested.base_cell_coordinates_to_healpix(face, x, y, 1)
    assert actual.shape == pixels.shape
    assert actual.dtype == np.uint64
    np.testing.assert_array_equal(actual, pixels)


@pytest.mark.parametrize(
    ("base_cell", "i", "j", "depth", "message"),
    [
        (-1, 0, 0, 1, "Base cell"),
        (12, 0, 0, 1, "Base cell"),
        (0, -1, 0, 1, "i"),
        (0, 2, 0, 1, "i"),
        (0, 0, -1, 1, "j"),
        (0, 0, 2, 1, "j"),
    ],
)
def test_base_cell_coordinates_to_healpix_rejects_invalid_coordinates(
    base_cell, i, j, depth, message
):
    with pytest.raises(ValueError, match=message):
        nested.base_cell_coordinates_to_healpix(base_cell, i, j, depth)


@pytest.mark.parametrize("depth", [-1, 30])
def test_topology_rejects_invalid_depth(depth):
    with pytest.raises(ValueError, match="Depth"):
        nested.healpix_to_base_cell_coordinates([0], depth)
    with pytest.raises(ValueError, match="Depth"):
        nested.base_cell_coordinates_to_healpix([0], [0], [0], depth)


def test_healpix_to_base_cell_coordinates_rejects_invalid_cell_id():
    with pytest.raises(ValueError, match="out of"):
        nested.healpix_to_base_cell_coordinates([48], 1)


def test_matches_healpy_reference():
    healpy = pytest.importorskip("healpy")
    rng = np.random.default_rng(234243)

    for depth in (0, 1, 2, 7, 14, 29):
        nside = 2**depth
        n_cells = 12 * nside**2
        cell_ids = rng.integers(0, n_cells, size=256, dtype=np.uint64)

        base_cell, i, j = nested.healpix_to_base_cell_coordinates(cell_ids, depth)
        expected_i, expected_j, expected_base_cell = (
            healpy.healpix_to_base_cell_coordinates(
                nside, cell_ids.astype(np.int64), nest=True
            )
        )

        np.testing.assert_array_equal(base_cell, expected_base_cell)
        np.testing.assert_array_equal(i, expected_i)
        np.testing.assert_array_equal(j, expected_j)
        np.testing.assert_array_equal(
            nested.base_cell_coordinates_to_healpix(base_cell, i, j, depth),
            healpy.base_cell_coordinates_to_healpix(
                nside,
                i.astype(np.int64),
                j.astype(np.int64),
                base_cell.astype(np.int64),
                nest=True,
            ),
        )
