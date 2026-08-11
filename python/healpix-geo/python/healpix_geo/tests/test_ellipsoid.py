import pytest

from healpix_geo import ellipsoid
from healpix_geo.typing import EllipsoidLike


@pytest.mark.parametrize(
    ["name", "expected"],
    (
        pytest.param(
            "WGS84",
            {
                "name": "WGS84",
                "semimajor_axis": 6378137.0,
                "inverse_flattening": 298.257223563,
            },
            id="WGS84",
        ),
        pytest.param(
            "unitsphere", {"name": "unitsphere", "radius": 1.0}, id="unitsphere"
        ),
    ),
)
def test_resolve(name: str, expected: EllipsoidLike) -> None:
    actual = ellipsoid.resolve(name)

    assert actual == expected
