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


def _geodesic_distances(indexing_scheme):
    return getattr(healpix_geo, indexing_scheme).geodesic_distances


def _random_pairs(rng, depth, size, width):
    n_cells = 12 * 4**depth
    from_ = rng.integers(0, n_cells, size=size, dtype="uint64")
    to_ = rng.integers(0, n_cells, size=(size, width), dtype="uint64")

    return from_, to_


def _pyproj_distances(from_, to_, depth, indexing_scheme):
    pyproj = pytest.importorskip("pyproj")

    module = getattr(healpix_geo, indexing_scheme)
    to_ = np.asarray(to_)
    from_ = np.broadcast_to(np.asarray(from_)[..., None], to_.shape)

    lon1, lat1 = module.healpix_to_lonlat(from_.ravel(), depth, ellipsoid="WGS84")
    lon2, lat2 = module.healpix_to_lonlat(to_.ravel(), depth, ellipsoid="WGS84")
    _, _, distances = pyproj.Geod(ellps="WGS84").inv(lon1, lat1, lon2, lat2)

    return distances.reshape(to_.shape)


@pytest.mark.parametrize("indexing_scheme", ["nested", "ring"])
@pytest.mark.parametrize("depth", [0, 5, 12, 22])
def test_geodesic_distances_match_pyproj(indexing_scheme, depth):
    rng = np.random.default_rng(seed=depth)
    from_, to_ = _random_pairs(rng, depth, size=500, width=4)

    actual = _geodesic_distances(indexing_scheme)(from_, to_, depth, ellipsoid="WGS84")
    expected = _pyproj_distances(from_, to_, depth, indexing_scheme)

    np.testing.assert_allclose(actual, expected, rtol=0, atol=1e-6)


# Radii from the IAU WG on Cartographic Coordinates and Rotational Elements
# (2015 report): equatorial radius `a` and polar radius `b`, in metres.
PLANETS = {
    "mars": (3_396_190.0, 3_376_200.0),
    "jupiter": (71_492_000.0, 66_854_000.0),
    "venus": (6_051_800.0, 6_051_800.0),
}


@pytest.mark.parametrize("indexing_scheme", ["nested", "ring", "zuniq"])
@pytest.mark.parametrize("planet", PLANETS)
def test_geodesic_distances_planets_match_pyproj(indexing_scheme, planet):
    pyproj = pytest.importorskip("pyproj")

    a, b = PLANETS[planet]
    if a == b:
        ellipsoid = {"radius": a}
    else:
        ellipsoid = {"semimajor_axis": a, "inverse_flattening": a / (a - b)}

    depth = 10
    rng = np.random.default_rng(seed=2)
    from_, to_ = _random_pairs(rng, depth, size=500, width=4)
    if indexing_scheme == "zuniq":
        from_ = healpix_geo.nested.to_zuniq(from_, depth)
        to_ = healpix_geo.nested.to_zuniq(to_, depth)
        lonlat_args = ()
    else:
        lonlat_args = (depth,)

    module = getattr(healpix_geo, indexing_scheme)
    actual = module.geodesic_distances(from_, to_, *lonlat_args, ellipsoid=ellipsoid)

    from_ = np.broadcast_to(from_[:, None], to_.shape)
    lon1, lat1 = module.healpix_to_lonlat(
        from_.ravel(), *lonlat_args, ellipsoid=ellipsoid
    )
    lon2, lat2 = module.healpix_to_lonlat(
        to_.ravel(), *lonlat_args, ellipsoid=ellipsoid
    )
    _, _, expected = pyproj.Geod(a=a, b=b).inv(lon1, lat1, lon2, lat2)

    np.testing.assert_allclose(actual, expected.reshape(to_.shape), rtol=0, atol=1e-6)


def test_geodesic_distances_neighbours_match_pyproj():
    depth = 18
    rng = np.random.default_rng(seed=0)
    from_ = rng.integers(0, 12 * 4**depth, size=1000, dtype="uint64")
    to_ = healpix_geo.nested.kth_neighbourhood(from_, depth, ring=2)

    actual = healpix_geo.nested.geodesic_distances(from_, to_, depth, ellipsoid="WGS84")
    expected = np.where(
        to_ == -1,
        np.nan,
        _pyproj_distances(from_, np.where(to_ == -1, 0, to_), depth, "nested"),
    )

    np.testing.assert_allclose(actual, expected, rtol=0, atol=1e-6)


def test_geodesic_distances_zuniq_matches_nested():
    rng = np.random.default_rng(seed=1)
    from_, to_ = _random_pairs(rng, depth=9, size=200, width=3)
    expected = healpix_geo.nested.geodesic_distances(from_, to_, 9, ellipsoid="WGS84")

    actual = healpix_geo.zuniq.geodesic_distances(
        healpix_geo.nested.to_zuniq(from_, 9),
        healpix_geo.nested.to_zuniq(to_, 9),
        ellipsoid="WGS84",
    )

    np.testing.assert_array_equal(actual, expected)


def test_geodesic_distances_zuniq_mixed_depths():
    pyproj = pytest.importorskip("pyproj")

    from_ = healpix_geo.nested.to_zuniq(np.array([5, 300]), depth=3)
    to_ = np.stack(
        [
            healpix_geo.nested.to_zuniq(np.array([7, 30]), depth=2),
            healpix_geo.nested.to_zuniq(np.array([1000, 40000]), depth=8),
        ],
        axis=-1,
    )

    actual = healpix_geo.zuniq.geodesic_distances(from_, to_, ellipsoid="WGS84")

    lon1, lat1 = healpix_geo.zuniq.healpix_to_lonlat(
        np.repeat(from_, 2), ellipsoid="WGS84"
    )
    lon2, lat2 = healpix_geo.zuniq.healpix_to_lonlat(to_.ravel(), ellipsoid="WGS84")
    _, _, expected = pyproj.Geod(ellps="WGS84").inv(lon1, lat1, lon2, lat2)

    np.testing.assert_allclose(actual, expected.reshape(2, 2), rtol=0, atol=1e-6)


@pytest.mark.parametrize("indexing_scheme", ["nested", "ring"])
def test_geodesic_distances_sphere(indexing_scheme):
    radius = 6_371_000.0
    rng = np.random.default_rng(seed=2)
    from_, to_ = _random_pairs(rng, depth=7, size=300, width=5)

    actual = _geodesic_distances(indexing_scheme)(
        from_, to_, 7, ellipsoid={"radius": radius}
    )
    angular = getattr(healpix_geo, indexing_scheme).angular_distances(from_, to_, 7)

    np.testing.assert_allclose(actual, angular * radius, rtol=1e-12)


@pytest.mark.parametrize("indexing_scheme", ["nested", "ring", "zuniq"])
def test_geodesic_distances_masking(indexing_scheme):
    from_ = np.array([0, 16, 25, 32, 46])
    to_ = np.array([[2, 4], [15, 7], [27, 26], [40, -1], [-1, 41]], dtype="int64")
    if indexing_scheme == "zuniq":
        from_ = healpix_geo.nested.to_zuniq(from_, 2)
        zuniq_to = healpix_geo.nested.to_zuniq(np.where(to_ == -1, 0, to_), 2)
        to_ = np.where(to_ == -1, -1, zuniq_to.astype("int64"))
        args = ()
    else:
        args = (2,)

    actual = _geodesic_distances(indexing_scheme)(from_, to_, *args, ellipsoid="WGS84")

    assert actual.shape == to_.shape
    np.testing.assert_array_equal(np.isnan(actual), to_ == -1)
    assert (actual[to_ != -1] > 0).all()


@pytest.mark.parametrize("indexing_scheme", ["nested", "ring"])
def test_geodesic_distances_shapes(indexing_scheme):
    geodesic_distances = _geodesic_distances(indexing_scheme)
    from_ = np.array([0, 16, 25, 32])
    to_ = np.array([2, 15, 27, 40])

    one_to_one = geodesic_distances(from_, to_, 1, ellipsoid="WGS84")
    padded = geodesic_distances(from_, to_[:, None], 1, ellipsoid="WGS84")
    n_dimensional = geodesic_distances(
        from_.reshape(2, 2), to_.reshape(2, 2, 1), 1, ellipsoid="WGS84"
    )
    empty = geodesic_distances(from_, np.empty((4, 0), dtype="int64"), 1)

    assert one_to_one.shape == (4,)
    np.testing.assert_array_equal(padded, one_to_one[:, None])
    np.testing.assert_array_equal(n_dimensional, one_to_one.reshape(2, 2, 1))
    assert empty.shape == (4, 0)


@pytest.mark.parametrize("indexing_scheme", ["nested", "ring", "zuniq"])
def test_geodesic_distances_shape_error(indexing_scheme):
    from_ = np.array([4, 7])
    to_ = np.array([[2, 3], [4, 6], [5, 4]])
    args = () if indexing_scheme == "zuniq" else (1,)

    with pytest.raises(ValueError, match="The shape of `from_` must be compatible"):
        _geodesic_distances(indexing_scheme)(from_, to_, *args)


@pytest.mark.parametrize("indexing_scheme", ["nested", "ring"])
def test_geodesic_distances_invalid_cells(indexing_scheme):
    with pytest.raises(ValueError, match="out of"):
        _geodesic_distances(indexing_scheme)(np.array([0, 48]), np.array([1, 2]), 1)
    with pytest.raises(ValueError, match="out of"):
        _geodesic_distances(indexing_scheme)(np.array([0, 1]), np.array([1, -2]), 1)


@pytest.mark.parametrize("indexing_scheme", ["nested", "ring"])
def test_geodesic_distances_num_threads(indexing_scheme):
    geodesic_distances = _geodesic_distances(indexing_scheme)
    rng = np.random.default_rng(seed=3)
    from_, to_ = _random_pairs(rng, depth=10, size=5000, width=8)

    results = [
        geodesic_distances(from_, to_, 10, ellipsoid="WGS84", num_threads=n)
        for n in (0, 1, 4)
    ]

    np.testing.assert_array_equal(results[0], results[1])
    np.testing.assert_array_equal(results[0], results[2])
