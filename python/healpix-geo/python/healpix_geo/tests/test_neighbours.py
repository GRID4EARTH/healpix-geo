import cdshealpix.nested
import cdshealpix.ring
import numpy as np
import pytest

import healpix_geo


@pytest.mark.parametrize("depth", [2, 8])
@pytest.mark.parametrize("ring", [0, 1])
@pytest.mark.parametrize(
    "indexing_scheme",
    [
        "nested",
        "ring",
    ],
)
def test_kth_neighbourhood_against_cdshealpix(depth, ring, indexing_scheme):
    if indexing_scheme == "nested":
        kth_neighbourhood = healpix_geo.nested.kth_neighbourhood
        neighbours = cdshealpix.nested.neighbours
    elif indexing_scheme == "ring":
        kth_neighbourhood = healpix_geo.ring.kth_neighbourhood

        def neighbours(ipix, depth):
            return cdshealpix.to_ring(
                cdshealpix.nested.neighbours(cdshealpix.from_ring(ipix, depth), depth),
                depth=depth,
            )

    ipixels = np.array([50, 100], dtype="int64")

    actual = kth_neighbourhood(depth=depth, ipix=ipixels, ring=ring)
    if ring == 0:
        expected = np.reshape(ipixels, (-1, 1))
    else:
        expected = neighbours(ipix=ipixels, depth=depth)

    np.testing.assert_equal(np.sort(actual, axis=-1), np.sort(expected, axis=-1))
    np.testing.assert_equal(actual[:, 0], ipixels)


@pytest.mark.parametrize(
    ["depth", "cell_ids", "indexing_scheme", "ring", "expected"],
    (
        pytest.param(
            1,
            np.array([7, 8, 23, 45], dtype="uint64"),
            "nested",
            1,
            np.array(
                [
                    [7, 1, 3, 15, 10, 11, 4, 6, 5],
                    [8, 43, 25, 27, 31, 30, 10, 11, 9],
                    [23, 0, 1, 6, 4, 20, 22, 21, -1],
                    [45, 18, 16, 34, 32, 44, 46, 47, -1],
                ],
                dtype="int64",
            ),
        ),
        pytest.param(
            5,
            np.array([92, 109], dtype="uint64"),
            "ring",
            2,
            np.array(
                [
                    [
                        92,
                        155,
                        121,
                        91,
                        66,
                        45,
                        67,
                        93,
                        122,
                        119,
                        90,
                        65,
                        44,
                        27,
                        234,
                        192,
                        154,
                        120,
                        28,
                        46,
                        68,
                        94,
                        123,
                        156,
                        193,
                    ],
                    [
                        109,
                        176,
                        140,
                        108,
                        81,
                        58,
                        82,
                        110,
                        141,
                        259,
                        215,
                        175,
                        139,
                        107,
                        80,
                        57,
                        38,
                        23,
                        39,
                        59,
                        83,
                        111,
                        142,
                        177,
                        216,
                    ],
                ],
                dtype="int64",
            ),
        ),
        pytest.param(
            8,
            np.array([1460288880640], dtype="uint64"),
            "zuniq",
            3,
            np.array(
                [
                    [
                        1460288880640,
                        2497997973430992896,
                        2497998042150469632,
                        2498000997087969280,
                        4415226380288,
                        4449586118656,
                        1494648619008,
                        1425929142272,
                        1391569403904,
                        2497997732912824320,
                        2497997939071254528,
                        2497998007790731264,
                        2498000962728230912,
                        2498001031447707648,
                        2498001065807446016,
                        2497997767272562688,
                        4483945857024,
                        4518305595392,
                        4621384810496,
                        4552665333760,
                        1597727834112,
                        1529008357376,
                        1322849927168,
                        1219770712064,
                        1185410973696,
                        2497997561114132480,
                        2497997629833609216,
                        2497997835992039424,
                        2497997904711516160,
                        2498000859649015808,
                        2498000928368492544,
                        2498001134526922752,
                        2498001237606137856,
                        2498001271965876224,
                        2497997698553085952,
                        2497997664193347584,
                        4690104287232,
                        4724464025600,
                        4827543240704,
                        4861902979072,
                        4655744548864,
                        4587025072128,
                        1632087572480,
                        1563368095744,
                        1357209665536,
                        1288490188800,
                        1254130450432,
                        1151051235328,
                        1116691496960,
                    ]
                ],
                dtype="int64",
            ),
        ),
    ),
)
def test_kth_neighbourhood(depth, cell_ids, ring, indexing_scheme, expected):
    funcs = {
        "ring": lambda cell_ids: healpix_geo.ring.kth_neighbourhood(
            cell_ids, depth, ring
        ),
        "nested": lambda cell_ids: healpix_geo.nested.kth_neighbourhood(
            cell_ids, depth, ring
        ),
        "zuniq": lambda cell_ids: healpix_geo.zuniq.kth_neighbourhood(cell_ids, ring),
    }

    func = funcs[indexing_scheme]
    actual = func(cell_ids)
    np.testing.assert_equal(actual, expected)


@pytest.mark.parametrize(
    ["depth", "cell_ids", "indexing_scheme", "ring", "expected"],
    (
        pytest.param(
            1,
            np.array([7, 8, 23, 45], dtype="uint64"),
            "nested",
            1,
            np.array(
                [
                    [4, 5, 10, 6, 11, 1, 3, 15],
                    [43, 30, 31, 25, 9, 27, 10, 11],
                    [20, 21, 4, 22, 6, 0, 1, -1],
                    [32, 34, 44, 16, 46, 47, 18, -1],
                ],
                dtype="int64",
            ),
        ),
        pytest.param(
            5,
            np.array([92, 109], dtype="uint64"),
            "ring",
            2,
            np.array(
                [
                    [
                        119,
                        90,
                        65,
                        44,
                        27,
                        234,
                        192,
                        154,
                        120,
                        28,
                        46,
                        68,
                        94,
                        123,
                        156,
                        193,
                    ],
                    [
                        259,
                        215,
                        175,
                        139,
                        107,
                        80,
                        57,
                        38,
                        23,
                        39,
                        59,
                        83,
                        111,
                        142,
                        177,
                        216,
                    ],
                ],
                dtype="int64",
            ),
        ),
        pytest.param(
            8,
            np.array([1460288880640], dtype="uint64"),
            "zuniq",
            3,
            np.array(
                [
                    [
                        2497997561114132480,
                        2497997629833609216,
                        2497997835992039424,
                        2497997904711516160,
                        2498000859649015808,
                        2498000928368492544,
                        2498001134526922752,
                        2498001237606137856,
                        2498001271965876224,
                        2497997698553085952,
                        2497997664193347584,
                        4690104287232,
                        4724464025600,
                        4827543240704,
                        4861902979072,
                        4655744548864,
                        4587025072128,
                        1632087572480,
                        1563368095744,
                        1357209665536,
                        1288490188800,
                        1254130450432,
                        1151051235328,
                        1116691496960,
                    ]
                ],
                dtype="int64",
            ),
        ),
    ),
)
def test_kth_neighbours(depth, cell_ids, ring, indexing_scheme, expected):
    funcs = {
        "ring": lambda cell_ids: healpix_geo.ring.kth_neighbours(cell_ids, depth, ring),
        "nested": lambda cell_ids: healpix_geo.nested.kth_neighbours(
            cell_ids, depth, ring
        ),
        "zuniq": lambda cell_ids: healpix_geo.zuniq.kth_neighbours(cell_ids, ring),
    }

    func = funcs[indexing_scheme]
    actual = func(cell_ids)
    print(repr(actual))
    np.testing.assert_equal(actual, expected)
