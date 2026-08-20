import numpy as np
import pytest

from healpix_geo import nested

DIRECTIONS = ("N", "NE", "E", "SE", "S", "SW", "W", "NW")
EXPECTED_NEIGHBOUR_FACES = (
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


class TestFaceNeighbourTransform:
def test_face_neighbour_transform_all_faces_and_directions():
    for face, expected_faces in enumerate(EXPECTED_NEIGHBOUR_FACES):
        for direction, expected_face in zip(DIRECTIONS, expected_faces, strict=True):
            transform = nested.face_neighbour_transform(face, direction)
            if expected_face is None:
                assert transform is None
            else:
                assert transform.target_face == expected_face


def test_face_neighbour_transform_orientation_cases():
    assert nested.face_neighbour_transform(0, "n") == nested.FaceTransform(
        target_face=2, swap_xy=False, flip_x=True, flip_y=True
    )
    assert nested.face_neighbour_transform(0, "NE") == nested.FaceTransform(
        target_face=1, swap_xy=True, flip_x=False, flip_y=True
    )
    assert nested.face_neighbour_transform(4, "E") == nested.FaceTransform(
        target_face=5, swap_xy=False, flip_x=False, flip_y=False
    )
    assert nested.face_neighbour_transform(8, "SE") == nested.FaceTransform(
        target_face=9, swap_xy=True, flip_x=True, flip_y=False
    )
    assert nested.face_neighbour_transform(8, "S") == nested.FaceTransform(
        target_face=10, swap_xy=False, flip_x=True, flip_y=True
    )


@pytest.mark.parametrize("face", [-1, 12, 256])
def test_face_neighbour_transform_rejects_invalid_face(face):
    with pytest.raises(ValueError, match="Face"):
        nested.face_neighbour_transform(face, "N")


@pytest.mark.parametrize("face", ["0", 0.0, [0]])
def test_face_neighbour_transform_rejects_non_integer_face(face):
    with pytest.raises(TypeError, match="face must be an integer"):
        nested.face_neighbour_transform(face, "N")


def test_face_neighbour_transform_rejects_invalid_direction():
    with pytest.raises(ValueError, match="direction"):
        nested.face_neighbour_transform(0, "up")
    with pytest.raises(TypeError, match="direction"):
        nested.face_neighbour_transform(0, 0)


@pytest.mark.parametrize("depth", range(6))
def test_exhaustive_small_depth_round_trip(depth):
    pixels = np.arange(12 * 4**depth, dtype=np.uint64)

    face, x, y = nested.pix2xyf(pixels, depth)

    np.testing.assert_array_equal(nested.xyf2pix(face, x, y, depth), pixels)
    actual_face, actual_x, actual_y = nested.pix2xyf(
        nested.xyf2pix(face, x, y, depth), depth
    )
    np.testing.assert_array_equal(actual_face, face)
    np.testing.assert_array_equal(actual_x, x)
    np.testing.assert_array_equal(actual_y, y)


def test_level_zero_is_the_base_face():
    pixels = np.arange(12, dtype=np.uint64)

    face, x, y = nested.pix2xyf(pixels, 0)

    np.testing.assert_array_equal(face, np.arange(12, dtype=np.uint8))
    np.testing.assert_array_equal(x, np.zeros(12, dtype=np.uint32))
    np.testing.assert_array_equal(y, np.zeros(12, dtype=np.uint32))


def test_scalar_inputs_return_length_one_arrays():
    face, x, y = nested.pix2xyf(47, 1)

    np.testing.assert_array_equal(face, np.array([11], dtype=np.uint8))
    np.testing.assert_array_equal(x, np.array([1], dtype=np.uint32))
    np.testing.assert_array_equal(y, np.array([1], dtype=np.uint32))
    np.testing.assert_array_equal(
        nested.xyf2pix(11, 1, 1, 1), np.array([47], dtype=np.uint64)
    )


def test_multidimensional_shape_and_dtypes_are_preserved():
    pixels = np.arange(48, dtype=np.uint64).reshape(2, 3, 8)

    face, x, y = nested.pix2xyf(pixels, 1)

    assert face.shape == pixels.shape
    assert x.shape == pixels.shape
    assert y.shape == pixels.shape
    assert face.dtype == np.uint8
    assert x.dtype == np.uint32
    assert y.dtype == np.uint32
    actual = nested.xyf2pix(face, x, y, 1)
    assert actual.shape == pixels.shape
    assert actual.dtype == np.uint64
    np.testing.assert_array_equal(actual, pixels)


def test_xyf2pix_broadcasts_inputs():
    face = np.arange(12, dtype=np.uint8)[:, np.newaxis]
    x = np.arange(4, dtype=np.uint32)[np.newaxis, :]
    y = np.array(2, dtype=np.uint32)

    pixels = nested.xyf2pix(face, x, y, 2)

    assert pixels.shape == (12, 4)
    actual_face, actual_x, actual_y = nested.pix2xyf(pixels, 2)
    np.testing.assert_array_equal(actual_face, np.broadcast_to(face, (12, 4)))
    np.testing.assert_array_equal(actual_x, np.broadcast_to(x, (12, 4)))
    np.testing.assert_array_equal(actual_y, np.full((12, 4), 2, dtype=np.uint32))


def test_array_depth_broadcasts_with_pixels():
    depth = np.array([0, 1, 2, 10, 29], dtype=np.uint8)
    face = np.array([0, 3, 7, 11, 5], dtype=np.uint8)
    x = np.array([0, 1, 3, 1023, 2**29 - 1], dtype=np.uint32)
    y = np.array([0, 0, 2, 17, 123_456_789], dtype=np.uint32)

    pixels = nested.xyf2pix(face, x, y, depth)
    actual = nested.pix2xyf(pixels, depth)

    for component, expected in zip(actual, (face, x, y), strict=True):
        np.testing.assert_array_equal(component, expected)


@pytest.mark.parametrize(
    ("face", "x", "y", "depth", "message"),
    [
        (-1, 0, 0, 1, "Face"),
        (12, 0, 0, 1, "Face"),
        (0, -1, 0, 1, "x"),
        (0, 2, 0, 1, "x"),
        (0, 0, -1, 1, "y"),
        (0, 0, 2, 1, "y"),
    ],
)
def test_xyf2pix_rejects_invalid_coordinates(face, x, y, depth, message):
    with pytest.raises(ValueError, match=message):
        nested.xyf2pix(face, x, y, depth)


@pytest.mark.parametrize("depth", [-1, 30])
def test_topology_rejects_invalid_depth(depth):
    with pytest.raises(ValueError, match="Depth"):
        nested.pix2xyf([0], depth)
    with pytest.raises(ValueError, match="Depth"):
        nested.xyf2pix([0], [0], [0], depth)


def test_pix2xyf_rejects_invalid_cell_id():
    with pytest.raises(ValueError, match="out of"):
        nested.pix2xyf([48], 1)


def test_matches_healpy_reference():
    healpy = pytest.importorskip("healpy")
    rng = np.random.default_rng(234243)

    for depth in (0, 1, 2, 7, 14, 29):
        nside = 2**depth
        npix = 12 * nside**2
        pixels = rng.integers(0, npix, size=256, dtype=np.uint64)

        face, x, y = nested.pix2xyf(pixels, depth)
        expected_x, expected_y, expected_face = healpy.pix2xyf(
            nside, pixels.astype(np.int64), nest=True
        )

        np.testing.assert_array_equal(face, expected_face)
        np.testing.assert_array_equal(x, expected_x)
        np.testing.assert_array_equal(y, expected_y)
        np.testing.assert_array_equal(
            nested.xyf2pix(face, x, y, depth),
            healpy.xyf2pix(
                nside,
                x.astype(np.int64),
                y.astype(np.int64),
                face.astype(np.int64),
                nest=True,
            ),
        )
