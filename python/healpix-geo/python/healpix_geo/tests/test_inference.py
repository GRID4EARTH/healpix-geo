from contextlib import nullcontext
from dataclasses import dataclass

import cdshealpix.nested
import cdshealpix.ring
import numpy as np
import pytest
from astropy.coordinates import Latitude, Longitude

import healpix_geo

# TODO(keewis): at least one multidimensional test for the explicit checks
# TODO(keewis): explicit tests for vertices
# TODO(keewis): maybe double check ellipsoidal lon / lat using pyproj?


@dataclass
class Sphere:
    radius: float


@dataclass
class Ellipsoid:
    semimajor_axis: float
    inverse_flattening: float


@pytest.mark.parametrize(
    ["ellipsoid_like", "error_handler"],
    (
        pytest.param("WGS84", nullcontext(), id="named-existing"),
        pytest.param(
            "unknown-ellipsoid",
            pytest.raises(ValueError, match="Operator 'unknown-ellipsoid' not found"),
            id="named-not_existing",
        ),
        pytest.param(Sphere(radius=1), nullcontext(), id="object-sphere"),
        pytest.param(
            Ellipsoid(semimajor_axis=1, inverse_flattening=10),
            nullcontext(),
            id="object-ellipsoid",
        ),
        pytest.param(
            object(),
            pytest.raises(TypeError, match="failed to extract enum"),
            id="object-unknown",
        ),
        pytest.param({"radius": 1}, nullcontext(), id="dict-sphere"),
        pytest.param(
            {"semimajor_axis": 1, "inverse_flattening": 10},
            nullcontext(),
            id="dict-ellipsoid",
        ),
        pytest.param(
            {"abc": 2},
            pytest.raises(TypeError, match="failed to extract enum"),
            id="dict-unknown",
        ),
        pytest.param(
            {"radius": -1},
            pytest.raises(
                ValueError, match="The radius must be greater than 0, but got -1.0"
            ),
            id="dict-ellipsoid-low_radius",
        ),
        pytest.param(
            {"semimajor_axis": 1, "inverse_flattening": 0.5},
            pytest.raises(
                ValueError,
                match="The inverse flattening must be greater than or equal to 2, but got 0.5",
            ),
            id="dict-ellipsoid-low_inverse_flattening",
        ),
        pytest.param(
            {"semimajor_axis": 0, "inverse_flattening": 10},
            pytest.raises(
                ValueError,
                match="The semimajor axis must be greater than 0, but got 0.0",
            ),
            id="dict-ellipsoid-low_semimajor_axis",
        ),
    ),
)
def test_ellipsoid_like(ellipsoid_like, error_handler):
    cell_ids = np.arange(12)
    depth = 0

    with error_handler:
        healpix_geo.nested.healpix_to_lonlat(cell_ids, depth, ellipsoid=ellipsoid_like)


class TestGeographicCartesian:
    def test_geographic_to_cartesian(self):
        lon = np.array([5.625, 50.625, 28.125], dtype="float64")
        lat = np.array([41.93785391, 19.55202227, 19.55202227], dtype="float64")

        x, y, z = healpix_geo.lonlat_to_cartesian(lon, lat, ellipsoid="WGS84")

        expected_x = np.array([4728734.69011096, 3814362.85063174, 5302653.40426395])
        expected_y = np.array([465739.71573273, 4647814.58136658, 2834327.29466645])
        expected_z = np.array([4240471.60205904, 2121029.89621885, 2121029.89621885])

        np.testing.assert_allclose(x, expected_x)
        np.testing.assert_allclose(y, expected_y)
        np.testing.assert_allclose(z, expected_z)

    def test_cartesian_to_geographic(self):
        x = np.array([4728734.69011096, 3814362.85063174, 5302653.40426395])
        y = np.array([465739.71573273, 4647814.58136658, 2834327.29466645])
        z = np.array([4240471.60205904, 2121029.89621885, 2121029.89621885])

        lon, lat = healpix_geo.cartesian_to_lonlat(x, y, z, ellipsoid="WGS84")

        expected_lon = np.array([5.625, 50.625, 28.125], dtype="float64")
        expected_lat = np.array(
            [41.93785391, 19.55202227, 19.55202227], dtype="float64"
        )

        np.testing.assert_allclose(lon, expected_lon)
        np.testing.assert_allclose(lat, expected_lat)


class TestHealpixToGeographic:
    @pytest.mark.parametrize(
        ["cell_ids", "depth", "indexing_scheme"],
        (
            pytest.param(np.array([0, 4, 5, 7, 9]), 0, "ring", id="level0-ring"),
            pytest.param(np.array([1, 2, 3, 8]), 0, "nested", id="level0-nested"),
            pytest.param(
                np.array(
                    [
                        864691128455135232,
                        1441151880758558720,
                        2017612633061982208,
                        4899916394579099648,
                    ],
                    dtype="uint64",
                ),
                0,
                "zuniq",
                id="level0-zuniq",
            ),
            pytest.param(
                np.array([3, 19, 54, 63, 104, 127]), 4, "ring", id="level4-ring"
            ),
            pytest.param(
                np.array([22, 89, 134, 154, 190]), 4, "nested", id="level4-nested"
            ),
        ),
    )
    def test_spherical(self, cell_ids, depth, indexing_scheme):
        if indexing_scheme == "ring":
            param_cds = 2**depth
            hg_healpix_to_lonlat = healpix_geo.ring.healpix_to_lonlat
            cds_healpix_to_lonlat = cdshealpix.ring.healpix_to_lonlat
        elif indexing_scheme == "zuniq":

            def hg_healpix_to_lonlat(cell_ids, depth, ellipsoid):
                return healpix_geo.zuniq.healpix_to_lonlat(
                    cell_ids, ellipsoid=ellipsoid
                )

            def cds_healpix_to_lonlat(cell_ids, depth):
                cell_ids, depths = healpix_geo.zuniq.to_nested(cell_ids)

                return cdshealpix.nested.healpix_to_lonlat(cell_ids, depths)

            param_cds = depth
        else:
            param_cds = depth
            hg_healpix_to_lonlat = healpix_geo.nested.healpix_to_lonlat
            cds_healpix_to_lonlat = cdshealpix.nested.healpix_to_lonlat

        actual_lon, actual_lat = hg_healpix_to_lonlat(
            cell_ids, depth, ellipsoid="sphere"
        )
        expected_lon_, expected_lat_ = cds_healpix_to_lonlat(cell_ids, param_cds)
        expected_lon = np.asarray(expected_lon_.to("degree"))
        expected_lat = np.asarray(expected_lat_.to("degree"))

        np.testing.assert_allclose(actual_lon, expected_lon)
        np.testing.assert_allclose(actual_lat, expected_lat)

    @pytest.mark.parametrize(
        "ellipsoid",
        [
            "unitsphere",
            "sphere",
            "WGS84",
            pytest.param({"radius": 1}, id="unitsphere-dict"),
            pytest.param(
                {"semimajor_axis": 6378388.0, "inverse_flattening": 297.0},
                id="intl-dict",
            ),
            pytest.param(Sphere(radius=6370997.0), id="sphere-obj"),
            pytest.param(
                Ellipsoid(semimajor_axis=6378388.0, inverse_flattening=297.0),
                id="intl-obj",
            ),
        ],
    )
    @pytest.mark.parametrize("depth", [0, 1, 9])
    @pytest.mark.parametrize("indexing_scheme", ["ring", "nested", "zuniq"])
    def test_ellipsoidal(self, depth, indexing_scheme, ellipsoid):
        cell_ids = np.arange(12)
        if indexing_scheme == "ring":
            param_cds = 2**depth
            hg_healpix_to_lonlat = healpix_geo.ring.healpix_to_lonlat
            cds_healpix_to_lonlat = cdshealpix.ring.healpix_to_lonlat
        elif indexing_scheme == "zuniq":

            def hg_healpix_to_lonlat(cell_ids, depth, ellipsoid):
                return healpix_geo.zuniq.healpix_to_lonlat(
                    cell_ids, ellipsoid=ellipsoid
                )

            def cds_healpix_to_lonlat(cell_ids, depth):
                cell_ids, depths = healpix_geo.zuniq.to_nested(cell_ids)

                return cdshealpix.nested.healpix_to_lonlat(cell_ids, depths)

            cell_ids = healpix_geo.zuniq.from_nested(cell_ids, depth)
            param_cds = depth
        else:
            param_cds = depth
            hg_healpix_to_lonlat = healpix_geo.nested.healpix_to_lonlat
            cds_healpix_to_lonlat = cdshealpix.nested.healpix_to_lonlat

        actual_lon, actual_lat = hg_healpix_to_lonlat(
            cell_ids, depth, ellipsoid=ellipsoid
        )
        expected_lon_, expected_lat_ = cds_healpix_to_lonlat(cell_ids, param_cds)
        expected_lon = np.asarray(expected_lon_.to("degree"))
        expected_lat = np.asarray(expected_lat_.to("degree"))

        np.testing.assert_allclose(actual_lon, expected_lon)

        diff_lat = actual_lat - expected_lat
        assert np.all(abs(diff_lat) < 0.3)
        # check that actual is pulled away from the equator compared to the spherical latitude
        assert np.all(np.abs(actual_lat) >= np.abs(expected_lat))
        assert np.all(np.sign(actual_lat) == np.sign(expected_lat))

    @pytest.mark.parametrize(
        [
            "depth",
            "indexing_scheme",
            "ellipsoid",
            "cell_ids",
            "expected_lon",
            "expected_lat",
        ],
        (
            pytest.param(
                7,
                "nested",
                "WGS84",
                np.array([[472, 840, 1082]], dtype="uint64"),
                np.array([[51.328125, 47.109375, 55.1953125]], dtype="float64"),
                np.array([[11.77091534, 12.99776877, 13.30538509]], dtype="float64"),
                id="nested",
            ),
            pytest.param(
                3,
                "ring",
                "bessel",
                np.array([13, 43], dtype="uint64"),
                np.array([45.0, 63.0], dtype="float64"),
                np.array([72.46118472, 60.54408708], dtype="float64"),
                id="ring",
            ),
            pytest.param(
                None,
                "zuniq",
                "GRS80",
                np.array([256272108617728], dtype="uint64"),
                np.array([45.82397461], dtype="float64"),
                np.array([1.40524054], dtype="float64"),
                id="zuniq",
            ),
        ),
    )
    def test_ellipsoidal_explicit(
        self, depth, indexing_scheme, ellipsoid, cell_ids, expected_lon, expected_lat
    ):
        funcs = {
            "nested": healpix_geo.nested.healpix_to_lonlat,
            "ring": healpix_geo.ring.healpix_to_lonlat,
            "zuniq": healpix_geo.zuniq.healpix_to_lonlat,
        }
        kwargs = {
            "ipix": cell_ids,
            "ellipsoid": ellipsoid,
        }
        if indexing_scheme != "zuniq":
            kwargs["depth"] = depth
        actual_lon, actual_lat = funcs[indexing_scheme](**kwargs)

        np.testing.assert_allclose(actual_lon, expected_lon)
        np.testing.assert_allclose(actual_lat, expected_lat)


class TestGeographicToHealpix:
    @pytest.mark.parametrize(
        ["lon", "lat", "depth", "indexing_scheme"],
        (
            pytest.param(
                np.array([-170.0, 10.0, 30.0, 124.0, 174.0]),
                np.array([-48.0, -30.0, -5.0, 15.0, 30.0]),
                0,
                "ring",
                id="level0-ring",
            ),
            pytest.param(
                np.array([-170.0, 10.0, 30.0, 124.0, 174.0]),
                np.array([-48.0, -30.0, -5.0, 15.0, 30.0]),
                0,
                "nested",
                id="level0-nested",
            ),
            pytest.param(
                np.array([-170.0, 10.0, 30.0, 124.0, 174.0]),
                np.array([-48.0, -30.0, -5.0, 15.0, 30.0]),
                0,
                "zuniq",
                id="level0-zuniq",
            ),
            pytest.param(
                np.array([-70.0, 135.0, 150.0]),
                np.array([-65.0, 0.0, 65.0]),
                4,
                "ring",
                id="level4-ring",
            ),
            pytest.param(
                np.array([-70.0, 135.0, 150.0]),
                np.array([-65.0, 0.0, 65.0]),
                4,
                "nested",
                id="level4-nested",
            ),
            pytest.param(
                np.array([-70.0, 135.0, 150.0]),
                np.array([-65.0, 0.0, 65.0]),
                4,
                "zuniq",
                id="level4-zuniq",
            ),
            pytest.param(
                np.array([-10.3, 0.0]),
                np.array([45.4, 88.0]),
                5,
                "ring",
                id="level5-ring",
            ),
        ),
    )
    def test_spherical(self, lon, lat, depth, indexing_scheme):
        if indexing_scheme == "ring":
            param_cds = 2**depth
            hg_lonlat_to_healpix = healpix_geo.ring.lonlat_to_healpix
            cds_lonlat_to_healpix = cdshealpix.ring.lonlat_to_healpix
        elif indexing_scheme == "zuniq":

            def cds_lonlat_to_healpix(lon, lat, depth):
                cell_ids = cdshealpix.nested.lonlat_to_healpix(lon, lat, depth)
                return healpix_geo.zuniq.from_nested(cell_ids, depth)

            param_cds = depth
            hg_lonlat_to_healpix = healpix_geo.zuniq.lonlat_to_healpix
        else:
            param_cds = depth
            hg_lonlat_to_healpix = healpix_geo.nested.lonlat_to_healpix
            cds_lonlat_to_healpix = cdshealpix.nested.lonlat_to_healpix

        actual = hg_lonlat_to_healpix(lon, lat, depth, ellipsoid="sphere")
        lon_ = Longitude(lon, unit="degree")
        lat_ = Latitude(lat, unit="degree")
        expected = cds_lonlat_to_healpix(lon_, lat_, param_cds)

        assert np.all(np.astype(actual, "int64") != -1)

        # TODO: remove once cdshealpix-python has been upgraded
        # last broken version: 0.8.1
        mask = np.astype(expected, "int64") != -1
        np.testing.assert_equal(actual[mask], expected[mask])

    @pytest.mark.parametrize("ellipsoid", ["unitsphere", "sphere", "WGS84", "bessel"])
    @pytest.mark.parametrize("depth", [0, 1, 9])
    @pytest.mark.parametrize("indexing_scheme", ["ring", "nested", "zuniq"])
    def test_ellipsoidal(self, ellipsoid, depth, indexing_scheme):
        lat = np.linspace(-90, 90, 50)
        lon = np.full_like(lat, fill_value=45.0)

        if indexing_scheme == "ring":
            param_cds = 2**depth
            hg = healpix_geo.ring.lonlat_to_healpix
            cds = cdshealpix.ring.lonlat_to_healpix
        elif indexing_scheme == "zuniq":

            def cds(lon, lat, depth):
                cell_ids = cdshealpix.nested.lonlat_to_healpix(lon, lat, depth)
                return healpix_geo.zuniq.from_nested(cell_ids, depth)

            param_cds = depth
            hg = healpix_geo.zuniq.lonlat_to_healpix
        else:
            param_cds = depth
            hg = healpix_geo.nested.lonlat_to_healpix
            cds = cdshealpix.nested.lonlat_to_healpix

        actual = hg(lon, lat, depth, ellipsoid=ellipsoid)

        lon_ = Longitude(lon, unit="degree")
        lat_ = Latitude(lat, unit="degree")
        expected = cds(lon_, lat_, param_cds)

        assert actual.dtype == "uint64"
        assert expected.dtype == "uint64"

        # TODO: this is currently a smoke check, try more thorough checks

    @pytest.mark.parametrize(
        ["depth", "indexing_scheme", "ellipsoid", "lon", "lat", "expected"],
        (
            pytest.param(
                7,
                "nested",
                "WGS84",
                np.array([[51.328125, 47.109375, 55.1953125]], dtype="float64"),
                np.array([[11.77091534, 12.99776877, 13.30538509]], dtype="float64"),
                np.array([[472, 840, 1082]], dtype="uint64"),
                id="nested",
            ),
            pytest.param(
                3,
                "ring",
                "bessel",
                np.array([45.0, 63.0], dtype="float64"),
                np.array([72.46118472, 60.54408708], dtype="float64"),
                np.array([13, 43], dtype="uint64"),
                id="ring",
            ),
            pytest.param(
                12,
                "zuniq",
                "GRS80",
                np.array([45.82397461], dtype="float64"),
                np.array([1.40524054], dtype="float64"),
                np.array([256272108617728], dtype="uint64"),
                id="zuniq",
            ),
        ),
    )
    def test_ellipsoidal_explicit(
        self, depth, indexing_scheme, ellipsoid, lon, lat, expected
    ):
        funcs = {
            "nested": healpix_geo.nested.lonlat_to_healpix,
            "ring": healpix_geo.ring.lonlat_to_healpix,
            "zuniq": healpix_geo.zuniq.lonlat_to_healpix,
        }
        actual = funcs[indexing_scheme](
            depth=depth, longitude=lon, latitude=lat, ellipsoid=ellipsoid
        )

        np.testing.assert_equal(actual, expected)


class TestHealpixToCartesian:
    @pytest.mark.parametrize(
        [
            "ipix",
            "depth",
            "scheme",
            "ellipsoid",
            "expected_x",
            "expected_y",
            "expected_z",
        ],
        (
            pytest.param(
                np.array([23, 42, 61], dtype="uint64"),
                3,
                "nested",
                "WGS84",
                np.array(
                    [476237.29439881, 4728734.69011096, 495094.69891854],
                    dtype="float64",
                ),
                np.array(
                    [4226722.89212488, 465739.71573273, 1195264.33678817],
                    dtype="float64",
                ),
                np.array(
                    [4736816.04690125, 4240471.60205904, 6224606.75696243],
                    dtype="float64",
                ),
                id="nested",
            ),
            pytest.param(
                np.array([90, 112, 5], dtype="uint64"),
                3,
                "ring",
                "sphere",
                np.array(
                    [475999.90684385, 4725794.69249386, 495071.15870441],
                    dtype="float64",
                ),
                np.array(
                    [4224616.019301899, 465450.15124155843, 1195207.5056839434],
                    dtype="float64",
                ),
                np.array(
                    [4745065.473958334, 4247331.333333333, 6238267.895833333],
                    dtype="float64",
                ),
                id="ring",
            ),
            pytest.param(
                np.array([23, 42, 61], dtype="uint64"),
                3,
                "zuniq",
                "WGS84",
                np.array(
                    [476237.29439881, 4728734.69011096, 495094.69891854],
                    dtype="float64",
                ),
                np.array(
                    [4226722.89212488, 465739.71573273, 1195264.33678817],
                    dtype="float64",
                ),
                np.array(
                    [4736816.04690125, 4240471.60205904, 6224606.75696243],
                    dtype="float64",
                ),
                id="zuniq",
            ),
        ),
    )
    def test_conversion(
        self, ipix, depth, scheme, ellipsoid, expected_x, expected_y, expected_z
    ):
        ns = getattr(healpix_geo, scheme)
        params = {"ellipsoid": ellipsoid}
        if scheme == "zuniq":
            ipix = ns.from_nested(ipix, depth)
        else:
            params["depth"] = depth

        actual_x, actual_y, actual_z = ns.healpix_to_cartesian(ipix, **params)

        np.testing.assert_allclose(actual_x, expected_x)
        np.testing.assert_allclose(actual_y, expected_y)
        np.testing.assert_allclose(actual_z, expected_z)


class TestVertices:
    @pytest.mark.parametrize("step", [1, 3])
    @pytest.mark.parametrize(
        ["cell_ids", "depth", "indexing_scheme"],
        (
            pytest.param(np.array([0, 4, 5, 7, 9]), 0, "ring", id="level0-ring"),
            pytest.param(np.array([1, 2, 3, 8]), 0, "nested", id="level0-nested"),
            pytest.param(
                np.array(
                    [
                        864691128455135232,
                        1441151880758558720,
                        2017612633061982208,
                        4899916394579099648,
                    ],
                    dtype="uint64",
                ),
                None,
                "zuniq",
                id="level0-zuniq",
            ),
            pytest.param(np.array([0, 4, 5, 7, 9]), 0, "ring", id="level0-ring"),
            pytest.param(np.array([1, 2, 3, 8]), 0, "nested", id="level0-nested"),
            pytest.param(
                np.array([3, 19, 54, 63, 104, 127]), 4, "ring", id="level4-ring"
            ),
            pytest.param(
                np.array([22, 89, 134, 154, 190]), 4, "nested", id="level4-nested"
            ),
            pytest.param(
                np.array(
                    [
                        50665495807918080,
                        201536083324829696,
                        302867074940665856,
                        347903071214370816,
                        428967864507039744,
                    ]
                ),
                4,
                "zuniq",
                id="level4-zuniq",
            ),
        ),
    )
    def test_spherical(self, cell_ids, depth, indexing_scheme, step):
        if indexing_scheme == "ring":
            param_cds = 2**depth
            hg_vertices = healpix_geo.ring.vertices
            cds_vertices = cdshealpix.ring.vertices
        elif indexing_scheme == "zuniq":

            def cds_vertices(cell_ids, depth, step):
                cell_ids, depths = healpix_geo.zuniq.to_nested(cell_ids)

                return cdshealpix.nested.vertices(cell_ids, depths, step=step)

            def hg_vertices(cell_ids, depth, ellipsoid, step):
                return healpix_geo.zuniq.vertices(
                    cell_ids, ellipsoid=ellipsoid, step=step
                )

            param_cds = depth
        else:
            param_cds = depth
            hg_vertices = healpix_geo.nested.vertices
            cds_vertices = cdshealpix.nested.vertices

        actual_lon, actual_lat = hg_vertices(
            cell_ids, depth, ellipsoid="sphere", step=step
        )
        expected_lon_, expected_lat_ = cds_vertices(cell_ids, param_cds, step=step)
        expected_lon = np.asarray(expected_lon_.to("degree"))
        expected_lat = np.asarray(expected_lat_.to("degree"))

        np.testing.assert_allclose(actual_lon, expected_lon)
        np.testing.assert_allclose(actual_lat, expected_lat)
