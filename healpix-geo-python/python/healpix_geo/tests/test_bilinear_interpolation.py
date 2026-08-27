import marray
import numpy as np
import pytest

import healpix_geo.nested

mxp = marray.masked_namespace(np)


@pytest.mark.parametrize(
    [
        "lon",
        "lat",
        "scheme",
        "depth",
        "ellipsoid",
        "expected_cell_ids",
        "expected_weights",
    ],
    (
        pytest.param(
            np.array([3.5, 71.2], dtype="float64"),
            np.array([23.1, -23.1], dtype="float64"),
            "nested",
            3,
            "sphere",
            mxp.asarray(
                np.array([[310, 311, 316, 317], [541, 328, 543, 330]]), mask=False
            ),
            mxp.asarray(
                np.array(
                    [
                        [
                            0.3816076602587845,
                            0.07548075123095703,
                            0.45325852900873526,
                            0.0896530595015232,
                        ],
                        [
                            0.09605305950152325,
                            0.08685852900873488,
                            0.4290807512309573,
                            0.3880076602587846,
                        ],
                    ],
                    dtype="float64",
                ),
                mask=False,
            ),
            id="nested-sphere-3",
        ),
        pytest.param(
            np.array([0.0, 90.0, 180.0, 270.0], dtype="float64"),
            np.array([-90, -33.0, 33.0, 90], dtype="float64"),
            "nested",
            5,
            "WGS84",
            mxp.asarray(
                np.array(
                    [
                        [10240, 9216, 11264, 8192],
                        [5132, 5133, 5134, 5135],
                        [7152, 7153, 7154, 7155],
                        [4095, 1023, 3071, 2047],
                    ]
                ),
                mask=False,
            ),
            mxp.asarray(
                np.array(
                    [
                        [
                            0.2499999999999976,
                            0.2499999999999976,
                            0.2500000000000024,
                            0.2500000000000024,
                        ],
                        [
                            0.2810770706752161,
                            0.2490899508144425,
                            0.2490899508144425,
                            0.22074302769589896,
                        ],
                        [
                            0.2207430276958973,
                            0.24908995081444238,
                            0.24908995081444238,
                            0.281077070675218,
                        ],
                        [
                            0.25000000000000355,
                            0.24999999999999645,
                            0.25000000000000355,
                            0.24999999999999645,
                        ],
                    ],
                    dtype="float64",
                ),
                mask=False,
            ),
            id="nested-wgs84-5",
        ),
        pytest.param(
            np.array([3.5, 71.2], dtype="float64"),
            np.array([23.1, -23.1], dtype="float64"),
            "ring",
            3,
            "sphere",
            mxp.asarray(
                np.array([[240, 209, 208, 176], [566, 535, 534, 502]], dtype="uint64"),
                mask=False,
            ),
            mxp.asarray(
                np.array(
                    [
                        [
                            0.3816076602587845,
                            0.07548075123095703,
                            0.45325852900873526,
                            0.0896530595015232,
                        ],
                        [
                            0.09605305950152325,
                            0.08685852900873488,
                            0.4290807512309573,
                            0.3880076602587846,
                        ],
                    ],
                    dtype="float64",
                ),
                mask=False,
            ),
            id="ring-sphere-3",
        ),
        pytest.param(
            np.array([3.5, 71.2], dtype="float64"),
            np.array([23.1, -23.1], dtype="float64"),
            "zuniq",
            3,
            "sphere",
            mxp.asarray(
                np.array(
                    [
                        [
                            2796735368597078016,
                            2805742567851819008,
                            2850778564125523968,
                            2859785763380264960,
                        ],
                        [
                            4877398396442247168,
                            2958864955182415872,
                            4895412794951729152,
                            2976879353691897856,
                        ],
                    ],
                    dtype="uint64",
                ),
                mask=False,
            ),
            mxp.asarray(
                np.array(
                    [
                        [
                            0.3816076602587845,
                            0.07548075123095703,
                            0.45325852900873526,
                            0.0896530595015232,
                        ],
                        [
                            0.09605305950152325,
                            0.08685852900873488,
                            0.4290807512309573,
                            0.3880076602587846,
                        ],
                    ],
                    dtype="float64",
                ),
                mask=False,
            ),
            id="zuniq-sphere-3",
        ),
    ),
)
def test_bilinear_interpolation(
    lon, lat, scheme, depth, ellipsoid, expected_cell_ids, expected_weights
):
    actual_cell_ids, actual_weights = getattr(
        healpix_geo, scheme
    ).bilinear_interpolation(lon, lat, depth=depth, ellipsoid=ellipsoid)

    np.testing.assert_equal(actual_cell_ids.data, expected_cell_ids.data)
    np.testing.assert_equal(actual_cell_ids.mask, expected_cell_ids.mask)
    np.testing.assert_equal(actual_weights.mask, expected_weights.mask)
    np.testing.assert_allclose(actual_weights.data, expected_weights.data)
