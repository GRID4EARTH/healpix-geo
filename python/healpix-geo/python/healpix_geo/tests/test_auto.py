import sys

import numpy as np
import pytest

import healpix_geo
from healpix_geo import auto


@pytest.mark.parametrize(
    ["scheme", "expected"],
    (
        ("nested", healpix_geo.nested),
        ("ring", healpix_geo.ring),
        ("zuniq", healpix_geo.zuniq),
    ),
    ids=["nested", "ring", "zuniq"],
)
def test_dispatch_module(scheme, expected):
    actual = auto._dispatch_module(scheme)

    assert actual is expected


@pytest.mark.parametrize("scheme", ["unknown1", "test9"])
def test_dispatch_module_failing(scheme):
    with pytest.raises(ValueError, match="unknown indexing scheme"):
        auto._dispatch_module(scheme)


@pytest.mark.parametrize(
    ["grid", "expected"],
    (
        (
            auto.Grid(level=5, indexing_scheme="nested", ellipsoid="sphere"),
            np.array([12, 84, 104, 72], dtype="uint64"),
        ),
        (
            auto.Grid(level=3, indexing_scheme="ring", ellipsoid="WGS84"),
            np.array([340, 245, 244, 277], dtype="uint64"),
        ),
        (
            auto.Grid(level=6, indexing_scheme="zuniq", ellipsoid="WGS84"),
            np.array(
                [
                    6825768185233408,
                    47358164831567872,
                    58617163899994112,
                    40602765390512128,
                ],
                dtype="uint64",
            ),
        ),
    ),
)
def test_lonlat_to_healpix(grid, expected):
    lon = np.array([45.0, 64.6875, 47.8125, 53.4375], dtype="float64")
    lat = np.array([5.9791568, 18.20995686, 18.20995686, 13.24801491], dtype="float64")

    actual = auto.lonlat_to_healpix(lon, lat, grid)

    np.testing.assert_equal(actual, expected)


@pytest.mark.parametrize(
    ["grid", "cell_ids", "expected_lon", "expected_lat"],
    (
        (
            auto.Grid(level=5, indexing_scheme="nested", ellipsoid="sphere"),
            np.array([12, 104], dtype="uint64"),
            np.array([45.0, 47.8125], dtype="float64"),
            np.array([5.9791568, 18.20995686], dtype="float64"),
        ),
        (
            auto.Grid(level=3, indexing_scheme="ring", ellipsoid="sphere"),
            np.array([340, 245, 244], dtype="uint64"),
            np.array([45.0, 61.875, 50.625]),
            np.array([4.78019185, 19.47122063, 19.47122063]),
        ),
        (
            auto.Grid(level=6, indexing_scheme="zuniq", ellipsoid="WGS84"),
            np.array([6825768185233408], dtype="uint64"),
            np.array([45.0]),
            np.array([5.40338952]),
        ),
    ),
)
def test_healpix_to_lonlat(grid, cell_ids, expected_lon, expected_lat):
    actual_lon, actual_lat = auto.healpix_to_lonlat(cell_ids, grid)

    np.testing.assert_allclose(actual_lon, expected_lon)
    np.testing.assert_allclose(actual_lat, expected_lat)


@pytest.mark.parametrize(
    ["grid", "cell_ids", "step", "expected_lon", "expected_lat"],
    (
        (
            auto.Grid(level=2, indexing_scheme="nested", ellipsoid="WGS84"),
            np.array([3, 54], dtype="uint64"),
            1,
            np.array([[45.0, 56.25, 45.0, 33.75], [326.25, 337.5, 330.0, 315.0]]),
            np.array(
                [
                    [19.55202227, 30.11125172, 41.93785391, 30.11125172],
                    [30.11125172, 41.93785391, 54.46234938, 41.93785391],
                ]
            ),
        ),
        (
            auto.Grid(level=3, indexing_scheme="ring", ellipsoid="WGS84"),
            np.array([19, 67, 94], dtype="uint64"),
            2,
            np.array(
                [
                    [
                        225.0,
                        231.42857143,
                        240.0,
                        234.0,
                        225.0,
                        216.0,
                        210.0,
                        218.57142857,
                    ],
                    [
                        115.71428571,
                        117.69230769,
                        120.0,
                        114.54545455,
                        108.0,
                        106.36363636,
                        105.0,
                        110.76923077,
                    ],
                    [
                        135.0,
                        138.0,
                        141.42857143,
                        138.46153846,
                        135.0,
                        131.53846154,
                        128.57142857,
                        132.0,
                    ],
                ],
                dtype="float64",
            ),
            np.array(
                [
                    [
                        66.53737405,
                        69.50681506,
                        72.46140572,
                        75.40341607,
                        78.33504545,
                        75.40341607,
                        72.46140572,
                        69.50681506,
                    ],
                    [
                        48.26869833,
                        51.38098728,
                        54.46234938,
                        57.51586389,
                        60.54441647,
                        57.51586389,
                        54.46234938,
                        51.38098728,
                    ],
                    [
                        41.93785391,
                        45.12217715,
                        48.26869833,
                        51.38098728,
                        54.46234938,
                        51.38098728,
                        48.26869833,
                        45.12217715,
                    ],
                ],
                dtype="float64",
            ),
        ),
        (
            auto.Grid(level=None, indexing_scheme="zuniq", ellipsoid="sphere"),
            np.array(
                [1963569437533536256, 824158731808800768, 5116089176692883456],
                dtype="uint64",
            ),
            1,
            np.array(
                [
                    [326.25, 337.5, 330.0, 315.0],
                    [146.25, 154.28571429, 150.0, 141.42857143],
                    [45.0, 67.5, 45.0, 22.5],
                ],
                dtype="float64",
            ),
            np.array(
                [
                    [30.0, 41.8103149, 54.3409123, 41.8103149],
                    [41.8103149, 48.14120779, 54.3409123, 48.14120779],
                    [-41.8103149, -19.47122063, 0.0, -19.47122063],
                ],
                dtype="float64",
            ),
        ),
    ),
    ids=["nested", "ring", "zuniq"],
)
def test_vertices(grid, cell_ids, step, expected_lon, expected_lat):
    actual_lon, actual_lat = auto.vertices(cell_ids, grid, step=step)

    np.testing.assert_allclose(actual_lon, expected_lon)
    np.testing.assert_allclose(actual_lat, expected_lat)


@pytest.mark.parametrize(
    ["grid", "cell_ids", "ring", "expected"],
    (
        pytest.param(
            auto.Grid(level=2, indexing_scheme="nested", ellipsoid="sphere"),
            np.array([3, 54], dtype="uint64"),
            1,
            np.array(
                [
                    [3, 0, 2, 8, 9, 12, 6, 4, 1],
                    [54, 49, 51, 57, 60, 61, 55, 53, 52],
                ],
                dtype="int64",
            ),
            id="nested",
        ),
        pytest.param(
            auto.Grid(level=1, indexing_scheme="ring", ellipsoid="sphere"),
            np.array([12, 31, 39], dtype="uint64"),
            2,
            np.array(
                [
                    [
                        12,
                        19,
                        11,
                        4,
                        13,
                        28,
                        27,
                        20,
                        43,
                        35,
                        26,
                        18,
                        10,
                        3,
                        0,
                        5,
                        14,
                        21,
                        29,
                        36,
                        -1,
                        -1,
                        -1,
                        -1,
                        -1,
                    ],
                    [
                        31,
                        30,
                        22,
                        15,
                        23,
                        32,
                        45,
                        38,
                        39,
                        47,
                        44,
                        37,
                        21,
                        14,
                        6,
                        1,
                        7,
                        16,
                        24,
                        40,
                        46,
                        -1,
                        -1,
                        -1,
                        -1,
                    ],
                    [
                        39,
                        23,
                        32,
                        40,
                        46,
                        45,
                        38,
                        31,
                        42,
                        47,
                        44,
                        37,
                        30,
                        22,
                        15,
                        7,
                        16,
                        24,
                        33,
                        41,
                        -1,
                        -1,
                        -1,
                        -1,
                        -1,
                    ],
                ]
            ),
            id="ring",
        ),
        pytest.param(
            auto.Grid(level=None, indexing_scheme="zuniq", ellipsoid="sphere"),
            np.array(
                [1963569437533536256, 824158731808800768, 5116089176692883456],
                dtype="uint64",
            ),
            1,
            np.array(
                [
                    [
                        1963569437533536256,
                        1783425452438716416,
                        1855483046476644352,
                        2071655828590428160,
                        2179742219647320064,
                        2215771016666284032,
                        1999598234552500224,
                        1927540640514572288,
                        1891511843495608320,
                    ],
                    [
                        824158731808800768,
                        797137134044577792,
                        815151532554059776,
                        1013309916158361600,
                        1022317115413102592,
                        1049338713177325568,
                        851180329573023744,
                        833165931063541760,
                        806144333299318784,
                    ],
                    [
                        5116089176692883456,
                        2377900603251621888,
                        2522015791327477760,
                        72057594037927936,
                        3242591731706757120,
                        2954361355555045376,
                        4683743612465315840,
                        4971973988617027584,
                        4827858800541171712,
                    ],
                ],
                dtype="int64",
            ),
            id="zuniq",
        ),
    ),
)
def test_kth_neighbours(grid, cell_ids, ring, expected):
    actual = auto.kth_neighbourhood(cell_ids, grid, ring=ring)

    np.testing.assert_equal(actual, expected)


def test_zone_coverage():
    bbox = (0.0, 0.0, 45.0, 45.0)
    grid = auto.Grid(level=2, indexing_scheme="nested", ellipsoid="WGS84")

    actual_cell_ids, actual_levels, actual_coverage = auto.zone_coverage(
        bbox, grid, flat=True
    )

    expected_cell_ids = np.array(
        [0, 2, 3, 8, 9, 10, 11, 12, 69, 70, 71, 76, 77, 79], dtype="uint64"
    )
    expected_coverage = np.array(
        [
            False,
            False,
            False,
            True,
            False,
            False,
            False,
            False,
            False,
            False,
            True,
            False,
            False,
            False,
        ],
        dtype="bool",
    )

    np.testing.assert_equal(actual_cell_ids, expected_cell_ids)
    np.testing.assert_equal(actual_levels, grid.level)
    np.testing.assert_equal(actual_coverage, expected_coverage)


def test_box_coverage():
    grid = auto.Grid(level=2, indexing_scheme="nested", ellipsoid="WGS84")

    expected_cell_ids = np.array([9, 11, 12, 13, 14], dtype="uint64")
    expected_coverage = np.array([False, False, False, False, False], dtype="uint64")

    actual_cell_ids, actual_levels, actual_coverage = auto.box_coverage(
        center=(35.0, 55.0), size=(10.0, 10.0), angle=25.0, grid=grid, flat=True
    )

    np.testing.assert_equal(actual_cell_ids, expected_cell_ids)
    np.testing.assert_equal(actual_levels, grid.level)
    np.testing.assert_equal(actual_coverage, expected_coverage)


@pytest.mark.skipif(
    sys.platform == "win32",
    reason="polygon_coverage returns a different result on windows",
)
def test_polygon_coverage():
    grid = auto.Grid(level=2, indexing_scheme="nested", ellipsoid="WGS84")

    expected_cell_ids = np.array(
        [0, 1, 2, 3, 6, 8, 9, 10, 11, 12, 53, 69, 70, 71, 76, 77, 78, 79],
        dtype="uint64",
    )
    expected_coverage = np.array(
        [
            False,
            False,
            False,
            False,
            False,
            True,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
        ],
        dtype="bool",
    )

    actual_cell_ids, actual_levels, actual_coverage = auto.polygon_coverage(
        np.array(
            [
                [0.0, 0.0],
                [45.0, 0.0],
                [45.0, 45.0],
                [0.0, 45.0],
            ],
            dtype="float64",
        ),
        grid=grid,
        flat=True,
    )

    np.testing.assert_equal(actual_cell_ids, expected_cell_ids)
    np.testing.assert_equal(actual_levels, grid.level)
    np.testing.assert_equal(actual_coverage, expected_coverage)


def test_cone_coverage():
    grid = auto.Grid(level=2, indexing_scheme="nested", ellipsoid="WGS84")

    expected_cell_ids = np.array(
        [
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
            13,
            14,
            15,
            27,
            30,
            31,
            53,
            55,
            61,
            63,
            79,
        ],
        dtype="uint64",
    )
    expected_coverage = np.array(
        [
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            True,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
        ],
        dtype="bool",
    )

    actual_cell_ids, actual_levels, actual_coverage = auto.cone_coverage(
        center=(35.0, 55.0), radius=25.0, grid=grid, flat=True
    )

    np.testing.assert_equal(actual_cell_ids, expected_cell_ids)
    np.testing.assert_equal(actual_levels, grid.level)
    np.testing.assert_equal(actual_coverage, expected_coverage)


def test_elliptical_cone_coverage():
    grid = auto.Grid(level=2, indexing_scheme="nested", ellipsoid="WGS84")

    expected_cell_ids = np.array(
        [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 30, 31, 53, 55, 61, 63, 79],
        dtype="uint64",
    )
    expected_coverage = np.array(
        [
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            True,
            True,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
            False,
        ],
        dtype="bool",
    )

    actual_cell_ids, actual_levels, actual_coverage = auto.elliptical_cone_coverage(
        center=(35.0, 55.0),
        ellipse_geometry=(25.0, 20.0),
        position_angle=25.0,
        grid=grid,
        flat=True,
    )

    np.testing.assert_equal(actual_cell_ids, expected_cell_ids)
    np.testing.assert_equal(actual_levels, grid.level)
    np.testing.assert_equal(actual_coverage, expected_coverage)
